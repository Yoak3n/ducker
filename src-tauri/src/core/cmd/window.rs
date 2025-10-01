use crate::utils::window_manager;

#[tauri::command]
pub async fn toggle_window(window_type: String) {
    window_manager::toggle_window_by_label(&window_type);
}

#[tauri::command]
pub async fn show_window(window_type: String,url: Option<String>) {
    window_manager::show_window_by_label(&window_type,url.as_deref());
}

#[tauri::command]
pub async fn close_window(window_type: String) {
    window_manager::close_window_by_label(&window_type);
}

#[tauri::command]
pub async fn minimize_window(window_type: String) -> bool {
    window_manager::minimize_window_by_label(&window_type)
}

// 主要是用来关闭浮动窗口，避免窗口被缓存
#[tauri::command]
pub async fn destroy_window(window_type: String) -> bool {
    window_manager::destroy_window_by_label(&window_type)
}
