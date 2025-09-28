use super::cmd;
#[cfg(desktop)]
use tauri_plugin_notification::NotificationExt;


/// Setup plugins for the Tauri builder
pub fn setup_plugins(builder: tauri::Builder<tauri::Wry>) -> tauri::Builder<tauri::Wry> {
    #[allow(unused_mut)]
    let mut builder = builder
        .plugin(tauri_plugin_window_state::Builder::new().build())
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
        .plugin(
            tauri_plugin_autostart::Builder::new()
                .app_name("ducker")
                .build(),
        );

    builder
}

/// Setup autostart plugin
pub fn setup_autostart(app: &tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(target_os = "macos")]
    let mut auto_start_plugin_builder = tauri_plugin_autostart::Builder::new();
    #[cfg(not(target_os = "macos"))]
    let auto_start_plugin_builder = tauri_plugin_autostart::Builder::new();

    #[cfg(target_os = "macos")]
    {
        auto_start_plugin_builder = auto_start_plugin_builder
            .macos_launcher(MacosLauncher::LaunchAgent)
            .app_name(app.config().identifier.clone());
    }
    app.handle().plugin(auto_start_plugin_builder.build())?;
    Ok(())
}

pub fn generate_handlers() -> impl Fn(tauri::ipc::Invoke<tauri::Wry>) -> bool + Send + Sync + 'static
{
    tauri::generate_handler![
        // Window
        cmd::window::toggle_window,
        cmd::window::show_window,
        cmd::window::minimize_window,
        cmd::window::close_window,
        // Actions
        #[cfg(target_os = "windows")]
        cmd::action::execute_actions,
        #[cfg(target_os = "windows")]
        cmd::action::execute_single_action,
        #[cfg(target_os = "windows")]
        cmd::action::create_action,
        #[cfg(target_os = "windows")]
        cmd::action::get_action,
        #[cfg(target_os = "windows")]
        cmd::action::delete_action,
        #[cfg(target_os = "windows")]
        cmd::action::update_action,
        #[cfg(target_os = "windows")]
        cmd::action::get_all_actions,
        #[cfg(target_os = "windows")]
        cmd::action::select_file,
        // Tasks
        cmd::task::create_task,
        cmd::task::gen_random_task_id,
        cmd::task::update_task,
        cmd::task::update_task_status,
        cmd::task::delete_task,
        cmd::task::get_task,
        cmd::task::get_all_tasks,
        cmd::task::get_tasks_by_date_range,
        cmd::task::get_tasks_by_status,
        cmd::task::get_tasks,
        cmd::task::get_enabled_periodic_tasks,
        // Periodic Tasks
        cmd::task::create_periodic_task,
        cmd::task::update_periodic_task,
        cmd::task::delete_periodic_task,
        // Config
        cmd::config::save_config,
        cmd::config::get_config,
        cmd::config::update_config
    ]
}
