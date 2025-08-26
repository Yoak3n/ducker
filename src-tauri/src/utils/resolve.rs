use std::time::Instant;

use once_cell::sync::OnceCell;
use parking_lot::{Mutex, RwLock};
use tauri::App;
// use anyhow::{bail, Result};
#[cfg(desktop)]
use crate::core::tray;
use crate::{
    config::Config,
    core::{handle, timer},
    logging, logging_error,
    module::lightweight::auto_lightweight_mode_init,
    utils::{logging::Type, window_manager},
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
        logging!(info, Type::Window, true, "Creating window preview");
        let config = Config::global().await.lock().unwrap();
        config.save().ok();
        if let Some(silent_start) = config.data().silent_launch{
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
