// 用户 PATH 注册（Windows）：直接操作 HKCU\Environment 的 Path 值。
//
// 安全实现（不用 setx）：
// - setx 有 1024 字符截断风险，且会把 REG_EXPAND_SZ 降级为 REG_SZ；
// - 这里用 winreg 的 get_raw_value/set_raw_value，保留原值类型
//   （REG_EXPAND_SZ）与未展开的 %...% 形式；
// - 段级幂等：按 ';' 分段、大小写不敏感比较，已存在则不重复添加；
// - 写入后广播 WM_SETTINGCHANGE，让新进程（资源管理器/dsh 等）拿到新 PATH。
//
// 写入结果由 GUI 侧记录到 config.yaml（mcp_path_registered），
// 作为幂等展示与卸载器清理的依据。

use anyhow::Result;

/// 当前可执行文件所在目录（ducker.exe 与 ducker-mcp.exe 同目录）
pub fn exe_dir() -> Result<std::path::PathBuf> {
    let exe = std::env::current_exe()?;
    exe.parent()
        .map(|p| p.to_path_buf())
        .ok_or_else(|| anyhow::anyhow!("failed to get executable directory"))
}

#[cfg(windows)]
pub fn add_to_user_path(dir: &std::path::Path) -> Result<()> {
    modify_user_path(dir, true)
}

#[cfg(windows)]
pub fn remove_from_user_path(dir: &std::path::Path) -> Result<()> {
    modify_user_path(dir, false)
}

#[cfg(windows)]
pub fn is_in_user_path(dir: &std::path::Path) -> Result<bool> {
    use winreg::enums::*;

    let dir_str = dir.to_string_lossy().to_string();
    let hkcu = winreg::RegKey::predef(HKEY_CURRENT_USER);
    let env = hkcu
        .open_subkey("Environment")
        .map_err(|e| anyhow::anyhow!("open HKCU\\Environment failed: {e}"))?;

    match env.get_raw_value("Path") {
        Ok(value) => {
            let raw = raw_value_to_string(&value);
            Ok(path_contains_segment(&raw, &dir_str))
        }
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(false),
        Err(e) => Err(anyhow::anyhow!("read Path value failed: {e}")),
    }
}

/// 以原类型读写 Path 值，按段幂等地添加/移除目录
#[cfg(windows)]
fn modify_user_path(dir: &std::path::Path, add: bool) -> Result<()> {
    use std::os::windows::ffi::OsStrExt;
    use winapi::shared::minwindef::{LPARAM, WPARAM};
    use winapi::um::winuser::{SendMessageTimeoutW, HWND_BROADCAST, SMTO_ABORTIFHUNG};
    use winreg::enums::*;
    use winreg::{RegKey, RegValue};

    let dir_str = dir.to_string_lossy().to_string();
    if dir_str.is_empty() {
        anyhow::bail!("executable directory is empty");
    }

    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let (env, _) = hkcu
        .create_subkey("Environment")
        .map_err(|e| anyhow::anyhow!("open HKCU\\Environment failed: {e}"))?;

    // 读取原始值，保留类型与未展开形式；Path 可能不存在（极少数精简系统）
    let (raw, value_type) = match env.get_raw_value("Path") {
        Ok(value) => {
            let raw = raw_value_to_string(&value);
            let ty = match value.vtype {
                REG_EXPAND_SZ => REG_EXPAND_SZ,
                REG_SZ => REG_SZ,
                other => {
                    // 未知类型（REG_MULTI_SZ 等）不动注册表，避免破坏数据
                    anyhow::bail!("unsupported Path value type: {other:?}")
                }
            };
            (raw, ty)
        }
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => (String::new(), REG_EXPAND_SZ),
        Err(e) => return Err(anyhow::anyhow!("read Path value failed: {e}")),
    };

    let contains = path_contains_segment(&raw, &dir_str);
    if add == contains {
        // 幂等：目标状态已达成，不写注册表
        return Ok(());
    }

    let new_raw = if add {
        if raw.is_empty() {
            dir_str.clone()
        } else if raw.ends_with(';') {
            format!("{raw}{dir_str}")
        } else {
            format!("{raw};{dir_str}")
        }
    } else {
        raw.split(';')
            .filter(|seg| !seg.trim().eq_ignore_ascii_case(&dir_str))
            .collect::<Vec<_>>()
            .join(";")
    };

    // 手工编码为 UTF-16LE + null 结尾，与 winreg 的 String 解码路径严格对应
    let reg_value = RegValue {
        bytes: to_utf16le_bytes(&new_raw),
        vtype: value_type,
    };
    env.set_raw_value("Path", &reg_value)
        .map_err(|e| anyhow::anyhow!("write Path value failed: {e}"))?;

    // 广播环境变量变更，让已运行进程（资源管理器等）感知
    let env_str: Vec<u16> = std::ffi::OsStr::new("Environment")
        .encode_wide()
        .chain(std::iter::once(0))
        .collect();
    unsafe {
        SendMessageTimeoutW(
            HWND_BROADCAST,
            0x001A, // WM_SETTINGCHANGE
            0 as WPARAM,
            env_str.as_ptr() as LPARAM,
            SMTO_ABORTIFHUNG,
            5000,
            std::ptr::null_mut(),
        );
    }

    crate::logging!(
        info,
        crate::utils::logging::Type::System,
        true,
        "{} user PATH: {} (broadcast sent)",
        if add { "added to" } else { "removed from" },
        dir_str
    );
    Ok(())
}

/// 按 ';' 分段、大小写不敏感地判断目录是否已在 Path 中
#[cfg(windows)]
fn path_contains_segment(path_value: &str, dir: &str) -> bool {
    path_value
        .split(';')
        .any(|seg| seg.trim().eq_ignore_ascii_case(dir))
}

/// 解码 REG_SZ / REG_EXPAND_SZ 原始字节（UTF-16LE，去 null 结尾），
/// 保留未展开的 %...% 形式
#[cfg(windows)]
fn raw_value_to_string(value: &winreg::RegValue) -> String {
    let words: Vec<u16> = value
        .bytes
        .chunks_exact(2)
        .map(|pair| u16::from_le_bytes([pair[0], pair[1]]))
        .collect();
    let mut s = String::from_utf16_lossy(&words);
    while s.ends_with('\u{0}') {
        s.pop();
    }
    s
}

#[cfg(windows)]
fn to_utf16le_bytes(s: &str) -> Vec<u8> {
    use std::os::windows::ffi::OsStrExt;
    std::ffi::OsStr::new(s)
        .encode_wide()
        .chain(std::iter::once(0)) // null 结尾
        .flat_map(|w| w.to_le_bytes())
        .collect()
}

#[cfg(all(windows, test))]
mod tests {
    use super::*;

    #[test]
    fn segment_match_is_case_insensitive_and_trimmed() {
        assert!(path_contains_segment(
            r"C:\Program Files;%USERPROFILE%\bin;D:\Tools",
            r"d:\tools"
        ));
        assert!(path_contains_segment("D:\\Tools;", " D:\\Tools "));
        assert!(!path_contains_segment(r"C:\Program Files", r"D:\Tools"));
        assert!(!path_contains_segment("", r"D:\Tools"));
    }

    #[test]
    fn utf16le_roundtrip_keeps_expand_form() {
        let original = r"%USERPROFILE%\AppData\Local\com.Yoaken.ducker;C:\Program Files";
        let bytes = to_utf16le_bytes(original);
        // RegValue 形态回读
        let value = winreg::RegValue {
            bytes,
            vtype: winreg::enums::REG_EXPAND_SZ,
        };
        assert_eq!(raw_value_to_string(&value), original);
    }

    /// 只读查询真实注册表，验证 raw API 读取路径（不写入）
    #[test]
    fn is_in_user_path_read_only_probe() {
        let probe = std::env::temp_dir().join("ducker-path-probe-nonexistent");
        assert!(!is_in_user_path(&probe).unwrap());
    }
}

#[cfg(not(windows))]
pub fn add_to_user_path(_dir: &std::path::Path) -> Result<()> {
    anyhow::bail!("PATH registration is only supported on Windows")
}

#[cfg(not(windows))]
pub fn remove_from_user_path(_dir: &std::path::Path) -> Result<()> {
    anyhow::bail!("PATH registration is only supported on Windows")
}

#[cfg(not(windows))]
pub fn is_in_user_path(_dir: &std::path::Path) -> Result<bool> {
    Ok(false)
}
