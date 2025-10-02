use crate::schema::{Action, ActionType, AppState};
use crate::store::module::ActionManager;
use crate::utils::logging::Type;
use crate::{get_app_handle, logging};
use std::process::Command;
use tauri_plugin_notification::NotificationExt;
use tauri_plugin_opener::OpenerExt;
use tauri::Manager;

pub async fn execute_action(action: Action) -> Result<String, String> {
    logging!(info, Type::Cmd,true, "执行动作{:?}",action);
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

                // 执行所有 actions
                let mut results = Vec::new();
                let mut success_count = 0;
                let mut error_count = 0;

                for sub_action in actions_to_execute {
                    let result = Box::pin(execute_action(sub_action.clone())).await;
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
use crate::core::handle::Handle;
async fn open_path(path: String) -> Result<(), String> {
    let handle = Handle::global();
    let app_handle = handle.app_handle().unwrap();
    let opener = app_handle.opener();
    if let Err(output) = opener.open_path(path, None::<&str>) {
        return Err(output.to_string());
    };
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
