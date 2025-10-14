use anyhow::Result;
use once_cell::sync::OnceCell;
use std::path::PathBuf;
use tauri::Manager;

use crate::core::handle;

// use crate::core::handle;

pub static SETUP_CONFIG: &str = "config.yaml";
pub static APP_ID: &str = "com.Yoaken.ducker";
// pub static PORTABLE_FLAG: OnceCell<bool> = OnceCell::new();

// pub fn init_portable_flag() -> Result<()> {
//     use tauri::utils::platform::current_exe;

//     let app_exe = current_exe()?;
//     if let Some(dir) = app_exe.parent() {
//         let dir = PathBuf::from(dir).join(".config/PORTABLE");

//         if dir.exists() {
//             PORTABLE_FLAG.get_or_init(|| true);
//         }
//     }
//     PORTABLE_FLAG.get_or_init(|| false);
//     Ok(())
// }

pub fn app_home_dir() -> Result<PathBuf> {
    // use tauri::utils::platform::current_exe;
    // let flag = PORTABLE_FLAG.get().unwrap_or(&false);
    // if *flag {
    //     let app_exe = current_exe()?;
    //     let app_exe = dunce::canonicalize(app_exe)?;
    //     let app_dir = app_exe
    //         .parent()
    //         .ok_or(anyhow::anyhow!("failed to get the portable app dir"))?;
    //     return Ok(PathBuf::from(app_dir).join(".config").join(APP_ID));
    // }
    let app_handle = match handle::Handle::global().app_handle() {
        Some(handle) => handle,
        None => {
            log::warn!(target: "app", "app_handle not initialized, using default path");
            // 使用可执行文件目录作为备用
            let exe_path = tauri::utils::platform::current_exe()?;
            let exe_dir = exe_path
                .parent()
                .ok_or(anyhow::anyhow!("failed to get executable directory"))?;

            // 使用系统临时目录 + 应用ID
            #[cfg(target_os = "windows")]
            {
                if let Some(local_app_data) = std::env::var_os("LOCALAPPDATA") {
                    let path = PathBuf::from(local_app_data).join(APP_ID);
                    return Ok(path);
                }
            }

            #[cfg(target_os = "macos")]
            {
                if let Some(home) = std::env::var_os("HOME") {
                    let path = PathBuf::from(home)
                        .join("Library")
                        .join("Application Support")
                        .join(APP_ID);
                    return Ok(path);
                }
            }

            #[cfg(target_os = "linux")]
            {
                if let Some(home) = std::env::var_os("HOME") {
                    let path = PathBuf::from(home)
                        .join(".local")
                        .join("share")
                        .join(APP_ID);
                    return Ok(path);
                }
            }

            // 如果无法获取系统目录，则回退到可执行文件目录
            let fallback_dir = PathBuf::from(exe_dir).join(".config").join(APP_ID);
            log::warn!(target: "app", "Using fallback data directory: {fallback_dir:?}");
            return Ok(fallback_dir);
        }
    };
    match app_handle.path().data_dir() {
        Ok(dir) => Ok(dir.join(APP_ID)),
        Err(e) => {
            log::error!(target: "app", "Failed to get the app home directory: {e}");
            Err(anyhow::anyhow!("Failed to get the app homedirectory"))
        }
    }
}

pub fn config_path() -> Result<PathBuf> {
    Ok(app_home_dir()?.join(SETUP_CONFIG))
}

pub fn path_to_str(path: &PathBuf) -> Result<&str> {
    let path_str = path
        .as_os_str()
        .to_str()
        .ok_or(anyhow::anyhow!("failed to get path from {:?}", path))?;
    Ok(path_str)
}

/// get the resources dir
pub fn app_resources_dir() -> Result<PathBuf> {
    // 避免在Handle未初始化时崩溃
    let app_handle = match handle::Handle::global().app_handle() {
        Some(handle) => handle,
        None => {
            log::warn!(target: "app", "app_handle not initialized in app_resources_dir, using fallback");
            // 使用可执行文件目录作为备用
            let exe_dir = tauri::utils::platform::current_exe()?
                .parent()
                .ok_or(anyhow::anyhow!("failed to get executable directory"))?
                .to_path_buf();
            return Ok(exe_dir.join("resources"));
        }
    };

    match app_handle.path().resource_dir() {
        Ok(dir) => Ok(dir.join("resources")),
        Err(e) => {
            log::error!(target: "app", "Failed to get the resource directory: {e}");
            Err(anyhow::anyhow!("Failed to get the resource directory"))
        }
    }
}