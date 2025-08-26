use crate::feat::action::execute_action;
use crate::logging;
use crate::schema::{Action, AppState};
use crate::utils::logging::Type;
use anyhow::Result;
use std::time::Duration;
use tauri::{async_runtime, AppHandle, State};
use tokio::time::timeout;

#[tauri::command]
pub async fn execute_single_action(action: Action) -> Result<String, String> {
    execute_action(action).await
}

#[tauri::command]
pub async fn execute_actions(actions: Vec<Action>) -> Result<(), String> {
    for action in actions {
        // 检查 action 类型是否需要同步执行
        let is_sync = action.wait > 0;
        if is_sync {
            // 同步执行 - 等待任务完成
            let max_retries = action.retry.unwrap_or(0); // 最大重试次数
            let mut retry_count = 0;
            let mut last_error = String::new();

            // 获取超时时间，如果未设置则使用默认值（例如30秒）
            let timeout_duration = Duration::from_secs(action.timeout.unwrap_or(30));

            while retry_count <= max_retries {
                // 使用 timeout 包装 execute_action 调用
                match timeout(timeout_duration, execute_action(action.clone())).await {
                    Ok(result) => {
                        // 任务在超时前完成
                        match result {
                            Ok(_) => {
                                logging!(info, Type::Service, true, "任务执行成功");
                                break;
                            }
                            Err(e) => {
                                logging!(error, Type::Service, true, "任务执行失败:{}", e);
                                last_error = e.to_string();

                                if max_retries <= 0 {
                                    return Err(format!("任务执行失败:{}", &last_error));
                                }
                                retry_count += 1;
                                if retry_count < max_retries {
                                    logging!(
                                        info,
                                        Type::Service,
                                        true,
                                        "任务执行失败，正在重试 ({}/{}): {}",
                                        retry_count,
                                        max_retries,
                                        &last_error
                                    );
                                    tokio::time::sleep(tokio::time::Duration::from_millis(1000))
                                        .await;
                                }
                            }
                        }
                    }
                    Err(_) => {
                        // 任务超时
                        last_error = format!("任务执行超时（{}秒）", timeout_duration.as_secs());
                        if max_retries <= 0 {
                            return Err(format!("任务执行失败:{}", &last_error));
                        }
                        retry_count += 1;
                        if retry_count < max_retries {
                            println!(
                                "任务执行超时，正在重试 ({}/{}): {}",
                                retry_count, max_retries, &last_error
                            );
                            tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;
                        }
                    }
                }
            }

            if retry_count > max_retries {
                return Err(format!("任务执行失败，已达到最大重试次数{}", &last_error));
            }

            // 使用异步等待而不是阻塞主线程
            tokio::time::sleep(tokio::time::Duration::from_millis(action.wait as u64)).await;
        } else {
            // 异步执行 - 不等待任务完成
            let action_name = action.name.clone();
            let action_clone = action.clone();
            let timeout_duration = Duration::from_secs(action.timeout.unwrap_or(30));

            async_runtime::spawn(async move {
                println!("异步执行任务: {}", &action_name);
                match timeout(timeout_duration, execute_action(action_clone)).await {
                    Ok(result) => {
                        if let Err(e) = result {
                            eprintln!("任务 {} 执行失败: {}", &action_name, e);
                        } else {
                            println!("任务 {} 执行成功", &action_name);
                        }
                    }
                    Err(_) => {
                        eprintln!(
                            "任务 {} 执行超时（{}秒）",
                            &action_name,
                            timeout_duration.as_secs()
                        );
                    }
                }
            });
        }
    }
    Ok(())
}

#[tauri::command]
pub async fn create_action(state: State<'_, AppState>, action: Action) -> Result<String, String> {
    let db = state.db.lock().unwrap();
    
    let res = db.create_action(&action);
    match res {
        Ok(data) => Ok(data.id),
        Err(e) => {
            println!("创建action失败: {:?}", e);
            Err(e.to_string())
        }
    }
}

use crate::store::module::ActionManager;
#[tauri::command]
pub async fn get_action(state: State<'_, AppState>, id: &str) -> Result<Action, String> {
    let db = state.db.lock().unwrap();
    
    let res = db.get_action(id);
    match res {
        Ok(data) => {
            let view = Action::try_from(data).unwrap();
            Ok(view)
        }
        Err(e) => {
            println!("获取action失败: {:?}", e);
            Err(e.to_string())
        }
    }
}

#[tauri::command]
pub async fn update_action(
    state: State<'_, AppState>,
    id: &str,
    action: Action,
) -> Result<Action, String> {
    let db = state.db.lock().unwrap();
    
    let res = db.update_action(id, &action);
    match res {
        Ok(data) => {
            let view = Action::try_from(data).unwrap();
            Ok(view)
        }
        Err(e) => {
            println!("更新action失败: {:?}", e);
            Err(e.to_string())
        }
    }
}

#[tauri::command]
pub async fn delete_action(state: State<'_, AppState>, id: &str) -> Result<(), String> {
    let db = state.db.lock().unwrap();
    
    let res = db.delete_action(id);
    match res {
        Ok(_) => Ok(()),
        Err(e) => Err(e.to_string()),
    }
}

use tauri_plugin_dialog::DialogExt;
#[tauri::command]
pub async fn select_file(app: AppHandle, file: bool) -> Result<String, String> {
    // let handle = app.app_handle();
    if file {
        let file_path = app.dialog().file().blocking_pick_file();
        if let Some(file) = file_path {
            return Ok(file.to_string());
        } else {
            return Err("未选择文件".to_string());
        }
    } else {
        let file_path = app.dialog().file().blocking_pick_folder();
        if let Some(file) = file_path {
            return Ok(file.to_string());
        } else {
            return Err("未选择文件夹".to_string());
        }
    }
}

#[tauri::command]
pub async fn get_all_actions(state: State<'_, AppState>) -> Result<Vec<Action>, String> {
    let db = state.db.lock().unwrap();
    
    let res = db.get_all_actions();
    match res {
        Ok(data) => {
            let views =
                Vec::from_iter(data.into_iter().map(|item| Action::try_from(item).unwrap()));
            Ok(views)
        }
        Err(e) => Err(e.to_string()),
    }
}
