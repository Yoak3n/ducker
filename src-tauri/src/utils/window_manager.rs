use once_cell::sync::OnceCell;
use parking_lot::Mutex;
use tauri::Manager;
use std::{
    sync::atomic::{AtomicBool, Ordering},
    time::{Duration, Instant},
};
use crate::{core::handle,logging,utils::logging::Type};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum WindowOperationResult {
    /// 窗口已显示并获得焦点
    Shown,
    /// 窗口已隐藏
    Hidden,
    /// 创建了新窗口
    Created,
    /// 操作失败
    Failed,
    /// 无需操作
    NoAction,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum WindowState {
    /// 窗口可见且有焦点
    VisibleFocused,
    /// 窗口可见但无焦点
    VisibleUnfocused,
    /// 窗口最小化
    Minimized,
    /// 窗口隐藏
    Hidden,
    /// 窗口不存在
    NotExist,
}
// 窗口操作防抖机制
static WINDOW_OPERATION_DEBOUNCE: OnceCell<Mutex<Instant>> = OnceCell::new();
static WINDOW_OPERATION_IN_PROGRESS: AtomicBool = AtomicBool::new(false);
const WINDOW_OPERATION_DEBOUNCE_MS: u64 = 500;

fn get_window_operation_debounce() -> &'static Mutex<Instant> {
    WINDOW_OPERATION_DEBOUNCE.get_or_init(|| Mutex::new(Instant::now() - Duration::from_secs(1)))
}

#[derive(Debug)]
pub enum WindowLabel {
    Main,
    Dashboard,
    Setting,
}

pub fn switch_main_window() {
    let app_handle = handle::Handle::global().app_handle().unwrap();
    if let Some(window) = app_handle.get_webview_window("main") {
        let visible = window.is_visible().unwrap_or(false);
        if visible {
            window.close().unwrap();
        } else {
            window.show().unwrap();
            window.set_focus().unwrap();
        }
    } else {
        create_main_window();
    }
}

pub fn create_window_by_label(label: &WindowLabel) {
    match label {
        WindowLabel::Main => {
            logging!(info, Type::Window, true, "Creating main window");
            create_main_window();
        }
        WindowLabel::Dashboard => {
            logging!(info, Type::Window, true, "Creating dashboard window");
            create_dashboard_window();
        }
        WindowLabel::Setting => {
            logging!(info, Type::Window, true, "Creating setting window");
        }
    }
}

pub fn create_main_window() {
    logging!(info, Type::Window, true, "Creating window");
    let app_handle = handle::Handle::global().app_handle().unwrap();
    #[cfg(desktop)]
    {
        if let Some(window) = handle::Handle::global().get_window() {
            logging!(
                info,
                Type::Window,
                true,
                "Found existing window, attempting to restore visibility"
            );

            if window.is_minimized().unwrap_or(false) {
                logging!(
                    info,
                    Type::Window,
                    true,
                    "Window is minimized, restoring window state"
                );
                let _ = window.unminimize();
            }
            let _ = window.show();
            let _ = window.set_focus();
            return;
        }

        logging!(info, Type::Window, true, "Creating new application window");

        #[cfg(target_os = "windows")]
        let window = tauri::WebviewWindowBuilder::new(
                    &app_handle,
                    "main".to_string(),
                    tauri::WebviewUrl::App("/main".into()),
                )
                .title("dida")
                .inner_size(890.0, 700.0)
                .min_inner_size(620.0, 550.0)
                .decorations(false)
                .focused(true)
                .maximizable(true)
                .additional_browser_args("--enable-features=msWebView2EnableDraggableRegions --disable-features=OverscrollHistoryNavigation,msExperimentalScrolling")
                .transparent(true)
                .shadow(false)
                .center()
                .build();
        match window {
            Ok(w) => {
                use crate::process::AsyncHandler;
                w.set_focus().unwrap();
                logging!(info, Type::Window, true, "Window created successfully");
                // let app_handle_clone = app_handle.clone();
                AsyncHandler::spawn(move || async move {
                    // 处理启动完成
                    handle::Handle::global().mark_startup_completed();
                });
            }
            Err(e) => {
                logging!(
                    error,
                    Type::Window,
                    true,
                    "Failed to create window: {:?}",
                    e
                );
            }
        }
    }
    #[cfg(mobile)]
    {
        println!("android");
        logging!(info, Type::Window, true, "Creating new application window");
        let _ = tauri::WebviewWindowBuilder::new(
            &app_handle,
            "main".to_string(),
            tauri::WebviewUrl::App("index.html".into()),
        )
        .build();
    }
}


fn create_dashboard_window(){

}