mod core;
mod feat;
mod process;
mod schema;
mod store;
mod utils;
mod service;
mod module;

use std::sync::{Arc, Mutex, Once};

use tauri::{AppHandle,Manager};
#[cfg(desktop)]
use tauri_plugin_notification::NotificationExt;
#[cfg(desktop)]
use tauri_plugin_single_instance;

// use utils::logging::Type;

use schema::state::AppState;
use store::db::Database;
use utils::resolve;
use core::handle::Handle;

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
            let mut app_handle = self.inner.lock().unwrap();
            *app_handle = Some(handle);
        });
    }

    /// Get the app handle if it has been initialized.
    pub fn get(&self) -> Option<AppHandle> {
        self.inner.lock().unwrap().clone()
    }

    /// Get the app handle, panics if it hasn't been initialized.
    pub fn get_handle(&self) -> AppHandle {
        self.get().expect("AppHandle not initialized")
    }

    pub fn set_activation_policy_regular(&self) {
        #[cfg(target_os = "macos")]
        {
            let app_handle = self.inner.lock().unwrap();
            let app_handle = app_handle.as_ref().unwrap();
            let _ = app_handle.set_activation_policy(tauri::ActivationPolicy::Regular);
        }
    }

    pub fn set_activation_policy_accessory(&self) {
        #[cfg(target_os = "macos")]
        {
            let app_handle = self.inner.lock().unwrap();
            let app_handle = app_handle.as_ref().unwrap();
            let _ = app_handle.set_activation_policy(tauri::ActivationPolicy::Accessory);
        }
    }

    pub fn set_activation_policy_prohibited(&self) {
        #[cfg(target_os = "macos")]
        {
            let app_handle = self.inner.lock().unwrap();
            let app_handle = app_handle.as_ref().unwrap();
            let _ = app_handle.set_activation_policy(tauri::ActivationPolicy::Prohibited);
        }
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    #[cfg(desktop)]
    let db = Database::new(".".into()).expect("Failed to initialize database");
    let app_state = AppState { 
        db: Arc::new(db),
        lightweight: Arc::new(Mutex::new(LightWeightState::default())),

    };
    let builder = tauri::Builder::default()
        .plugin(tauri_plugin_single_instance::init(|app, _, _| {
            let _ = app.get_webview_window("main")
                       .expect("no main window")
                       .set_focus();
        }))
        .manage(app_state)
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_single_instance::init(|app, _, _| {
            app.notification()
                .builder()
                .title("The program is already running. Please do not start it again!")
                .icon("dida")
                .show()
                .unwrap();
        }))
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            AppHandleManager::global().init(app.handle().clone());
            Handle::global().init(&app.handle());
            tauri::async_runtime::block_on(async move {
                resolve::resolve_setup(app).await;
            });
            Ok(())
        });

    #[cfg(mobile)]
    let builder = tauri::Builder::default()
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            tauri::async_runtime::block_on(async move {
                resolve::resolve_setup(app).await;
            });
            Ok(())
        });
    let app = builder
        .invoke_handler(tauri::generate_handler![
            // Window
            core::cmd::window::toggle_main_window,
            core::cmd::window::toggle_dashboard_window,
            core::cmd::window::toggle_action_window,

            // Actions
            #[cfg(target_os = "windows")]
            core::cmd::action::execute_actions,
            #[cfg(target_os = "windows")]
            core::cmd::action::execute_single_action,
            #[cfg(target_os = "windows")]
            core::cmd::action::create_action,
            #[cfg(target_os = "windows")]
            core::cmd::action::get_action,
            #[cfg(target_os = "windows")]
            core::cmd::action::delete_action,
            #[cfg(target_os = "windows")]
            core::cmd::action::update_action,
            #[cfg(target_os = "windows")]
            core::cmd::action::get_all_actions,
            #[cfg(target_os = "windows")]
            core::cmd::action::select_file,
            
            // Tasks
            core::cmd::task::create_task,
            core::cmd::task::gen_random_task_id,
            core::cmd::task::update_task,
            core::cmd::task::update_task_status,
            core::cmd::task::delete_task,
            core::cmd::task::get_task,
            core::cmd::task::get_all_tasks,
            core::cmd::task::get_tasks_by_date_range,
            core::cmd::task::get_tasks_by_status,
            core::cmd::task::get_tasks
        ])
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
            if label == "main" {
                match event {
                    tauri::WindowEvent::CloseRequested { api, .. } => {
                        // use crate::core::handle;

                        use crate::utils::logging::Type;
                        logging!(info, Type::Window, true, "窗口关闭请求");

                        #[cfg(target_os = "macos")]
                        AppHandleManager::global().set_activation_policy_accessory();
                        if core::handle::Handle::global().is_exiting() {
                            return;
                        }
                        api.prevent_close();
                        let window = core::handle::Handle::global().get_window().unwrap();
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
            }
        }
        _ => {}
    });
}
