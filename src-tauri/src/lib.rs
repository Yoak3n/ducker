mod config;
pub mod core;
mod feat;
mod module;
mod process;
mod schema;
mod service;
mod store;
mod utils;
use parking_lot::{Mutex, Once};
use std::sync::Arc;
use tauri::{AppHandle, Manager};
use tracing::Level;

// use utils::logging::Type;

use core::{app_init, handle::Handle};
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
    let subscriber = tracing_subscriber::fmt()
        // filter spans/events with level TRACE or higher.
        .with_max_level(Level::TRACE)
        // build but do not install the subscriber.
        .finish();
    tracing::subscriber::with_default(subscriber, || {
        tracing::info!("This will be logged to stdout");
    });

    #[cfg(desktop)]
    let app_state = AppState {
        db: Arc::new(Mutex::new(Database::none())),
        lightweight: Arc::new(Mutex::new(LightWeightState::default())),
    };
    let mut builder = tauri::Builder::default().manage(app_state);
    builder = app_init::setup_plugins(builder);
    let app = builder
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
        })
        .invoke_handler(app_init::generate_handlers())
        .build(tauri::generate_context!())
        .expect("error while running tauri application");
    app.run( app_init::app_event_handle);
}
