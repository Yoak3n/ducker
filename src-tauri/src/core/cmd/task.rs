use crate::{
    logging, schema::{
        task::{TaskData, TaskRecord, TaskView},
        AppState, PeriodicTask, PeriodicTaskData,
    }, service::periodic, store::module::{
        PeriodicTaskManager, TaskManager
    }, utils::{
        help::random_string, 
        logging::Type
    }
};
use tauri::State;

#[tauri::command]
pub async fn create_task(state: State<'_, AppState>, task: TaskData) -> Result<String, String> {
    let db = state.db.lock();
    let res = db.create_task(&task);
    match res {
        Ok(data) => Ok(data.id),
        Err(e) => {
            logging!(info, Type::Database,true,"创建任务失败: {:?}", e);
            Err(e.to_string())
        }
    }
}

#[tauri::command]
pub async fn update_task(
    state: State<'_, AppState>,
    id: &str,
    task: TaskData,
) -> Result<TaskRecord, String> {
    let db = state.db.lock();

    let res = db.update_task(id, &task);
    match res {
        Ok(data) => Ok(data),
        Err(e) => {
            println!("更新任务失败: {:?}", e);
            Err(e.to_string())
        }
    }
}

#[tauri::command]
pub async fn update_task_status(
    state: State<'_, AppState>,
    id: &str,
    completed: bool,
) -> Result<bool, String> {
    let db = state.db.lock();

    let res = db.update_task_status(id, completed);
    match res {
        Ok(data) => Ok(data),
        Err(e) => {
            println!("更新任务状态失败: {:?}", e);
            Err(e.to_string())
        }
    }
}

#[tauri::command]
pub async fn delete_task(state: State<'_, AppState>, id: &str) -> Result<(), String> {
    
    let db = state.db.lock();

    let res = db.delete_task(id);
    match res {
        Ok(_) => Ok(()),
        Err(e) => {
            logging!(error, Type::Database, true, "删除任务失败: {:?}", e);
            Err(e.to_string())
        }
    }
}

#[tauri::command]
pub async fn get_task(state: State<'_, AppState>, id: &str) -> Result<TaskView, String> {
    let db = state.db.lock();

    let res = db.get_task(id);
    match res {
        Ok(data) => Ok(TaskView::try_from((&data, state.inner())).unwrap()),
        Err(e) => {
            logging!(error, Type::Database, true, "获取任务失败: {:?}", e);
            Err(e.to_string())
        }
    }
}

#[tauri::command]
pub async fn get_all_tasks(state: State<'_, AppState>) -> Result<Vec<TaskView>, String> {
    // 快速获取数据并立即释放数据库锁
    let data = {
        let db = state.db.lock();   
        db.get_all_tasks()
    };
    match data {
        Ok(task_records) => {
            let mut tasks = Vec::new();
            for task in task_records {
                tasks.push(TaskView::try_from((&task, state.inner())).unwrap());
            }
            Ok(tasks)
        }
        Err(e) => {
            Err(e.to_string())
        }
    }
}

#[tauri::command]
pub async fn get_tasks_by_date_range(
    state: State<'_, AppState>,
    start_date: i64,
    end_date: i64,
) -> Result<Vec<TaskView>, String> {
    logging!(info, Type::Database, true, "获取任务范围: {:?}", (start_date, end_date));
    
    // 快速获取数据并立即释放数据库锁
    let data = {
        let db = state.db.lock();
        db.get_tasks_by_date_range(start_date, end_date)
    };
    
    match data {
        Ok(task_records) => {
            logging!(info, Type::Database, true, "获取任务范围成功: {:?}", task_records.len());
            let mut tasks = Vec::new();
            for task in task_records {
                tasks.push(TaskView::try_from((&task, state.inner())).unwrap());
            }
            Ok(tasks)
        }
        Err(e) => {
            println!("获取任务失败: {:?}", e);
            Err(e.to_string())
        }
    }
}

#[tauri::command]
pub async fn get_tasks_by_status(
    state: State<'_, AppState>,
    completed: bool,
) -> Result<Vec<TaskRecord>, String> {
    let db = state.db.lock();

    let res = db.get_tasks_by_status(completed);
    match res {
        Ok(data) => Ok(data),
        Err(e) => {
            println!("获取任务失败: {:?}", e);
            Err(e.to_string())
        }
    }
}

#[tauri::command]
pub async fn get_tasks(
    state: State<'_, AppState>,
    ids: Vec<String>,
) -> Result<Vec<TaskView>, String> {
    // 快速获取数据并立即释放数据库锁
    let data = {
        let db = state.db.lock();
        db.get_tasks(&ids)
    };

    match data {
        Ok(task_records) => {
            let mut tasks = Vec::new();
            for task in task_records {
                tasks.push(TaskView::try_from((&task, state.inner())).unwrap());
            }
            Ok(tasks)
        }
        Err(e) => {
            println!("获取任务失败: {:?}", e);
            Err(e.to_string())
        }
    }
}

#[tauri::command]
pub async fn gen_random_task_id() -> Result<String, String> {
    let random_str = format!("task_{}", random_string(6));
    Ok(random_str)
}


#[tauri::command]
pub async fn get_enabled_periodic_tasks(state: State<'_, AppState>) -> Result<Vec<PeriodicTask>, String> {
    let records = {
        let db = state.db.lock();
        db.get_enabled_periodic_tasks()
    };

    match records {
        Ok(data) => {
            let mut periodic_tasks = Vec::new();
            for record in data {
                match PeriodicTask::try_from((&record, state.inner())) {
                    Ok(task) => periodic_tasks.push(task),
                    Err(e) => {
                        logging!(error, Type::Database, true, "转换周期性任务失败: {:?}", e);
                        return Err(e.to_string());
                    }
                }
            }
            Ok(periodic_tasks)
        }
        Err(e) => {
            logging!(error, Type::Database, true, "获取周期性任务失败: {:?}", e);
            Err(e.to_string())
        }
    }
}

#[tauri::command]
pub async fn get_all_startup_periodic_tasks(state: State<'_, AppState>) -> Result<Vec<PeriodicTask>, String> {
    let records = {
        let db = state.db.lock();
        db.get_startup_periodic_tasks()
    };

    match records {
        Ok(data) => {
            let mut startup_tasks = Vec::new();
            for record in data {
                match PeriodicTask::try_from((&record, state.inner())) {
                    Ok(task) => startup_tasks.push(task),
                    Err(e) => {
                        logging!(error, Type::Database, true, "转换启动时周期性任务失败: {:?}", e);
                        return Err(e.to_string());
                    }
                }
            }
            Ok(startup_tasks)
        }
        Err(e) => {
            logging!(error, Type::Database, true, "获取启动时周期性任务失败: {:?}", e);
            Err(e.to_string())
        }
    }
}

#[tauri::command]
pub async fn create_periodic_task(state: State<'_, AppState>,task: PeriodicTaskData) -> Result<String, String> {
    logging!(info, Type::Database, true, "创建周期性任务: {:?}", task);
    let db = state.db.lock();
    let res = db.create_periodic_task(&task);
    match res {
        Ok(data) => {
            logging!(debug, Type::Database,true, "创建周期性任务成功: {}", data.id);
            Ok(data.id)
        }
        Err(e) => {
            logging!(error, Type::Database, true, "创建周期性任务失败: {:?}", e);
            Err(e.to_string())
        }
    }
}

#[tauri::command]
pub async fn update_periodic_task(
    state: State<'_, AppState>,
    task: PeriodicTaskData,
) -> Result<PeriodicTask, String> {

    let record = {
        let db = state.db.lock();
        if let Err(e) = db.update_task(task.task.id.clone().unwrap().as_str(), &task.task) {
            logging!(error, Type::Database, true, "更新基本任务失败: {:?}", e);
            return Err(e.to_string());
        }
        db.update_periodic_task(task.task.id.clone().unwrap().as_str(), &task)
    };

    match record {
        Ok(data) => {
            logging!(info, Type::Database, "更新周期性任务成功: {}", data.id);
            // 将 PeriodicTaskRecord 转换为 PeriodicTask
            match PeriodicTask::try_from((&data, state.inner())) {
                Ok(periodic_task) => Ok(periodic_task),
                Err(e) => {
                    logging!(error, Type::Database, true, "转换周期性任务失败: {:?}", e);
                    Err(e.to_string())
                }
            }
        }
        Err(e) => {
            logging!(error, Type::Database, true, "更新周期性任务失败: {:?}", e);
            Err(e.to_string())
        }
    }
}

#[tauri::command]
pub async fn update_periodic_task_last_period(state: State<'_, AppState>, id: String)-> Result<(),String>{
    let db = state.db.lock();
    match db.update_periodic_task_last_period(id.as_str(), None) {
        Ok(_) => Ok(()),
        Err(e) => {
            logging!(error, Type::Database, true, "更新周期性任务最后运行时间失败: {:?}", e);
            Err(e.to_string())
        }
    }
}

#[tauri::command]
pub async fn delete_periodic_task(state: State<'_, AppState>, id: String) -> Result<(), String> {
    let db = state.db.lock();

    let res = db.delete_periodic_task(id.as_str());
    match res {
        Ok(_) => {
            logging!(info, Type::Database, "删除周期性任务成功: {}", id);
            Ok(())
        }
        Err(e) => {
            logging!(error, Type::Database, true, "删除周期性任务失败: {:?}", e);
            Err(e.to_string())
        }
    }
}

// #[tauri::command]
// pub async fn get_today_tasks() -> Result<std::collections::HashMap<i64, Vec<TaskView>>, String> {
//     Ok(periodic::get_today_tasks())
// }

#[tauri::command]
pub async fn get_weekly_tasks() -> Result<std::collections::HashMap<i64, Vec<TaskView>>, String> {
    Ok(periodic::get_weekly_tasks())
}

#[tauri::command]
pub async fn get_monthly_tasks() -> Result<std::collections::HashMap<i64, Vec<TaskView>>, String> {
    Ok(periodic::get_monthly_tasks())
}


