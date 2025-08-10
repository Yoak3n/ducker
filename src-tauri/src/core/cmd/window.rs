use crate::utils::window_manager;

#[tauri::command]
pub async fn toggle_main_window()-> String {
    match window_manager::WindowManager::toggle_main_window() {
        window_manager::WindowOperationResult::Created => "created".to_string(),
        window_manager::WindowOperationResult::Shown => "shown".to_string(),
        window_manager::WindowOperationResult::Hidden => "hidden".to_string(),
        window_manager::WindowOperationResult::Failed => "failed".to_string(),
        window_manager::WindowOperationResult::NoAction => "no action".to_string(),
    }
}

#[tauri::command]
pub async fn toggle_dashboard_window(){
    window_manager::toggle_dashboard_window();
}
