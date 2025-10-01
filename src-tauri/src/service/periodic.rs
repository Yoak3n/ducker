use std::collections::HashMap;

use chrono::{Datelike, Duration, Local, TimeZone};
use tauri::Manager;

use crate::{
    core::handle::Handle, logging, schema::{
        AppState, PeriodicTaskRecord, TaskRecord, TaskView
    }, store::module::{
        PeriodicTaskManager, TaskManager
    }, utils::{date::{self, calculate_next_period, calculate_next_period_from_now}, logging::Type}
};

/// 计算重复任务在指定时间范围内的所有执行时间，主要是计算周循环任务和月度任务
/// 
/// # 参数
/// - `periodic_task`: 周期任务记录
/// - `start_timestamp`: 开始时间戳（秒）
/// - `end_timestamp`: 结束时间戳（秒）
/// 
/// # 返回
/// 返回在时间范围内的所有执行时间戳列表
fn calculate_periodic_task_occurrences(
    periodic_task: &PeriodicTaskRecord,
    start_timestamp: i64,
    end_timestamp: i64,
) -> Vec<i64> {
    let mut occurrences = Vec::new();
    
    // 跳过特殊类型的任务
    match periodic_task.interval {
        0 | 100 => return occurrences, // OnStart 和 OnceStarted - 不在时间范围计算中
        _ => {}
    };
    
    // 确定起始计算点
    let calculation_start = if let Some(last_period) = periodic_task.last_period {
        // 如果有上次执行时间，从下次应该执行的时间开始
        if let Some(next_period) = periodic_task.next_period {
            next_period as i64
        } else {
            // 如果没有 next_period，根据间隔类型计算下次执行时间
            match periodic_task.interval {
                30 => calculate_next_period(last_period.try_into().unwrap(), 30),
                _ => {
                    // 其他间隔：使用天数计算
                    let interval_days = periodic_task.interval as i64;
                    last_period as i64 + interval_days * 24 * 3600
                }
            }
        }
    } else {
        // 如果没有执行历史，从查询开始时间开始
        start_timestamp
    };
    
    if calculation_start > start_timestamp && calculation_start <= end_timestamp {
        occurrences.push(calculation_start);
    }

    // 根据任务类型生成执行时间
    match periodic_task.interval {
        30 => {
            // 月度任务：使用日期计算，确保每月相同日期执行
            let next_month_period= calculate_next_period_from_now(calculation_start, 30);
            let start_dt = Local.timestamp_opt(next_month_period, 0).single().unwrap();
            let mut current_dt = start_dt;
            
            // 生成范围内的所有月度执行时间
            while current_dt.timestamp() <= end_timestamp {
                // 添加当前时间到结果中
                occurrences.push(current_dt.timestamp());
                current_dt = date::next_month(current_dt);
            }
        }
        _ => {
            // 其他间隔类型：使用天数计算
            let interval_days = periodic_task.interval as i64;
            let mut current_time = calculation_start;
            
            // 如果计算起始点早于查询范围，调整到查询范围内
            if current_time < start_timestamp {
                let days_diff = (start_timestamp - current_time) / (24 * 3600);
                let periods_to_skip = (days_diff / interval_days) + 1;
                current_time += periods_to_skip * interval_days * 24 * 3600;
            }
            
            // 生成范围内的所有执行时间
            while current_time <= end_timestamp {
                occurrences.push(current_time);
                current_time += interval_days * 24 * 3600;
            }
        }
    }
    
    occurrences
}

/// 获取指定时间范围内的重复任务及其执行时间
/// 
/// # 参数
/// - `start_timestamp`: 开始时间戳（秒）
/// - `end_timestamp`: 结束时间戳（秒）
/// 
/// # 返回
/// 返回 (任务记录, 执行时间戳) 的元组列表
fn get_periodic_tasks_with_occurrences(
    start_timestamp: i64,
    end_timestamp: i64,
) -> Vec<(TaskRecord, i64)> {
    let app_handle = Handle::global().app_handle().unwrap();
    let state = app_handle.state::<AppState>();
    let db_guard = state.db.lock().unwrap();
    
    let periodic_tasks = db_guard
        .get_enabled_periodic_tasks()
        .unwrap_or_else(|e| {
            logging!(
                warn,
                Type::Database,
                "get_enabled_periodic_tasks error: {}",
                e
            );
            Vec::new()
        });
    
    let mut result = Vec::new();
    
    for periodic_task in periodic_tasks {
        // 跳过特殊类型的任务
        if periodic_task.interval == 0 || periodic_task.interval == 100 {
            continue;
        }
        
        // 计算该任务在时间范围内的所有执行时间
        let occurrences = calculate_periodic_task_occurrences(
            &periodic_task,
            start_timestamp,
            end_timestamp,
        );
        
        // 获取任务详情
        if let Ok(task_record) = db_guard.get_task(&periodic_task.id) {
            for occurrence_time in occurrences {
                // 创建任务副本，设置正确的执行时间
                let mut task_with_time = task_record.clone();
                task_with_time.due_to = occurrence_time;
                result.push((task_with_time, occurrence_time));
            }
        }
    }
    
    result
}

/// 通用的时间范围任务获取函数
/// 
/// # 参数
/// - `start_timestamp`: 开始时间戳（秒）
/// - `end_timestamp`: 结束时间戳（秒）
/// 
/// # 返回
/// 返回按时间戳分组的任务视图映射
fn get_tasks_by_time_range(start_timestamp: i64, end_timestamp: i64) -> HashMap<i64, Vec<TaskView>> {
    let app_handle = Handle::global().app_handle().unwrap();
    let state = app_handle.state::<AppState>();
    let db_guard = state.db.lock().unwrap();
    
    let mut result = HashMap::new();
    
    // 1. 获取普通任务
    let normal_tasks = db_guard
        .get_uncompleted_tasks_by_date_range(start_timestamp, end_timestamp)
        .unwrap_or_else(|e| {
            logging!(
                warn,
                Type::Database,
                "get_uncompleted_tasks_by_date_range error: {}",
                e
            );
            Vec::new()
        });
    
    // 2. 获取重复任务及其执行时间
    drop(db_guard); // 释放数据库锁
    let periodic_tasks_with_times = get_periodic_tasks_with_occurrences(start_timestamp, end_timestamp);
    
    // 3. 处理普通任务
    for task in normal_tasks {
        if let Ok(task_view) = TaskView::try_from((&task, state.inner())) {
            result
                .entry(task.due_to)
                .or_insert_with(Vec::new)
                .push(task_view);
        }
        
    }
    
    // 4. 处理重复任务
    for (task, execution_time) in periodic_tasks_with_times {
        if let Ok(mut task_view) = TaskView::try_from((&task, state.inner())) {
            // 设置正确的执行时间
            task_view.due_to = Some(execution_time.to_string());
            result
                .entry(execution_time)
                .or_insert_with(Vec::new)
                .push(task_view);
        }
    }
    
    result
}

/// 获取今天的所有任务（包括重复任务的准确执行时间）
// pub fn get_today_tasks() -> HashMap<i64, Vec<TaskView>> {
//     let now = Local::now();
//     let start_of_day = Local
//         .with_ymd_and_hms(now.year(), now.month(), now.day(), 0, 0, 0)
//         .unwrap()
//         .timestamp();
//     let end_of_day = Local
//         .with_ymd_and_hms(now.year(), now.month(), now.day(), 23, 59, 59)
//         .unwrap()
//         .timestamp();
    
//     get_tasks_by_time_range(start_of_day, end_of_day)
// }

/// 获取本周的所有任务（包括重复任务的准确执行时间）
pub fn get_weekly_tasks() -> HashMap<i64, Vec<TaskView>> {
    let now = Local::now();
    
    // 计算本周的开始（周一）和结束（周日）
    let weekday = now.weekday().num_days_from_monday() as i64;
    let start_of_week = now.date_naive().and_hms_opt(0, 0, 0).unwrap() - Duration::days(weekday);
    let end_of_week = start_of_week + Duration::days(6) + Duration::hours(23) + Duration::minutes(59) + Duration::seconds(59);
    
    let start_timestamp = Local.from_local_datetime(&start_of_week).unwrap().timestamp();
    let end_timestamp = Local.from_local_datetime(&end_of_week).unwrap().timestamp();
    
    get_tasks_by_time_range(start_timestamp, end_timestamp)
}

/// 获取本月的所有任务（包括重复任务的准确执行时间）
pub fn get_monthly_tasks() -> HashMap<i64, Vec<TaskView>> {
    let now = Local::now();
    
    // 计算本月的开始和结束
    let start_of_month = Local
        .with_ymd_and_hms(now.year(), now.month(), 1, 0, 0, 0)
        .unwrap()
        .timestamp();
    
    // 下个月的第一天减去一秒，得到本月的最后一秒
    let next_month = if now.month() == 12 {
        Local.with_ymd_and_hms(now.year() + 1, 1, 1, 0, 0, 0).unwrap()
    } else {
        Local.with_ymd_and_hms(now.year(), now.month() + 1, 1, 0, 0, 0).unwrap()
    };
    let end_of_month = next_month.timestamp() - 1;
    
    get_tasks_by_time_range(start_of_month, end_of_month)
}
