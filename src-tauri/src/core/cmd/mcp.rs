// MCP 服务管理命令：状态查询、用户 PATH 注册/注销
use crate::config::Config;
use crate::utils::{user_path, logging::Type};

use anyhow::Result;
use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct McpStatus {
    /// 配置中 MCP 服务开关
    pub enabled: bool,
    /// 当前 exe 目录是否已在用户 PATH 中（实时查注册表）
    pub path_registered: bool,
    /// 配置文件记录的注册结果（卸载器清理依据）
    pub config_recorded: bool,
    /// 当前可执行文件所在目录
    pub exe_dir: String,
    /// ducker-mcp 可执行文件是否与 GUI 同目录存在
    pub mcp_exe_exists: bool,
}

fn current_status() -> Result<McpStatus> {
    let dir = user_path::exe_dir()?;
    let config = Config::global().lock().clone();

    let path_registered = user_path::is_in_user_path(&dir).unwrap_or(false);
    let mcp_exe_exists = {
        #[cfg(windows)]
        {
            dir.join("ducker-mcp.exe").exists()
        }
        #[cfg(not(windows))]
        {
            dir.join("ducker-mcp").exists()
        }
    };

    Ok(McpStatus {
        enabled: config.enable_mcp.unwrap_or(false),
        path_registered,
        config_recorded: config.mcp_path_registered.unwrap_or(false),
        exe_dir: dir.to_string_lossy().to_string(),
        mcp_exe_exists,
    })
}

fn record_path_registered(registered: bool) {
    let global = Config::global();
    let mut guard = global.lock();
    guard.mcp_path_registered = Some(registered);
    guard.save().ok();
    crate::logging!(
        info,
        Type::Config,
        true,
        "mcp_path_registered recorded: {}",
        registered
    );
}

#[tauri::command]
pub async fn get_mcp_status() -> Result<McpStatus, String> {
    current_status().map_err(|e| e.to_string())
}

/// 将 ducker 二进制所在目录加入用户 PATH（幂等）
#[tauri::command]
pub async fn register_mcp_path() -> Result<McpStatus, String> {
    let dir = user_path::exe_dir().map_err(|e| e.to_string())?;
    user_path::add_to_user_path(&dir).map_err(|e| e.to_string())?;
    record_path_registered(true);
    current_status().map_err(|e| e.to_string())
}

/// 从用户 PATH 移除 ducker 二进制所在目录（幂等）
#[tauri::command]
pub async fn unregister_mcp_path() -> Result<McpStatus, String> {
    let dir = user_path::exe_dir().map_err(|e| e.to_string())?;
    user_path::remove_from_user_path(&dir).map_err(|e| e.to_string())?;
    record_path_registered(false);
    current_status().map_err(|e| e.to_string())
}
