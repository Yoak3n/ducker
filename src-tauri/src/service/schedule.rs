use std::collections::HashMap;

use crate::{
    core::handle::Handle,
    logging,
    schema::{
        state::AppState,
        task::{TaskRecord, TaskView},
    },
    store::module::{PeriodicTaskManager, TaskManager},
    utils::logging::Type,
};
use chrono::{Datelike, Local, TimeZone};
use tauri::Manager;


fn get_valid_periodic_tasks_until_end_of_day(
    start_date: i64,
    end_date: i64,
) -> Vec<TaskRecord> {
    let app_handle = Handle::global().app_handle().unwrap();
    let state = app_handle.state::<AppState>();
    let db_guard = state.db.lock().unwrap();
    let mut i2t_map = HashMap::new();
    db_guard
        .get_enabled_periodic_tasks()
        .unwrap_or_else(|e| {
            logging!(
                warn,
                Type::Database,
                "get_enabled_periodic_tasks error: {}",
                e
            );
            Vec::new()
        })
        .iter()
        .filter(|p| {
            p.interval != 0
                && p.interval != 100
                && p.next_period.unwrap_or(0) >= start_date as u64
                && p.next_period.unwrap_or(0) <= end_date as u64
        })
        .for_each(|p| {
            i2t_map.insert(p.id.clone(), p.interval);
        });
    let periodic_tasks_ids = i2t_map.keys().cloned().collect::<Vec<_>>();
    let periodic_tasks = db_guard
        .get_tasks(&periodic_tasks_ids)
        .unwrap_or_else(|e| {
            logging!(warn, Type::Database, "get_tasks error: {}", e);
            Vec::new()
        })
        .into_iter()
        .map(|mut t| {
            t.due_to = i2t_map.get(&t.id).cloned().unwrap_or(0) as i64;
            t
        })
        .collect::<Vec<_>>();
    periodic_tasks
}

fn get_uncompleted_tasks_until_end_of_day(
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
    // 获取到今天结束为止的任务
    let task_records = get_uncompleted_tasks_until_end_of_day(start_date, end_of_day);
    let periodic_task_records = get_valid_periodic_tasks_until_end_of_day(start_date, end_of_day);
    let mut t2i_map = HashMap::new();
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
    for task in periodic_task_records.iter() {
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
