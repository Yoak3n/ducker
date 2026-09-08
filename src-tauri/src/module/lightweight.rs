use crate::{
    config::Config, 
    core::{
        handle, 
        timer::Timer, 
        tray::Tray
    }, 
    log_err, logging, 
    schema::{
        state::LightWeightState, 
        AppState
    }, 
    utils::{
        logging::Type,
        window_manager
    }
};

#[cfg(target_os = "macos")]
use crate::logging_error;
#[cfg(target_os = "macos")]
use crate::AppHandleManager;

use anyhow::Result;
use parking_lot::Mutex;
use std::sync::OnceLock;
use std::time::Duration;
use tauri::{Listener, Manager};

// TODO 考虑为每个窗口添加单独的定时任务，当一个窗口关闭一分钟时，则销毁它

/// 10 分钟倒计时任务句柄（tokio sleep 实现，取代 delay_timer 时间轮 ——
/// 时间轮对「一次性 + 随时取消」的任务存在整点迟到的进位 bug，见 core/timer.rs 顶部说明）
static LIGHT_WEIGHT_TIMER: OnceLock<Mutex<Option<tauri::async_runtime::JoinHandle<()>>>> =
    OnceLock::new();

fn light_weight_timer_handle() -> &'static Mutex<Option<tauri::async_runtime::JoinHandle<()>>> {
    LIGHT_WEIGHT_TIMER.get_or_init(|| Mutex::new(None))
}

// 添加退出轻量模式的锁，防止并发调用
// static EXITING_LIGHTWEIGHT: AtomicBool = AtomicBool::new(false);

fn with_lightweight_status<F, R>(f: F) -> Option<R>
where
    F: FnOnce(&mut LightWeightState) -> R,
{
    if let Some(app_handle) = handle::Handle::global().app_handle() {
        // Try to get state, but don't panic if it's not managed yet
        if let Some(state) = app_handle.try_state::<AppState>() {
            let mut guard = state.lightweight.lock();
            Some(f(&mut guard))
        } else {
            // State not managed yet, return None
            None
        }
    } else {
        // App handle not available yet
        None
    }
}


pub fn auto_lightweight_mode_init() {
    if let Some(app_handle) = handle::Handle::global().app_handle() {
        // Check if state is available before accessing it
        if app_handle.try_state::<AppState>().is_none() {
            logging!(
                warn,
                Type::Lightweight,
                true,
                "AppState 尚未初始化，跳过自动轻量模式初始化"
            );
            return;
        }

        let is_silent_start = Config::global().lock().silent_launch.unwrap_or(false);
        // let enable_auto = true;

        if is_silent_start {
            logging!(
                info,
                Type::Lightweight,
                "非静默启动直接挂载自动进入轻量模式监听器！"
            );

            // 确保托盘状态更新
            if let Err(e) = Tray::global().update_part() {
                log::warn!("Failed to update tray: {e}");
            }
        }
        enable_auto_light_weight_mode();
    }
}


pub fn enable_auto_light_weight_mode() {
    Timer::global().init().unwrap();
    logging!(info, Type::Lightweight, "开启自动轻量模式");

    // 设置监听器
    setup_window_close_listener();
}


pub fn entry_lightweight_mode() {
    let result = window_manager::hide_main_window();
    logging!(
        info,
        Type::Lightweight,
        true,
        "轻量模式隐藏窗口结果: {:?}",
        result
    );
    // 销毁所有窗口
    if let Some(app_handle) = handle::Handle::global().app_handle() {
        // 获取所有窗口类型并销毁它们
        for window_type in &crate::schema::WindowType::all() {
            let label = window_type.label();
            if let Some(webview) = app_handle.get_webview_window(label) {
                logging!(info, Type::Lightweight, true, "销毁窗口: {}", label);
                let _ = webview.destroy();
            }
            window_manager::destroy_window_by_label(label);
        }

        #[cfg(target_os = "macos")]
        AppHandleManager::global().set_activation_policy_accessory();
    }
    let _ = cancel_light_weight_timer();

    // 更新托盘显示
    let _tray = crate::core::tray::Tray::global();
}


#[cfg(target_os = "macos")]
pub fn add_light_weight_timer() {
    logging_error!(Type::Lightweight, setup_light_weight_timer());
}

/// 设置窗口关闭监听器（初始化时调用）
pub fn setup_window_close_listener() {
    let window_labels = ["main"];

    // 使用动态监听机制为所有已存在的窗口添加监听器
    for window_label in &window_labels {
        add_window_listeners(window_label);
    }

    logging!(info, Type::Lightweight, true, "完成初始窗口监听器设置");
}

/// 为单个窗口添加监听器（动态添加）
pub fn add_window_listeners(window_label: &str) {
    if let Some(app_handle) = handle::Handle::global().app_handle() {
        if let Some(window) = app_handle.get_webview_window(window_label) {
            with_lightweight_status(|state| {
                let window_id = window.label();

                // 检查是否已经监听过这个窗口
                if state.listened_windows.contains(window_id) {
                    logging!(
                        debug,
                        Type::Lightweight,
                        true,
                        "窗口 {} 已经被监听，跳过重复添加",
                        window_id
                    );
                    return;
                }

                // 添加关闭监听器
                let close_handler = window.listen("tauri://close-requested", move |_event| {
                    // 检查是否所有窗口都已关闭
                    if window_manager::are_all_windows_closed() {
                        let _ = setup_light_weight_timer();
                        logging!(
                            info,
                            Type::Lightweight,
                            true,
                            "所有窗口已关闭，开始轻量模式计时"
                        );
                    } else {
                        logging!(
                            info,
                            Type::Lightweight,
                            "窗口关闭，但仍有其他窗口打开，不启动轻量模式"
                        );
                    }
                });
                state.close_listeners.push(close_handler);

                // 添加焦点监听器
                let focus_handler = window.listen("tauri://focus", move |_event| {
                    // 任何窗口获得焦点都取消轻量模式计时
                    log_err!(cancel_light_weight_timer());
                    logging!(
                        info,
                        Type::Lightweight,
                        "监听到窗口获得焦点，取消轻量模式计时"
                    );
                });
                state.focus_listeners.push(focus_handler);

                // 记录已监听的窗口
                state.listened_windows.insert(window_id.to_string());

                logging!(
                    info,
                    Type::Lightweight,
                    true,
                    "为窗口 {} 添加了监听器",
                    window_id
                );
            });
        } else {
            logging!(warn, Type::Lightweight, true, "未找到窗口 {}", window_label);
        }
    }
}

fn setup_light_weight_timer() -> Result<()> {
    Timer::global().init()?;

    // 取消可能残留的旧倒计时，再重新开始
    cancel_light_weight_timer().ok();

    let handle = tauri::async_runtime::spawn(async move {
        tokio::time::sleep(Duration::from_secs(10 * 60)).await;
        logging!(info, Type::Timer, true, "计时器到期，开始进入轻量模式");
        entry_lightweight_mode();
    });
    *light_weight_timer_handle().lock() = Some(handle);

    logging!(
        info,
        Type::Timer,
        "计时器已设置，10 分钟后将自动进入轻量模式"
    );

    Ok(())
}

fn cancel_light_weight_timer() -> Result<()> {
    if let Some(handle) = light_weight_timer_handle().lock().take() {
        handle.abort();
    }
    logging!(info, Type::Timer, "计时器已取消");
    Ok(())
}
