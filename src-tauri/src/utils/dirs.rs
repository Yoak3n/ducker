use std::path::PathBuf;

use anyhow::Result;
use once_cell::sync::OnceCell;

// use crate::core::handle;

pub static SETUP_CONFIG: &str = "config.yaml";
pub static PORTABLE_FLAG: OnceCell<bool> = OnceCell::new();

pub fn init_portable_flag() -> Result<()> {
    use tauri::utils::platform::current_exe;

    let app_exe = current_exe()?;
    if let Some(dir) = app_exe.parent() {
        let dir = PathBuf::from(dir).join(".config/PORTABLE");

        if dir.exists() {
            PORTABLE_FLAG.get_or_init(|| true);
        }
    }
    PORTABLE_FLAG.get_or_init(|| false);
    Ok(())
}
pub fn app_home_dir() -> Result<PathBuf> {
    use std::env::current_exe;
    let exe_path = current_exe()?;
    let install_dir = if cfg!(target_os = "macos") {
        // macOS: 可执行文件位于 .app/Contents/MacOS/ 下
        exe_path
            .parent() // MacOS 目录
            .and_then(|p| p.parent()) // Contents 目录
            .and_then(|p| p.parent()) // .app 目录
            .unwrap()
            .to_path_buf()
    } else {
        // Windows 和 Linux: 可执行文件在安装目录的子目录或根目录
        exe_path.parent().unwrap().to_path_buf()
    };
    Ok(install_dir)
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
