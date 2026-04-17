use crate::{
    core::tray,
    logging,
    utils::logging::Type,
    schema::WindowType,
};

use crate::core::window::{manager::WindowManager, schema::{WindowState,WindowOperationResult}};







// 保持向后兼容的公共API
// pub fn toggle_main_window() -> WindowOperationResult {
//     WindowManager::global().toggle_window(WindowType::Main)
// }

pub fn hide_main_window() -> WindowOperationResult {
    WindowManager::global().close_window(WindowType::Main)
}

pub fn create_main_window() {
    let _ = WindowManager::global().create_window(WindowType::Main, None);
    WindowManager::global().update_window_state(WindowType::Main, WindowState::VisibleFocused);
}

pub fn toggle_window_by_label(label: &str) -> WindowOperationResult {
    if let Some(window_type) = WindowType::from_label(label){
        WindowManager::global().toggle_window(window_type)
    } else {
        logging!(error, Type::Window, true, "Unknown window label: {}", label);
        WindowOperationResult::Failed
    }
}

pub fn show_window_by_label(label: &str, url: Option<&str>) {
    if let Some(window_type) = WindowType::from_label(label){
        let _ = WindowManager::global().show_window(window_type, url);
        if window_type == WindowType::Main {
            tray::Tray::global().update_menu_visible(true);
        }
    } else {
        logging!(error, Type::Window, true, "Unknown window label: {}", label);
    }
}

pub fn minimize_window_by_label(label: &str) -> bool {
    if let Some(window_type) = WindowType::from_label(label){
        WindowManager::global().minimized_window(window_type)
    } else {
        logging!(error, Type::Window, true, "Unknown window label: {}", label);
        false
    }
}

pub fn close_window_by_label(label: &str) {
    if let Some(window_type) = WindowType::from_label(label){
        WindowManager::global().close_window(window_type);
        if window_type == WindowType::Main {
            tray::Tray::global().update_menu_visible(false);
        }
    } else {
        logging!(error, Type::Window, true, "Unknown window label: {}", label);
    }
}

pub fn destroy_window_by_label(label: &str) -> bool {
    if let Some(window_type) = WindowType::from_label(label){
        WindowManager::global().destroy_window(window_type)
    } else {
        logging!(error, Type::Window, true, "Unknown window label: {}", label);
        false
    }
}

/// 检查是否所有窗口都已关闭
pub fn are_all_windows_closed() -> bool {
    WindowManager::global().are_all_windows_closed()
}
