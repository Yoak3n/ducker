use crate::utils::window_manager;

#[tauri::command]
pub async fn toggle_main_window() {
    window_manager::toggle_main_window() 
}

#[tauri::command]
pub async fn toggle_dashboard_window(){
    window_manager::toggle_dashboard_window();
}

#[tauri::command]
pub async fn toggle_action_window(){
    window_manager::toggle_action_window();
}
