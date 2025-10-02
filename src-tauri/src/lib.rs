mod config;
pub mod core;
mod feat;
mod module;
mod process;
mod schema;
mod service;
mod store;
mod utils;
use std::sync::Arc;
use parking_lot::{Mutex, Once};
use tauri::{AppHandle, Manager};

// use utils::logging::Type;

use core::{handle::Handle, app_init};
use schema::state::AppState;
use store::db::Database;
use utils::resolve;

use crate::schema::LightWeightState;
pub struct AppHandleManager {
    inner: Mutex<Option<AppHandle>>,
    init: Once,
}

impl AppHandleManager {
    /// Get the global instance of the app handle manager.
    pub fn global() -> &'static Self {
        static INSTANCE: AppHandleManager = AppHandleManager {
            inner: Mutex::new(None),
            init: Once::new(),
        };
        &INSTANCE
    }

    /// Initialize the app handle manager with an app handle.
    pub fn init(&self, handle: AppHandle) {
        self.init.call_once(|| {
            let mut app_handle = self.inner.lock();
            *app_handle = Some(handle);
        });
    }



    /// Get the app handle if it has been initialized.
    pub fn get(&self) -> Option<AppHandle> {
        self.inner.lock().clone()
    }

    /// Get the app handle, panics if it hasn't been initialized.
    pub fn get_handle(&self) -> AppHandle {
        self.get().expect("AppHandle not initialized")
    }

    pub fn set_activation_policy_regular(&self) {
        #[cfg(target_os = "macos")]
        {
            let app_handle = self.inner.lock();
            let app_handle = app_handle.as_ref().unwrap();
            let _ = app_handle.set_activation_policy(tauri::ActivationPolicy::Regular);
        }
    }

    pub fn set_activation_policy_accessory(&self) {
        #[cfg(target_os = "macos")]
        {
            let app_handle = self.inner.lock();
            let app_handle = app_handle.as_ref().unwrap();
            let _ = app_handle.set_activation_policy(tauri::ActivationPolicy::Accessory);
        }
    }

    pub fn set_activation_policy_prohibited(&self) {
        #[cfg(target_os = "macos")]
        {
            let app_handle = self.inner.lock();
            let app_handle = app_handle.as_ref().unwrap();
            let _ = app_handle.set_activation_policy(tauri::ActivationPolicy::Prohibited);
        }
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    #[cfg(desktop)]
    let app_state = AppState {
        db: Arc::new(Mutex::new(Database::none())),
        lightweight: Arc::new(Mutex::new(LightWeightState::default())),
    };
    let builder = app_init::setup_plugins(tauri::Builder::default());
    let app = builder
        .manage(app_state)
        .setup(|app| {
            let local_data_dir = app.handle().path().app_data_dir().unwrap();
            std::fs::create_dir_all(&local_data_dir).expect("Failed to create app data dir");
            let db = Database::new(local_data_dir).expect("Failed to initialize database");
            {
                let state = app.state::<AppState>();
                let mut db_guard = state.db.lock();
                *db_guard = db;
            }
            AppHandleManager::global().init(app.handle().clone());
            Handle::global().init(app.handle().clone());
            tauri::async_runtime::block_on(async move {
                resolve::resolve_setup(app).await;
                // 启动时检查周期性任务
                app_init::check_periodic_task().await;
            });

            Ok(())
        }).invoke_handler(app_init::generate_handlers())
        .build(tauri::generate_context!())
        .expect("error while running tauri application");
    app.run(|_, e| match e {
        tauri::RunEvent::Ready | tauri::RunEvent::Resumed => {
            #[cfg(target_os = "macos")]
            {
                if let Some(window) = AppHandleManager::global()
                    .get_handle()
                    .get_webview_window("main")
                {
                    let _ = window.set_title("ducker");
                }
            }
        }
        #[cfg(target_os = "macos")]
        tauri::RunEvent::Reopen {
            has_visible_windows,
            ..
        } => {
            if !has_visible_windows {
                AppHandleManager::global().set_activation_policy_regular();
            }
            AppHandleManager::global().init(app_handle.clone());
        }
        tauri::RunEvent::ExitRequested { api, code, .. } => {
            if code.is_none() {
                api.prevent_exit();
            }
        }
        tauri::RunEvent::WindowEvent { label, event, .. } => {
            // if label == "main" {
            match event {
                tauri::WindowEvent::CloseRequested { api, .. } => {
                    // use crate::core::handle;

                    use crate::utils::logging::Type;
                    logging!(info, Type::Window, "窗口关闭请求, 窗口标签: {}", label);

                    #[cfg(target_os = "macos")]
                    AppHandleManager::global().set_activation_policy_accessory();
                    if core::handle::Handle::global().is_exiting() {
                        return;
                    }
                    api.prevent_close();
                    let window = core::handle::Handle::global()
                        .get_window_by_label(&label)
                        .unwrap();
                    let _ = window.hide();
                }
                tauri::WindowEvent::Focused(true) => {
                    #[cfg(target_os = "macos")]
                    {
                        log_err!(hotkey::Hotkey::global().register("CMD+Q", "quit"));
                        log_err!(hotkey::Hotkey::global().register("CMD+W", "hide"));
                    }

                    #[cfg(not(target_os = "macos"))]
                    {
                        // log_err!(hotkey::Hotkey::global().register("Control+Q", "quit"));
                    };
                }
                tauri::WindowEvent::Focused(false) => {
                    #[cfg(target_os = "macos")]
                    {
                        log_err!(hotkey::Hotkey::global().unregister("CMD+Q"));
                        log_err!(hotkey::Hotkey::global().unregister("CMD+W"));
                    }
                }
                tauri::WindowEvent::Destroyed => {
                    #[cfg(target_os = "macos")]
                    {
                        log_err!(hotkey::Hotkey::global().unregister("CMD+Q"));
                        log_err!(hotkey::Hotkey::global().unregister("CMD+W"));
                    }
                }
                _ => {}
            }
            // }
        }
        _ => {}
    });
}
