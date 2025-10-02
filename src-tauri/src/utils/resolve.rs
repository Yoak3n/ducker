use std::time::Instant;

use once_cell::sync::OnceCell;
use parking_lot::{Mutex, RwLock};
use tauri::App;
// use anyhow::{bail, Result};
#[cfg(desktop)]
use crate::core::tray;
use crate::{
    config::Config, core::{handle, timer}, logging, logging_error, module::lightweight::auto_lightweight_mode_init, utils::{logging::Type, window_manager}
};

pub static VERSION: OnceCell<String> = OnceCell::new();

// 添加全局UI准备就绪标志
static UI_READY: OnceCell<RwLock<bool>> = OnceCell::new();

// 窗口创建锁，防止并发创建窗口
static WINDOW_CREATING: OnceCell<Mutex<(bool, Instant)>> = OnceCell::new();

// UI就绪阶段状态枚举
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum UiReadyStage {
    NotStarted,
    Loading,
    DomReady,
    ResourcesLoaded,
    Ready,
}

// UI就绪详细状态
#[derive(Debug)]
struct UiReadyState {
    stage: RwLock<UiReadyStage>,
}

impl Default for UiReadyState {
    fn default() -> Self {
        Self {
            stage: RwLock::new(UiReadyStage::NotStarted),
        }
    }
}

// 获取UI就绪状态细节
static UI_READY_STATE: OnceCell<UiReadyState> = OnceCell::new();

fn get_window_creating_lock() -> &'static Mutex<(bool, Instant)> {
    WINDOW_CREATING.get_or_init(|| Mutex::new((false, Instant::now())))
}

fn get_ui_ready() -> &'static RwLock<bool> {
    UI_READY.get_or_init(|| RwLock::new(false))
}

fn get_ui_ready_state() -> &'static UiReadyState {
    UI_READY_STATE.get_or_init(UiReadyState::default)
}

// 更新UI准备阶段
pub fn update_ui_ready_stage(stage: UiReadyStage) {
    let state = get_ui_ready_state();
    let mut stage_lock = state.stage.write();

    *stage_lock = stage;
    // 如果是最终阶段，标记UI完全就绪
    if stage == UiReadyStage::Ready {
        mark_ui_ready();
    }
}

// 标记UI已准备就绪
pub fn mark_ui_ready() {
    let mut ready = get_ui_ready().write();
    *ready = true;
    logging!(info, Type::Window, true, "UI已标记为完全就绪");
}

// 重置UI就绪状态
pub fn reset_ui_ready() {
    {
        let mut ready = get_ui_ready().write();
        *ready = false;
    }
    {
        let state = get_ui_ready_state();
        let mut stage = state.stage.write();
        *stage = UiReadyStage::NotStarted;
    }
    logging!(info, Type::Window, true, "UI就绪状态已重置");
}

/// handle something when start app
pub async fn resolve_setup(app: &mut App) {
    // error::redirect_panic_to_log();
    #[cfg(target_os = "macos")]
    {
        AppHandleManager::global().init(app.app_handle().clone());
        AppHandleManager::global().set_activation_policy_accessory();
    }
    let start_time = Instant::now();
    logging!(
        info,
        Type::Setup,
        true,
        "开始执行异步设置任务... 线程ID: {:?}",
        std::thread::current().id()
    );
    if VERSION.get().is_none() {
        let version = app.package_info().version.to_string();
        VERSION.get_or_init(|| {
            logging!(info, Type::Setup, true, "初始化版本信息: {}", version);
            version.clone()
        });
    }

    #[cfg(target_os = "windows")]
    {
        logging_error!(Type::Tray, true, tray::Tray::global().init());
        if let Some(app_handle) = handle::Handle::global().app_handle() {
            logging_error!(
                Type::Tray,
                true,
                tray::Tray::global().create_tray_from_handle(&app_handle)
            );
        } else {
            logging!(
                error,
                Type::Tray,
                true,
                "无法创建系统托盘: app_handle不存在"
            );
        }

        // 初始化热键
        // logging!(trace, Type::System, true, "Initial hotkeys");
        // logging_error!(Type::System, true, hotkey::Hotkey::global().init());
        // logging!(info, Type::Window, true, "Creating window preview");
        let config = Config::global().lock();
        config.save().ok();
        if let Some(silent_start) = config.data().silent_launch {
            if !silent_start {
                window_manager::create_main_window();
            }
        }
    }
    // #[cfg(desktop)]
    logging_error!(Type::Tray, true, tray::Tray::global().update_part());
    logging_error!(Type::System, true, timer::Timer::global().init());
    auto_lightweight_mode_init();
    let elapsed = start_time.elapsed();
    logging!(
        info,
        Type::Setup,
        true,
        "异步设置任务完成，耗时: {:?}",
        elapsed
    );

    // 如果初始化时间过长，记录警告
    if elapsed.as_secs() > 10 {
        logging!(
            warn,
            Type::Setup,
            true,
            "异步设置任务耗时较长({:?})",
            elapsed
        );
    }
}


// pub async fn create_window(is_show: bool) -> bool {
//     logging!(
//         info,
//         Type::Window,
//         true,
//         "开始创建/显示主窗口, is_show={}",
//         is_show
//     );

//     if !is_show {
//         logging!(info, Type::Window, true, "静默模式启动时不创建窗口");
//         lightweight::set_lightweight_mode(true);
//         // handle::Handle::notify_startup_completed();
//         return false;
//     }

//     if let Some(app_handle) = handle::Handle::global().app_handle() {
//         if let Some(window) = app_handle.get_webview_window("main") {
//             logging!(info, Type::Window, true, "主窗口已存在，将显示现有窗口");
//             if is_show {
//                 if window.is_minimized().unwrap_or(false) {
//                     logging!(info, Type::Window, true, "窗口已最小化，正在取消最小化");
//                     let _ = window.unminimize();
//                 }
//                 let _ = window.show();
//                 let _ = window.set_focus();

//                 #[cfg(target_os = "macos")]
//                 {
//                     AppHandleManager::global().set_activation_policy_regular();
//                 }
//             }
//             return true;
//         }
//     }

//     let creating_lock = get_window_creating_lock();
//     let mut creating = creating_lock.lock();

//     let (is_creating, last_time) = *creating;
//     let elapsed = last_time.elapsed();

//     if is_creating && elapsed < Duration::from_secs(2) {
//         logging!(
//             info,
//             Type::Window,
//             true,
//             "窗口创建请求被忽略，因为最近创建过 ({:?}ms)",
//             elapsed.as_millis()
//         );
//         return false;
//     }

//     *creating = (true, Instant::now());

//     // ScopeGuard 确保创建状态重置，防止 webview 卡死
//     let _guard = scopeguard::guard(creating, |mut creating_guard| {
//         *creating_guard = (false, Instant::now());
//         logging!(debug, Type::Window, true, "[ScopeGuard] 窗口创建状态已重置");
//     });

//     let app_handle = match handle::Handle::global().app_handle() {
//         Some(handle) => handle,
//         None => {
//             logging!(
//                 error,
//                 Type::Window,
//                 true,
//                 "无法获取app_handle，窗口创建失败"
//             );
//             return false;
//         }
//     };

//     match tauri::WebviewWindowBuilder::new(
//         &app_handle,
//         "main", /* the unique window label */
//         tauri::WebviewUrl::App("index.html".into()),
//     )
//     .title("Clash Verge")
//     .center()
//     .decorations(true)
//     .fullscreen(false)
//     .inner_size(300.0 , 480.0 )
//     .min_inner_size(520.0, 520.0)
//     .visible(true) // 立即显示窗口，避免用户等待
//     .initialization_script(
//         r#"
//         console.log('[Tauri] 窗口初始化脚本开始执行');

//         function createLoadingOverlay() {

//             if (document.getElementById('initial-loading-overlay')) {
//                 console.log('[Tauri] 加载指示器已存在');
//                 return;
//             }

//             console.log('[Tauri] 创建加载指示器');
//             const loadingDiv = document.createElement('div');
//             loadingDiv.id = 'initial-loading-overlay';
//             loadingDiv.innerHTML = `
//                 <div style="
//                     position: fixed; top: 0; left: 0; right: 0; bottom: 0;
//                     background: var(--bg-color, #f5f5f5); color: var(--text-color, #333);
//                     display: flex; flex-direction: column; align-items: center;
//                     justify-content: center; z-index: 9999;
//                     font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
//                     transition: opacity 0.3s ease;
//                 ">
//                     <div style="margin-bottom: 20px;">
//                         <div style="
//                             width: 40px; height: 40px; border: 3px solid #e3e3e3;
//                             border-top: 3px solid #3498db; border-radius: 50%;
//                             animation: spin 1s linear infinite;
//                         "></div>
//                     </div>
//                     <div style="font-size: 14px; opacity: 0.7;">Loading Clash Verge...</div>
//                 </div>
//                 <style>
//                     @keyframes spin {
//                         0% { transform: rotate(0deg); }
//                         100% { transform: rotate(360deg); }
//                     }
//                     @media (prefers-color-scheme: dark) {
//                         :root { --bg-color: #1a1a1a; --text-color: #ffffff; }
//                     }
//                 </style>
//             `;

//             if (document.body) {
//                 document.body.appendChild(loadingDiv);
//             } else {
//                 document.addEventListener('DOMContentLoaded', () => {
//                     if (document.body && !document.getElementById('initial-loading-overlay')) {
//                         document.body.appendChild(loadingDiv);
//                     }
//                 });
//             }
//         }

//         createLoadingOverlay();

//         if (document.readyState === 'loading') {
//             document.addEventListener('DOMContentLoaded', createLoadingOverlay);
//         } else {
//             createLoadingOverlay();
//         }

//         console.log('[Tauri] 窗口初始化脚本执行完成');
//     "#,
//     )
//     .build()
//     {
//         Ok(newly_created_window) => {
//             logging!(debug, Type::Window, true, "主窗口实例创建成功");

//             update_ui_ready_stage(UiReadyStage::NotStarted);

//             AsyncHandler::spawn(move || async move {
//                 handle::Handle::global().mark_startup_completed();
//                 logging!(
//                     debug,
//                     Type::Window,
//                     true,
//                     "异步窗口任务开始 (启动已标记完成)"
//                 );

//                 // 先运行轻量模式检测
//                 lightweight::run_once_auto_lightweight().await;

//                 // 发送启动完成事件，触发前端开始加载
//                 logging!(
//                     debug,
//                     Type::Window,
//                     true,
//                     "发送 verge://startup-completed 事件"
//                 );
//                 // handle::Handle::notify_startup_completed();

//                 if is_show {
//                     let window_clone = newly_created_window.clone();

//                     // 立即显示窗口
//                     let _ = window_clone.show();
//                     let _ = window_clone.set_focus();
//                     logging!(info, Type::Window, true, "窗口已立即显示");
//                     #[cfg(target_os = "macos")]
//                     {
//                         AppHandleManager::global().set_activation_policy_regular();
//                     }

//                     let timeout_seconds = if crate::module::lightweight::is_in_lightweight_mode() {
//                         3
//                     } else {
//                         8
//                     };

//                     logging!(
//                         info,
//                         Type::Window,
//                         true,
//                         "开始监控UI加载状态 (最多{}秒)...",
//                         timeout_seconds
//                     );

//                     // 异步监控UI状态，使用try_read避免死锁
//                     AsyncHandler::spawn(move || async move {
//                         logging!(
//                             debug,
//                             Type::Window,
//                             true,
//                             "启动UI状态监控线程，超时{}秒",
//                             timeout_seconds
//                         );

//                         let ui_ready_checker = || async {
//                             let (mut check_count, mut consecutive_failures) = (0, 0);

//                             loop {
//                                 let is_ready = get_ui_ready()
//                                     .try_read()
//                                     .map(|guard| *guard)
//                                     .unwrap_or_else(|| {
//                                         consecutive_failures += 1;
//                                         if consecutive_failures > 50 {
//                                             logging!(
//                                                 warn,
//                                                 Type::Window,
//                                                 true,
//                                                 "UI状态监控连续{}次无法获取读锁，可能存在死锁",
//                                                 consecutive_failures
//                                             );
//                                             consecutive_failures = 0;
//                                         }
//                                         false
//                                     });

//                                 if is_ready {
//                                     logging!(
//                                         debug,
//                                         Type::Window,
//                                         true,
//                                         "UI状态监控检测到就绪信号，退出监控"
//                                     );
//                                     return;
//                                 }

//                                 consecutive_failures = 0;
//                                 tokio::time::sleep(Duration::from_millis(100)).await;
//                                 check_count += 1;

//                                 if check_count % 20 == 0 {
//                                     logging!(
//                                         debug,
//                                         Type::Window,
//                                         true,
//                                         "UI加载状态检查... ({}秒)",
//                                         check_count / 10
//                                     );
//                                 }
//                             }
//                         };

//                         let wait_result = tokio::time::timeout(
//                             Duration::from_secs(timeout_seconds),
//                             ui_ready_checker(),
//                         )
//                         .await;

//                         match wait_result {
//                             Ok(_) => {
//                                 logging!(info, Type::Window, true, "UI已完全加载就绪");
//                                 handle::Handle::global()
//                                     .get_main_window()
//                                     .map(|window| window.eval(r"
//                                         const overlay = document.getElementById('initial-loading-overlay');
//                                         if (overlay) {
//                                             overlay.style.opacity = '0';
//                                             setTimeout(() => overlay.remove(), 300);
//                                         }
//                                     "));
//                             }
//                             Err(_) => {
//                                 logging!(
//                                     warn,
//                                     Type::Window,
//                                     true,
//                                     "UI加载监控超时({}秒)，但窗口已正常显示",
//                                     timeout_seconds
//                                 );

//                                 get_ui_ready()
//                                     .try_write()
//                                     .map(|mut guard| {
//                                         *guard = true;
//                                         logging!(
//                                             info,
//                                             Type::Window,
//                                             true,
//                                             "超时后成功设置UI就绪状态"
//                                         );
//                                     })
//                                     .unwrap_or_else(|| {
//                                         logging!(
//                                             error,
//                                             Type::Window,
//                                             true,
//                                             "超时后无法获取UI状态写锁，可能存在严重死锁"
//                                         );
//                                     });
//                             }
//                         }
//                     });

//                     logging!(info, Type::Window, true, "窗口显示流程完成");
//                 } else {
//                     logging!(
//                         debug,
//                         Type::Window,
//                         true,
//                         "is_show为false，窗口保持隐藏状态"
//                     );
//                 }
//             });
//             true
//         }
//         Err(e) => {
//             logging!(error, Type::Window, true, "主窗口构建失败: {}", e);
//             false
//         }
//     }
// }
