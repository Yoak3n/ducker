use crate::{
    logging, schema::{
        task::{TaskData, TaskRecord, TaskView},
        AppState,
    }, store::module::TaskManager, utils::{help::random_string, logging::Type}
};
use tauri::State;

#[tauri::command]
pub async fn create_task(state: State<'_, AppState>, task: TaskData) -> Result<String, String> {
    let db = state.db.lock().unwrap();
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
    let db = state.db.lock().unwrap();

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
    let db = state.db.lock().unwrap();

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
    let db = state.db.lock().unwrap();

    let res = db.delete_task(id);
    match res {
        Ok(_) => Ok(()),
        Err(e) => {
            println!("删除任务失败: {:?}", e);
            Err(e.to_string())
        }
    }
}

#[tauri::command]
pub async fn get_task(state: State<'_, AppState>, id: &str) -> Result<TaskView, String> {
    let db = state.db.lock().unwrap();

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
        let db = state.db.lock().unwrap();
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
        let db = state.db.lock().unwrap();
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
    let db = state.db.lock().unwrap();

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
        let db = state.db.lock().unwrap();
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
// #[tauri::command]
// pub async fn execute_task_actions(state: State<'_, AppState>, id: String) -> Result<(), String> {
//     let task = state.db.get_task(&id).map_err(|e| e.to_string())?;
//     let actions = state.db.get_actions(&task.actions).map_err(|e| e.to_string())?;

//     for action in actions {
//         execute_action(action.into()).await?;
//     }
//     Ok(())
// }
