use super::cmd;
#[cfg(desktop)]
use tauri_plugin_notification::NotificationExt;
use chrono::{Local, TimeZone, Utc};


/// Setup plugins for the Tauri builder
pub fn setup_plugins(builder: tauri::Builder<tauri::Wry>) -> tauri::Builder<tauri::Wry> {
    #[allow(unused_mut)]
    let mut builder = builder
        // .plugin(tauri_plugin_window_state::Builder::new().build())
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

// TODO Setup autostart plugin 修复更新后开机自启失效的问题
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

pub fn generate_handlers() -> impl Fn(tauri::ipc::Invoke<tauri::Wry>) -> bool + Send + Sync + 'static{
    tauri::generate_handler![
        // Window
        cmd::window::toggle_window,
        cmd::window::show_window,
        cmd::window::minimize_window,
        cmd::window::close_window,
        cmd::window::destroy_window,
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
        // TODO 暂时不开放获取周期任务的接口
        // cmd::task::get_today_tasks,
        cmd::task::get_weekly_tasks,
        cmd::task::get_monthly_tasks,
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

/// 判断给定的时间戳是否为今天
fn is_today(timestamp: u64) -> bool {
    let today = Local::now().date_naive();
    let task_date = match Utc.timestamp_opt(timestamp as i64, 0) {
        chrono::LocalResult::Single(dt) => dt.with_timezone(&Local).date_naive(),
        _ => return false,
    };
    today == task_date
}

pub fn check_periodic_task(){
    use tauri::Manager;
    use crate::{
        core::handle::Handle,
        schema::{
            action::Action,
            state::AppState
        },
        service::execute::execute_plural_actions,
        store::module::PeriodicTaskManager,
    };
    let app_handle = Handle::global().app_handle().unwrap();
    let state = app_handle.state::<AppState>();
    let db_guard = state.db.lock().unwrap();
    let res = db_guard.get_enabled_periodic_tasks();
    if let Ok(tasks) = res {
        use crate::store::module::{TaskManager, ActionManager};
        
        let onstart_task_ids: Vec<String> = tasks
            .iter()
            .filter(|task| task.interval == 0)
            .map(|task| task.id.clone())
            .collect();
            
        // 过滤 oncestarted 任务：排除今天已经运行过的任务
        let oncestarted_task_ids: Vec<String> = tasks
            .iter()
            .filter(|task| {
                task.interval == 100 && 
                task.last_period.map_or(true, |timestamp| !is_today(timestamp))
            })
            .map(|task| task.id.clone())
            .collect();
        
        if !onstart_task_ids.is_empty() {
            if let Ok(onstart_task_records) = db_guard.get_tasks(&onstart_task_ids) {
                // 收集所有立即执行任务的action ID
                let onstart_action_ids: Vec<String> = onstart_task_records
                    .iter()
                    .flat_map(|task| task.actions.iter().cloned())
                    .collect();
                
                if !onstart_action_ids.is_empty() {
                    if let Ok(onstart_actions) = db_guard.get_actions(&onstart_action_ids) {
                        let onstart_actions: Vec<Action> = onstart_actions
                            .into_iter()
                            .map(|action| action.into())
                            .collect();
                            let _ = execute_plural_actions(onstart_actions);
                            let _ = db_guard.update_periodic_tasks_last_run(&onstart_task_ids).unwrap();
                    
                    }
                }
            }
        }

        if !oncestarted_task_ids.is_empty() {
            if let Ok(oncestarted_task_records) = db_guard.get_tasks(&oncestarted_task_ids) {
                // 收集所有周期性任务的action ID
                let oncestarted_action_ids: Vec<String> = oncestarted_task_records
                    .iter()
                    .flat_map(|task| task.actions.iter().cloned())
                    .collect();
                
                if !oncestarted_action_ids.is_empty() {
                    if let Ok(oncestarted_actions) = db_guard.get_actions(&oncestarted_action_ids) {
                        let oncestarted_actions: Vec<Action> = oncestarted_actions
                            .into_iter()
                            .map(|action| action.into())
                            .collect();
                            let _ = execute_plural_actions(oncestarted_actions);
                            let _ = db_guard.update_periodic_tasks_last_run(&oncestarted_task_ids).unwrap();
                    }
                }
            }
        }
    }
    
}