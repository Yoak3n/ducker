// 轻量文件日志：实现 log::Log 门面，零新增依赖。
//
// 背景：GUI 无持久化日志（init.rs 的 log4rs 方案被注释），排查定时任务
// 「晚一小时执行」类问题时无轨迹可查。此模块让 GUI（ducker）与
// MCP（ducker-mcp）两个进程的运行日志都落到文件，供事后精确追踪。
//
// 用法：进程入口最早处调用 `file_log::init(prefix)`（prefix 区分进程），
// 此后所有 log::*!/logging! 调用按 Info 及以上级别落盘，一行代码不用改。
//
// 行为：
// - 目录：{data_dir}/{APP_ID}/logs/（与数据库同目录），文件名 {prefix}-YYYYMMDD.log
// - 按天滚动：写入时发现跨天自动切新文件；写入即 flush，崩溃少丢行
// - 保留 LOG_KEEP_DAYS 天，初始化时清理更早的文件
// - 任何 IO 失败都静默降级为空操作，绝不 panic、绝不影响主流程
// - 每分钟一次的定时器刷新日志已在调用点降为 debug，Info 级文件日志天然不含

use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};

use anyhow::anyhow;
use chrono::{Local, NaiveDate};
use parking_lot::Mutex;

use crate::utils::dirs::APP_ID;

/// 日志文件保留天数
const LOG_KEEP_DAYS: i64 = 14;

/// 每进程全局状态；None 表示日志未初始化或 IO 失败已禁用
static STATE: Mutex<Option<LogState>> = Mutex::new(None);
static INSTALLED: AtomicBool = AtomicBool::new(false);

struct LogState {
    prefix: String,
    dir: PathBuf,
    /// 当前打开的文件与所属日期；None = 打开失败，静默丢弃后续写入
    file: Option<(NaiveDate, File)>,
}

struct FileLogger;

impl log::Log for FileLogger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= log::Level::Info
    }

    fn log(&self, record: &log::Record) {
        if !self.enabled(record.metadata()) {
            return;
        }
        let now = Local::now();
        let today = now.date_naive();
        let mut guard = STATE.lock();
        let Some(state) = guard.as_mut() else {
            return;
        };
        // 跨天滚动：换新文件再写
        if state.file.as_ref().map_or(true, |(d, _)| *d != today) {
            state.file = match open_log_file(&state.dir, &state.prefix, today) {
                Ok(f) => Some((today, f)),
                Err(_) => None,
            };
        }
        let Some((_, file)) = state.file.as_mut() else {
            return;
        };
        let _ = writeln!(
            file,
            "{}.{:03} [{}] {}",
            now.format("%Y-%m-%d %H:%M:%S"),
            now.timestamp_subsec_millis(),
            record.level(),
            record.args()
        );
        let _ = file.flush();
    }

    fn flush(&self) {}
}

/// 安装全局文件日志（每进程只生效一次，重复调用为空操作）。
/// 失败不 panic：任何一步出错时保持现状（无文件日志）。
pub fn init(prefix: &str) {
    if INSTALLED.swap(true, Ordering::SeqCst) {
        return;
    }
    let Ok(dir) = resolve_log_dir() else {
        return;
    };
    let _ = std::fs::create_dir_all(&dir);
    cleanup_old_logs(&dir, prefix);

    let today = Local::now().date_naive();
    let mut file = open_log_file(&dir, prefix, today).ok().map(|f| (today, f));
    if let Some((_, f)) = file.as_mut() {
        // 启动标记：区分进程实例，便于对齐排查时间线
        let _ = writeln!(
            f,
            "{}.{:03} [INFO] === {} 启动 pid={} version={} ===",
            Local::now().format("%Y-%m-%d %H:%M:%S"),
            Local::now().timestamp_subsec_millis(),
            prefix,
            std::process::id(),
            env!("CARGO_PKG_VERSION")
        );
        let _ = f.flush();
    }

    *STATE.lock() = Some(LogState {
        prefix: prefix.to_string(),
        dir,
        file,
    });

    if log::set_boxed_logger(Box::new(FileLogger)).is_ok() {
        // Info 及以上落盘；每分钟的刷新日志（debug 级）被天然排除
        log::set_max_level(log::LevelFilter::Info);
    }
}

/// 日志目录：与数据库同根（{data_dir}/{APP_ID}/logs），GUI 与 MCP 进程一致。
/// DUCKER_DB_DIR 显式指定时视为开发/测试模式，日志跟着该目录走，
/// 与 mcp::resolve_db_dir 语义一致，避免测试实例污染真实数据目录。
fn resolve_log_dir() -> anyhow::Result<PathBuf> {
    if let Some(dir) = std::env::var_os("DUCKER_DB_DIR") {
        return Ok(PathBuf::from(dir).join("logs"));
    }
    dirs::data_dir()
        .map(|d| d.join(APP_ID).join("logs"))
        .ok_or_else(|| anyhow!("cannot resolve data dir"))
}

fn open_log_file(dir: &PathBuf, prefix: &str, date: NaiveDate) -> std::io::Result<File> {
    let path = dir.join(format!("{prefix}-{}.log", date.format("%Y%m%d")));
    let file = OpenOptions::new().create(true).append(true).open(&path)?;
    // 新文件写入 UTF-8 BOM：让记事本/PowerShell 等按 UTF-8 识别中文日志
    if file.metadata()?.len() == 0 {
        let mut f = &file;
        let _ = f.write_all(&[0xEF, 0xBB, 0xBF]);
    }
    Ok(file)
}

/// 删除超过保留期的日志文件（按文件名中的日期判断；解析失败的文件不动）
fn cleanup_old_logs(dir: &PathBuf, prefix: &str) {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };
    let today = Local::now().date_naive();
    let prefix_tag = format!("{prefix}-");
    for entry in entries.flatten() {
        let name = entry.file_name().to_string_lossy().to_string();
        let Some(rest) = name.strip_prefix(&prefix_tag) else {
            continue;
        };
        let Some(date_part) = rest.strip_suffix(".log") else {
            continue;
        };
        let Ok(date) = NaiveDate::parse_from_str(date_part, "%Y%m%d") else {
            continue;
        };
        if today.signed_duration_since(date).num_days() >= LOG_KEEP_DAYS {
            let _ = std::fs::remove_file(entry.path());
        }
    }
}
