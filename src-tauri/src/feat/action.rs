use crate::schema::{Action, ActionType, AppState};
use crate::store::module::ActionManager;
use crate::utils::logging::Type;
use crate::{get_app_handle, logging};
use std::process::Command;
use std::time::Duration;
use tauri_plugin_notification::NotificationExt;
use tauri_plugin_opener::OpenerExt;
use tauri::Manager;
use tokio::time::timeout;

pub async fn execute_action(action: Action) -> Result<String, String> {
    // 否则使用原始的执行逻辑
    execute_action_internal(action).await
}
use crate::core::handle::Handle;
async fn open_path(path: String) -> Result<(), String> {
    let handle = Handle::global();
    let app_handle = handle.app_handle().unwrap();
    let opener = app_handle.opener();
    if let Err(output) = opener.open_path(path, None::<&str>) {
        return Err(output.to_string());
    }
    Ok(())
}

async fn open_url(url: String) -> Result<(), String> {
    let handle = Handle::global();
    let app_handle = handle.app_handle().unwrap();
    let opener = app_handle.opener();
    if let Err(output) = opener.open_url(url, None::<&str>) {
        return Err(output.to_string());
    };
    Ok(())
}

#[cfg(target_os = "windows")]
async fn execute_command(command: String, args: Option<Vec<String>>) -> Result<String, String> {
    let mut cmd = Command::new("cmd");
    cmd.arg("/C").arg(&command);
    if let Some(args) = args.as_ref() {
        for arg in args {
            cmd.arg(arg);
        }
    }
    let output = cmd.output().map_err(|e| e.to_string())?;
    if !output.status.success() {
        let error_message = String::from_utf8_lossy(&output.stderr);
        return Err(error_message.to_string());
    }
    let output_message = String::from_utf8_lossy(&output.stdout);

    Ok(output_message.to_string())
}

#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;
#[cfg(target_os = "windows")]
async fn execute_command_indepent(
    command: String,
    args: Option<Vec<String>>,
) -> Result<String, String> {
    let mut cmd = Command::new("cmd");
    cmd.arg("/C").arg(&command);
    if let Some(args) = args.as_ref() {
        for arg in args {
            cmd.arg(arg);
        }
    }
    cmd.creation_flags(0x08000000);
    match cmd.spawn() {
        Ok(_child) => {
            // 不等待子进程完成，直接返回成功
            Ok("命令已启动，独立运行中".to_string())
        }
        Err(e) => Err(format!("启动命令失败: {}", e)),
    }
}

/// 执行配置结构体
#[derive(Debug, Clone)]
pub struct ExecutionConfig {
    pub max_retries: usize,
    pub timeout_seconds: u64,
    pub retry_delay_ms: u64,
}

impl Default for ExecutionConfig {
    fn default() -> Self {
        Self {
            max_retries: 3,
            timeout_seconds: 30,
            retry_delay_ms: 1000,
        }
    }
}

impl From<&Action> for ExecutionConfig {
    fn from(action: &Action) -> Self {
        Self {
            max_retries: action.retry.unwrap_or(3),
            timeout_seconds: action.timeout.unwrap_or(30),
            retry_delay_ms: 1000,
        }
    }
}

/// 带有retry和timeout机制的action执行函数，用于处理group Action
pub async fn execute_action_with_retry(action: Action) -> Result<String, String> {
    let config = ExecutionConfig::from(&action);
    let mut last_error = String::new();
    
    for attempt in 0..=config.max_retries {
        logging!(info, Type::Cmd, true, "执行动作 {} (尝试 {}/{})", action.name, attempt + 1, config.max_retries + 1);
        
        // 使用timeout包装执行
        let execution_future = execute_action_internal(action.clone());
        let timeout_duration = Duration::from_secs(config.timeout_seconds);
        
        match timeout(timeout_duration, execution_future).await {
            Ok(Ok(result)) => {
                if attempt > 0 {
                    logging!(info, Type::Cmd, true, "动作 {} 在第 {} 次尝试后成功", action.name, attempt + 1);
                }
                return Ok(result);
            }
            Ok(Err(error)) => {
                last_error = error;
                logging!(warn, Type::Cmd, true, "动作 {} 第 {} 次尝试失败: {}", action.name, attempt + 1, last_error);
            }
            Err(_) => {
                last_error = format!("执行超时 ({}秒)", config.timeout_seconds);
                logging!(warn, Type::Cmd, true, "动作 {} 第 {} 次尝试超时", action.name, attempt + 1);
            }
        }
        
        // 如果不是最后一次尝试，等待一段时间再重试
        if attempt < config.max_retries {
            tokio::time::sleep(Duration::from_millis(config.retry_delay_ms)).await;
        }
    }
    
    let final_error = format!("动作执行失败，已重试 {} 次。最后错误: {}", config.max_retries, last_error);
    logging!(error, Type::Cmd, true, "{}", final_error);
    Err(final_error)
}

/// 原始的action执行逻辑，重命名为内部函数
pub async fn execute_action_internal(action: Action) -> Result<String, String> {
    logging!(info, Type::Cmd, true, "内部执行动作: {:?}", action);
    if let Ok(t) = ActionType::try_from(action.typ.as_str()) {
        match t {
            ActionType::Directory => {
                open_path(action.command).await?;
                Ok(format!("open_dir: ok"))
            }
            ActionType::File => {
                open_path(action.command).await?;
                Ok(format!("open_file: ok"))
            }
            ActionType::Url => {
                open_url(action.command).await?;
                Ok(format!("open_url: ok"))
            }
            ActionType::Command => {
                if action.wait > 0 {
                    let output = execute_command(action.command, action.args).await?;
                    Ok(output)
                } else {
                    let output = execute_command_indepent(action.command, action.args).await?;
                    Ok(output)
                }
            }
            ActionType::Notice => {
                let handle = Handle::global();
                let app_handle = handle.app_handle().unwrap();
                let notification = app_handle.notification();
                
                // title 从 command 中获取
                let title = if action.command.trim().is_empty() {
                    "Ducker"
                } else {
                    &action.command
                };
                
                // body 从 args 中获取
                let body = if let Some(args) = &action.args {
                    if let Some(custom_body) = args.first() {
                        if !custom_body.trim().is_empty() {
                            custom_body.trim()
                        } else {
                            "通知消息"
                        }
                    } else {
                        "通知消息"
                    }
                } else {
                    "通知消息"
                };
                
                notification.builder()
                    .title(title)
                    .body(body)
                    .show()
                    .unwrap();
                Ok(format!("notice: sent notification with title '{}' and body '{}'", title, body))
            }
            ActionType::Group => {
                // 解析 args 中的 action IDs（逗号分隔）
                let action_ids = if let Some(args) = &action.args {
                    args.iter()
                        .flat_map(|arg| arg.split(','))
                        .map(|id| id.trim().to_string())
                        .filter(|id| !id.is_empty())
                        .collect::<Vec<String>>()
                } else {
                    Vec::new()
                };

                if action_ids.is_empty() {
                    return Ok("group: no actions to execute".to_string());
                }

                // 获取数据库实例并立即获取数据，然后释放锁
                let actions_to_execute = {
                    let app_handle = get_app_handle!();
                    let state = app_handle.state::<AppState>();
                    let db = state.db.lock();
                    
                    match db.get_actions(&action_ids) {
                        Ok(action_records) => {
                            action_records.into_iter()
                                .map(Action::from)
                                .collect::<Vec<Action>>()
                        }
                        Err(e) => {
                            return Err(format!("Failed to get actions: {}", e));
                        }
                    }
                };

                if actions_to_execute.is_empty() {
                    return Ok("group: no valid actions found".to_string());
                }

                // 执行所有 actions，使用带retry的版本
                let mut results = Vec::new();
                let mut success_count = 0;
                let mut error_count = 0;

                for sub_action in actions_to_execute {
                    let result = Box::pin(execute_action_with_retry(sub_action.clone())).await;
                    match result {
                        Ok(result) => {
                            results.push(format!("✓ {}: {}", sub_action.name, result));
                            success_count += 1;
                        }
                        Err(error) => {
                            results.push(format!("✗ {}: {}", sub_action.name, error));
                            error_count += 1;
                        }
                    }
                    // 等待指定时间避免过快执行
                    tokio::time::sleep(Duration::from_millis(sub_action.wait as u64)).await;
                }

                let summary = format!(
                    "group: executed {} actions (success: {}, failed: {})",
                    success_count + error_count,
                    success_count,
                    error_count
                );

                if error_count > 0 {
                    // 如果有错误，返回详细信息
                    Ok(format!("{}\nDetails:\n{}", summary, results.join("\n")))
                } else {
                    // 如果全部成功，返回简要信息
                    Ok(summary)
                }
            }
        }
    } else {
        return Err("未知操作类型".to_string());
    }
}
