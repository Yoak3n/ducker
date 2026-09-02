use std::fmt;
use std::sync::atomic::{AtomicBool, Ordering};

/// 控制台（stdout）输出开关。
/// MCP server 进程的 stdout 是 JSON-RPC 协议通道，必须在此进程启动时关闭，
/// 避免 `logging!(..., true, ...)` 的 println! 污染协议流。
static CONSOLE_OUTPUT: AtomicBool = AtomicBool::new(true);

pub fn set_console_output(enabled: bool) {
    CONSOLE_OUTPUT.store(enabled, Ordering::Relaxed);
}

pub fn console_output() -> bool {
    CONSOLE_OUTPUT.load(Ordering::Relaxed)
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Type {
    Cmd,
    Core,
    Config,
    Setup,
    System,
    Service,
    Database,
    Hotkey,
    Window,
    Tray,
    Timer,
    Frontend,
    Backup,
    Lightweight,
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Type::Cmd => write!(f, "[Cmd]"),
            Type::Core => write!(f, "[Core]"),
            Type::Config => write!(f, "[Config]"),
            Type::Setup => write!(f, "[Setup]"),
            Type::System => write!(f, "[System]"),
            Type::Service => write!(f, "[Service]"),
            Type::Database => write!(f, "[Database]"),
            Type::Hotkey => write!(f, "[Hotkey]"),
            Type::Window => write!(f, "[Window]"),
            Type::Tray => write!(f, "[Tray]"),
            Type::Timer => write!(f, "[Timer]"),
            Type::Frontend => write!(f, "[Frontend]"),
            Type::Backup => write!(f, "[Backup]"),
            Type::Lightweight => write!(f, "[Lightweight]"),
        }
    }
}

#[macro_export]
macro_rules! error {
    ($result: expr) => {
        log::error!(target: "app", "{}", $result);
    };
}

#[macro_export]
macro_rules! log_err {
    ($result: expr) => {
        if let Err(err) = $result {
            log::error!(target: "app", "{err}");
        }
    };

    ($result: expr, $err_str: expr) => {
        if let Err(_) = $result {
            log::error!(target: "app", "{}", $err_str);
        }
    };
}

#[macro_export]
macro_rules! trace_err {
    ($result: expr, $err_str: expr) => {
        if let Err(err) = $result {
            log::trace!(target: "app", "{}, err {}", $err_str, err);
        }
    }
}

/// wrap the anyhow error
/// transform the error to String
#[macro_export]
macro_rules! wrap_err {
    ($stat: expr) => {
        match $stat {
            Ok(a) => Ok(a),
            Err(err) => {
                log::error!(target: "app", "{}", err.to_string());
                Err(format!("{}", err.to_string()))
            }
        }
    };
}

#[macro_export]
macro_rules! logging {
    // 带 println 的版本（支持格式化参数）；console_output 关闭时（MCP 模式）不污染 stdout
    ($level:ident, $type:expr, true, $($arg:tt)*) => {{
        if $crate::utils::logging::console_output() {
            println!("{} {}", $type, format_args!($($arg)*));
        }
        log::$level!(target: "app", "{} {}", $type, format_args!($($arg)*));
    }};

    // 带 println 的版本（使用 false 明确不打印）
    ($level:ident, $type:expr, false, $($arg:tt)*) => {{
        log::$level!(target: "app", "{} {}", $type, format_args!($($arg)*));
    }};

    // 不带 print 参数的版本（默认不打印）；块展开，表达式位可用
    ($level:ident, $type:expr, $($arg:tt)*) => {{
        log::$level!(target: "app", "{} {}", $type, format_args!($($arg)*));
    }};
}

#[macro_export]
macro_rules! logging_error {
    // Handle Result<T, E>
    ($type:expr, $expr:expr) => {
        if let Err(err) = $expr {
            log::error!(target: "app", "[{}] {}", $type, err);
        }
    };

    // Handle formatted message: always print to stdout and log as error
    ($type:expr, $fmt:literal $(, $arg:expr)*) => {
        log::error!(target: "app", "[{}] {}", $type, format_args!($fmt $(, $arg)*));
    };
}
