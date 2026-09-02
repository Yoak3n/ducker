// 命令执行共享层：GUI（feat/action.rs）与 MCP（mcp/exec.rs）共用，不依赖 Tauri。
// 注意：前台执行使用 tokio::process，这样外层的 tokio::time::timeout 才能真正生效
// （std::process::output 是阻塞调用，timeout 包不住它）。

/// 前台执行并捕获 stdout（Windows 经 cmd /S /C；其他平台直接执行）
pub async fn execute_command(
    command: String,
    args: Option<Vec<String>>,
) -> Result<String, String> {
    #[cfg(target_os = "windows")]
    {
        let full_command = build_windows_command_line(&command, args.as_ref());
        let mut cmd = tokio::process::Command::new("cmd");
        cmd.args(["/S", "/C", &full_command]);
        // GUI 是 windows_subsystem 程序：前台执行也不允许弹出命令行窗口
        cmd.creation_flags(0x08000000); // CREATE_NO_WINDOW
        let output = cmd.output().await.map_err(|e| e.to_string())?;
        if !output.status.success() {
            return Err(String::from_utf8_lossy(&output.stderr).to_string());
        }
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }
    #[cfg(not(target_os = "windows"))]
    {
        let mut cmd = tokio::process::Command::new(&command);
        if let Some(args) = &args {
            cmd.args(args);
        }
        let output = cmd.output().await.map_err(|e| e.to_string())?;
        if !output.status.success() {
            return Err(String::from_utf8_lossy(&output.stderr).to_string());
        }
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }
}

/// 分离启动（不等待子进程完成）；Windows 下隐藏控制台窗口。
/// 子进程 stdout/stderr 必须置空：MCP 进程的 stdout 是 JSON-RPC 协议通道，
/// 子进程继承句柄会把命令输出写进协议流，破坏响应帧。
pub fn execute_command_indepent(
    command: String,
    args: Option<Vec<String>>,
) -> Result<String, String> {
    use std::process::Stdio;

    #[cfg(target_os = "windows")]
    {
        use std::os::windows::process::CommandExt;
        let full_command = build_windows_command_line(&command, args.as_ref());
        let mut cmd = std::process::Command::new("cmd");
        cmd.args(["/S", "/C", &full_command]);
        cmd.creation_flags(0x08000000); // CREATE_NO_WINDOW
        cmd.stdout(Stdio::null()).stderr(Stdio::null()).stdin(Stdio::null());
        match cmd.spawn() {
            Ok(_child) => Ok("命令已启动，独立运行中".to_string()),
            Err(e) => Err(format!("启动命令失败: {e}")),
        }
    }
    #[cfg(not(target_os = "windows"))]
    {
        let mut cmd = std::process::Command::new(&command);
        if let Some(args) = &args {
            cmd.args(args);
        }
        cmd.stdout(Stdio::null()).stderr(Stdio::null()).stdin(Stdio::null());
        match cmd.spawn() {
            Ok(_child) => Ok("command started in background".to_string()),
            Err(e) => Err(format!("failed to spawn command: {e}")),
        }
    }
}

#[cfg(target_os = "windows")]
pub fn quote_windows_cmd_arg(arg: &str) -> String {
    if arg.is_empty() {
        return "\"\"".to_string();
    }

    if !arg.chars().any(|c| c.is_whitespace() || c == '"' || c == '^' || c == '&' || c == '|' || c == '<' || c == '>') {
        return arg.to_string();
    }

    let escaped = arg.replace('"', "\\\"");
    format!("\"{}\"", escaped)
}

#[cfg(target_os = "windows")]
pub fn build_windows_command_line(command: &str, args: Option<&Vec<String>>) -> String {
    let mut parts = vec![command.to_string()];
    if let Some(args) = args {
        parts.extend(args.iter().map(|arg| quote_windows_cmd_arg(arg)));
    }
    parts.join(" ")
}

#[cfg(test)]
mod tests {
    #[cfg(target_os = "windows")]
    use super::*;

    #[cfg(target_os = "windows")]
    #[test]
    fn windows_command_line_quotes_complex_args() {
        let cmdline = build_windows_command_line(
            "echo",
            Some(&vec![
                "hello world".to_string(),
                "plain".to_string(),
                "a\"b".to_string(),
            ]),
        );
        assert_eq!(cmdline, "echo \"hello world\" plain \"a\\\"b\"");
    }

    #[tokio::test]
    async fn execute_command_returns_stdout() {
        let output = execute_command("echo".to_string(), Some(vec!["hello exec".to_string()]))
            .await
            .unwrap();
        assert!(output.contains("hello exec"));
    }
}
