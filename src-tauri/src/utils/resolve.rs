use once_cell::sync::OnceCell;
use tauri::{App, Manager};
// use anyhow::{bail, Result};
#[cfg(desktop)]
use crate::core::tray;
use crate::{
    core::handle, logging, logging_error, process::AsyncHandler,
    utils::{logging::Type,window_manager},

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
    let version = app.package_info().version.to_string();

    handle::Handle::global().init(app.app_handle());
    VERSION.get_or_init(|| version.clone());

    #[cfg(mobile)]
    create_window();
    // // 启动核心
    // logging!(trace, Type::Config, true, "Initial config");
    // logging_error!(Type::Config, true, Config::init_config().await);

    // if service::check_service().await.is_err() {
    //     match service::install_service().await {
    //         Ok(_) => {
    //             log::info!(target:"app", "install service susccess.");
    //             #[cfg(not(target_os = "macos"))]
    //             std::thread::sleep(std::time::Duration::from_millis(1000));
    //             #[cfg(target_os = "macos")]
    //             {
    //                 let mut service_runing = false;
    //                 for _ in 0..40 {
    //                     if service::check_service().await.is_ok() {
    //                         service_runing = true;
    //                         break;
    //                     } else {
    //                         log::warn!(target: "app", "service not runing, sleep 500ms and check again.");
    //                         std::thread::sleep(std::time::Duration::from_millis(500));
    //                     }
    //                 }
    //                 if !service_runing {
    //                     log::warn!(target: "app", "service not running, will fallback to user mode");
    //                 }
    //             }
    //         }
    //         Err(e) => {
    //             log::warn!(target: "app", "failed to install service: {e:?}, will fallback to user mode");
    //         }
    //     }
    // }

    // logging!(trace, Type::Core, "Starting CoreManager");
    // logging_error!(Type::Core, true, CoreManager::global().init().await);

    // // setup a simple http server for singleton
    // log::trace!(target: "app", "launch embed server");
    // server::embed_server();

    #[cfg(desktop)]
    {
        log::trace!(target: "app", "Initial system tray");
        logging_error!(Type::Tray, true, tray::Tray::global().init());
        logging_error!(Type::Tray, true, tray::Tray::global().create_tray_from_handle(app.app_handle()));
        // 初始化热键
        // logging!(trace, Type::System, true, "Initial hotkeys");
        // logging_error!(Type::System, true, hotkey::Hotkey::global().init());
        logging!(info, Type::Window, true, "Creating window preview");
        // if let Some(silent_start) = { Config::setup_config().data().silent_start } {
        //     if !silent_start {
        //         create_window();
        //     }
        // } else {
        window_manager::create_main_window();
        // }
    }

    // #[cfg(desktop)]
    logging_error!(Type::Tray, true, tray::Tray::global().update_part());
    // logging_error!(Type::System, true, timer::Timer::global().init());
}
