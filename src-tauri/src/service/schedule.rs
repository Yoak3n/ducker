use std::collections::HashMap;

use crate::{
    core::handle::Handle,
    logging,
    schema::{
        state::AppState,
        task::{TaskRecord, TaskView},
    },
    store::module::TaskManager,
    utils::logging::Type,
};
use chrono::{Datelike, Local, TimeZone};
use tauri::Manager;

fn get_uncompleted_tasks_by_date_range_timestamp(
    start_date: i64,
    end_date: i64,
) -> Vec<TaskRecord> {
    let app_handle = Handle::global().app_handle().unwrap();
    let state = app_handle.state::<AppState>();
    let db_guard = state.db.lock().unwrap();
    let res = db_guard.get_uncompleted_tasks_by_date_range(start_date, end_date);
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

pub fn create_scheduled_tasks() -> HashMap<i64, Vec<TaskView>> {
    let now = Local::now();
    // 获取半分钟前开始的任务，防止某些任务未执行而因刷新被删除
    let start_date = now.timestamp() - 30;
    let end_of_day = Local
        .with_ymd_and_hms(now.year(), now.month(), now.day(), 23, 59, 59)
        .unwrap()
        .timestamp();
    // 考虑一下这一秒的刷新
    if start_date == end_of_day {
        return HashMap::new();
    }
    let task_records = get_uncompleted_tasks_by_date_range_timestamp(start_date, end_of_day);
    let mut t2i_map = HashMap::new();
    // let mut actions_map = HashMap::new();
    let app_handle = Handle::global().app_handle().unwrap();
    let app_state = app_handle.state::<AppState>();
    for task in task_records.iter() {
        if !task.auto {
            continue;
        }
        let task_view = TaskView::try_from((task, app_state.inner())).unwrap();
        t2i_map
            .entry(task.due_to)
            .or_insert_with(Vec::new)
            .push(task_view);
    }
    t2i_map
}
