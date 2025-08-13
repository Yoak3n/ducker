use std::time::Duration;
use tauri::async_runtime;
use anyhow::Result;

use crate::{
    schema::action::Action,
    feat::action::execute_action, 
    logging, 
    utils::logging::Type};
use tokio::time::timeout;

pub async fn execute_single_action(action: &Action){
     let is_sync = action.wait > 0;
        println!("{:?}", action);
        if is_sync {
            // 同步执行 - 等待任务完成
            println!("同步执行任务: {}", &action.name);
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
                                logging!(info, Type::Service,true, "任务执行成功");
                                break;
                            }
                            Err(e) => {
                                last_error = e.to_string();
                                if max_retries <= 0 {
                                    logging!(error, Type::Service,true, "任务执行失败:{}", &last_error);
                                }
                                retry_count += 1;
                                if retry_count < max_retries {
                                    println!(
                                        "任务执行失败，正在重试 ({}/{}): {}",
                                        retry_count, max_retries, &last_error
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
                            logging!(error, Type::Service,true, "任务执行失败:{}", &last_error);
                            return ;
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
                logging!(error, Type::Service,true, "任务执行失败，已达到最大重试次数{}", &last_error);
                return ;
            }

            // 使用异步等待而不是阻塞主线程
            tokio::time::sleep(tokio::time::Duration::from_millis(action.wait as u64)).await;
        } else {
            // 异步执行 - 不等待任务完成
            let action_name = action.name.clone();
            let action_clone = action.clone();
            let timeout_duration = Duration::from_secs(action.timeout.unwrap_or(30));

            async_runtime::spawn(async move {
                logging!(info, Type::Service,true, "异步执行任务: {}", &action_name);
                match timeout(timeout_duration, execute_action(action_clone)).await {
                    Ok(result) => {
                        if let Err(e) = result {
                            logging!(error, Type::Service,true, "任务 {} 执行失败: {}", &action_name, e);
                        } else {
                            logging!(info, Type::Service,true, "任务 {} 执行成功", &action_name);
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


pub async fn execute_plural_actions(actions: Vec<Action>) -> Result<()> {
    for action in actions {
        execute_single_action(&action).await;
    }
    Ok(())
}
