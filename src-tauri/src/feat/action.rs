use crate::schema::Action;
use std::process::Command;
use tauri_plugin_opener::OpenerExt;

pub async fn execute_action(action: Action) -> Result<(), String> {
    match action.typ.as_str() {
        "open_dir" => {
            open_path(action.command).await?;
        }
        "open_file" => {
            open_path(action.command).await?;
        }
        "open_url" => {
            open_url(action.command).await?;
        }
        "exec_command" => {
            if action.wait > 0 {
                execute_command(action.command, action.args).await?;
            } else {
                execute_command_indepent(action.command, action.args).await?;
            }
        }
        _ => {
            return Err("未知操作类型".to_string());
        }
    }

    Ok(())
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
