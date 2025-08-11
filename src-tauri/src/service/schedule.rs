use std::collections::HashMap;

use tauri::Manager;
use chrono::{Local,TimeZone, Datelike};
use crate::{
    core::handle::Handle,
    logging,
    schema::{state::AppState, task::TaskRecord,action::Action},
    store::module::{TaskManager,ActionManager},
    utils::logging::Type,
};

fn get_tasks_by_date_range_timestamp(start_date: i64, end_date: i64) -> Vec<TaskRecord> {
    let app_handle = Handle::global().app_handle().unwrap();
    let db = app_handle.state::<AppState>().db.clone();

    let res = db.get_tasks_by_date_range(start_date, end_date);
    res.unwrap_or_else(|e| {
        logging!(
                warn,
                Type::Database,
                "get_tasks_by_date_range_timestamp error: {}",
                e
            );
        Vec::new()
    })
}

fn get_actions_info_by_ids(ids: Vec<String>) -> Vec<Action> {
    let app_handle = Handle::global().app_handle().unwrap();
    let db = app_handle.state::<AppState>().db.clone();

    let res = db.get_actions(&ids).unwrap();
    let actions = res
            .into_iter()
            .map(|record| Action::from(record))
            .collect();
    actions
}



pub fn create_scheduled_tasks()-> HashMap<i64, Vec<Action>> {
    let now = Local::now();
    let start_date = now.timestamp();
    let end_of_day = Local
        .with_ymd_and_hms(now.year(), now.month(), now.day(), 23, 59, 59)
        .unwrap().timestamp();
    let tasks = get_tasks_by_date_range_timestamp(start_date, end_of_day);
    let mut t2i_map = HashMap::new();
    let mut actions_map = HashMap::new();
    let all_ids = tasks
        .iter()
        .filter(|task| task.auto)
        .map(|task| {
            let ids = task.actions.clone();
            t2i_map.insert(task.due_to.clone(), ids.clone());
            task.actions.clone().into_iter().collect()
        })
        .collect();
    let actions = get_actions_info_by_ids(all_ids);
    // 时间戳映射actions
    let mut i2a_map = HashMap::new();
    for action in actions {
        if let Some(id) = &action.id {
            i2a_map.insert(id.clone(), action.clone());
        }
    }
    t2i_map.iter().fold((), |_, (k, v)| {
        let actions = v
            .iter()
            .map(|id| i2a_map.get(id).unwrap().clone())
            .collect::<Vec<Action>>();
        actions_map.insert(k.clone(), actions);
    });
    actions_map
}