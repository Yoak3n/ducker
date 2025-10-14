use std::time::Instant;
use once_cell::sync::OnceCell;
use tauri::App;
// use anyhow::{bail, Result};
#[cfg(desktop)]
use crate::core::tray;
use crate::{
    config::Config, core::{handle, timer}, logging, logging_error, module::lightweight::auto_lightweight_mode_init, utils::{logging::Type, window_manager}
};

pub static VERSION: OnceCell<String> = OnceCell::new();


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
        logging_error!(Type::Tray, tray::Tray::global().init());
        if let Some(app_handle) = handle::Handle::global().app_handle() {
            logging_error!(
                Type::Tray,
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
    logging_error!(Type::Tray, tray::Tray::global().update_part());
    logging_error!(Type::System, timer::Timer::global().init());
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
