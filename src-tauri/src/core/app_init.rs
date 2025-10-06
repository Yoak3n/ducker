use crate::schema::ActionType;

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
                .icon("ducker")
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

pub fn generate_handlers() -> impl Fn(tauri::ipc::Invoke<tauri::Wry>) -> bool + Send + Sync + 'static
{
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
        cmd::task::get_all_startup_periodic_tasks,
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

pub async fn check_periodic_task() {
    use crate::{
        get_app_handle,logging,
        schema::action::Action,
        schema::AppState,
        service::execute::execute_plural_actions,
        store::module::{ActionManager, PeriodicTaskManager, TaskManager},
        utils::date::is_today,utils::logging::Type,
    };
    use tauri::Manager;
    let app_handle = get_app_handle!();
    let state = app_handle.state::<AppState>();
    let db_guard = state.db.lock();
    let res = db_guard.get_enabled_periodic_tasks();
    logging!(info, Type::Database,true, "获取所有启用的周期性任务");
    if let Ok(tasks) = res {
        let prepared_tasks_ids: Vec<String> = tasks
            .iter()
            .filter(|task| 
                task.interval == 0 || 
                (task.interval == 100 && task.last_period.map_or(false, |timestamp| !is_today(timestamp)))
            )
            .map(|task| task.id.clone())
            .collect();

        if !prepared_tasks_ids.is_empty() {
            if let Ok(prepared_task_records) = db_guard.get_tasks(&prepared_tasks_ids) {
                // 收集所有立即执行任务的action ID
                let prepared_action_ids: Vec<String> = prepared_task_records
                    .iter()
                    .flat_map(|task| task.actions.iter().cloned())
                    .collect();

                if !prepared_action_ids.is_empty() {
                    if let Ok(prepared_actions) = db_guard.get_actions(&prepared_action_ids) {
                        let mut collected_actions: Vec<Action> = Vec::new();
                        prepared_actions
                            .into_iter()
                            .for_each(|a|{                     
                                match a.typ{
                                    ActionType::Group => {
                                        if let Ok(aa) = db_guard.get_actions(&[a.args.clone()]){
                                            for ar in aa {
                                                collected_actions.push(ar.into());
                                            }
                                        }
                                    }
                                    _ => collected_actions.push(a.into()),
                                }
                            });
                        logging!(info, Type::Database,true, "获取所有启用的周期性任务的所有动作{:?}",prepared_action_ids);
                        
                        let _ = db_guard
                            .update_periodic_tasks_last_run(&prepared_tasks_ids)    
                            .unwrap();
                        drop(db_guard);
                        let r = execute_plural_actions(collected_actions).await;
                        if let Err(e) = r {
                            logging!(error, Type::Database,true, "执行周期性任务的所有动作失败{:?}",e);
                        } else {
                            logging!(info, Type::Database,true, "执行周期性任务的所有动作成功");
                        }
                    }
                }
            }
        }
    }
}
