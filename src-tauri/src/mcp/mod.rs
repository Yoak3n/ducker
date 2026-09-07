// ducker-mcp: Model Context Protocol server over stdio。
// 协议：JSON-RPC 2.0，每行一条消息（LSP 风格分行帧）。
// stdout 仅输出协议帧；所有日志走 stderr 或丢弃，绝不写 stdout。

pub mod exec;
pub mod tools;

use std::path::PathBuf;

use serde_json::{json, Value};

use crate::store::db::Database;

const PROTOCOL_VERSION: &str = "2024-11-05";

pub fn run() {
    // 文件日志（前缀区分进程，避免与 GUI 的 ducker-*.log 混淆）。
    // CLI 一次性调用模式也初始化：任何经 MCP 的动作执行都有落盘轨迹。
    crate::utils::file_log::init("ducker-mcp");

    // 一次性 CLI 模式：ducker-mcp call <tool> [json-args]，执行单个工具立即退出
    let args: Vec<String> = std::env::args().collect();
    if args.len() >= 2 && args[1] == "call" {
        run_cli_call(&args);
        return;
    }

    // 以下为 stdio MCP server 模式
    // stdout 是协议通道，关闭业务日志的 console 输出
    crate::utils::logging::set_console_output(false);

    // 服务门控：设置面板允许后才工作（work review 式开关）
    if !service_enabled() {
        eprintln!(
            "[ducker-mcp] MCP service is disabled. Enable it in ducker Settings (enable_mcp)."
        );
        std::process::exit(2);
    }

    let db = match open_database() {
        Ok(db) => std::sync::Arc::new(db),
        Err(e) => {
            eprintln!("[ducker-mcp] failed to open database: {e:#}");
            std::process::exit(1);
        }
    };

    let runtime = match tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
    {
        Ok(rt) => rt,
        Err(e) => {
            eprintln!("[ducker-mcp] failed to build tokio runtime: {e}");
            std::process::exit(1);
        }
    };

    if let Err(e) = runtime.block_on(serve(db)) {
        eprintln!("[ducker-mcp] stdio loop terminated: {e:#}");
        std::process::exit(1);
    }
}

fn open_database() -> anyhow::Result<Database> {
    Database::new(resolve_db_dir()?)
}

/// 一次性命令行模式：执行单个工具后立即退出，不启动 stdio 循环。
/// 用法：ducker-mcp call <tool> [json-arguments]
/// 适合脚本调用；stdio MCP 客户端（dsh）不受影响，二者共用工具实现与门控。
fn run_cli_call(args: &[String]) {
    // CLI 拥有 stdout，但保持与 server 模式一致的日志行为，输出干净
    crate::utils::logging::set_console_output(false);

    let tool = match args.get(2) {
        Some(t) => t.clone(),
        None => {
            eprintln!("usage: ducker-mcp call <tool> [json-arguments|\"-\"]");
            eprintln!("example: ducker-mcp call task_list \"{{}}\"");
            eprintln!("         echo '{{\"completed\":false}}' | ducker-mcp call task_list -");
            eprintln!(
                "tools: {}",
                tools::definitions()
                    .iter()
                    .filter_map(|t| t.get("name").and_then(|n| n.as_str()))
                    .collect::<Vec<_>>()
                    .join(", ")
            );
            std::process::exit(64);
        }
    };
    let arguments: serde_json::Value = match args.get(3) {
        // "-" 占位：从 stdin 读取 JSON，规避 shell 命令行引号/空格转义问题
        Some(s) if s == "-" => {
            use std::io::Read;
            let mut buf = String::new();
            if std::io::stdin().read_to_string(&mut buf).is_err() {
                eprintln!("failed to read JSON from stdin");
                std::process::exit(64);
            }
            match serde_json::from_str(buf.trim()) {
                Ok(v) => v,
                Err(e) => {
                    eprintln!("invalid JSON from stdin: {e}");
                    std::process::exit(64);
                }
            }
        }
        Some(s) => match serde_json::from_str(s) {
            Ok(v) => v,
            Err(e) => {
                eprintln!("invalid JSON arguments: {e}");
                std::process::exit(64);
            }
        },
        None => serde_json::json!({}),
    };

    if !service_enabled() {
        eprintln!(
            "[ducker-mcp] MCP service is disabled. Enable it in ducker Settings (enable_mcp)."
        );
        std::process::exit(2);
    }
    let db = match open_database() {
        Ok(db) => std::sync::Arc::new(db),
        Err(e) => {
            eprintln!("[ducker-mcp] failed to open database: {e:#}");
            std::process::exit(1);
        }
    };

    let runtime = match tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
    {
        Ok(rt) => rt,
        Err(e) => {
            eprintln!("[ducker-mcp] runtime error: {e}");
            std::process::exit(1);
        }
    };

    match runtime.block_on(tools::call(&db, &tool, &arguments)) {        Ok(value) => {
            match serde_json::to_string_pretty(&value) {
                Ok(text) => println!("{text}"),
                Err(e) => {
                    eprintln!("[ducker-mcp] serialize error: {e}");
                    std::process::exit(1);
                }
            }
        }
        Err(message) => {
            eprintln!("[ducker-mcp] {tool} failed: {message}");
            std::process::exit(1);
        }
    }
}

/// MCP 服务开关：读取 GUI 写下的 config.yaml，enable_mcp == true 才允许运行。
/// DUCKER_DB_DIR 显式指定目录时视为开发/测试模式，不受门控。
fn service_enabled() -> bool {
    if std::env::var_os("DUCKER_DB_DIR").is_some() {
        return true;
    }
    let Ok(dir) = resolve_db_dir() else {
        return false;
    };
    let config_path = dir.join("config.yaml");
    let Ok(text) = std::fs::read_to_string(&config_path) else {
        return false;
    };

    #[derive(serde::Deserialize)]
    struct Gate {
        enable_mcp: Option<bool>,
    }
    serde_yaml::from_str::<Gate>(&text)
        .map(|gate| gate.enable_mcp == Some(true))
        .unwrap_or(false)
}

fn resolve_db_dir() -> anyhow::Result<PathBuf> {
    // 测试/多实例场景可用环境变量覆盖
    if let Some(dir) = std::env::var_os("DUCKER_DB_DIR") {
        return Ok(PathBuf::from(dir));
    }
    // 与 GUI 侧 tauri app_data_dir() 保持一致：{data_dir}/{identifier}
    dirs::data_dir()
        .map(|d| d.join(crate::utils::dirs::APP_ID))
        .ok_or_else(|| anyhow::anyhow!("cannot resolve user data dir"))
}

async fn serve(db: std::sync::Arc<Database>) -> anyhow::Result<()> {
    use tokio::io::{AsyncBufReadExt, BufReader};

    log::info!("MCP stdio 服务启动");
    let mut reader = BufReader::new(tokio::io::stdin()).lines();
    let mut out = tokio::io::stdout();

    while let Some(line) = reader.next_line().await? {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let msg: Value = match serde_json::from_str(line) {
            Ok(v) => v,
            Err(e) => {
                write_error(&mut out, Value::Null, -32700, format!("Parse error: {e}")).await?;
                continue;
            }
        };

        let id = msg.get("id").cloned();
        let params = msg.get("params").cloned().unwrap_or(Value::Null);

        let Some(method) = msg.get("method").and_then(|v| v.as_str()).map(str::to_owned) else {
            if let Some(id) = id {
                write_error(&mut out, id, -32600, "Invalid Request: missing method".into()).await?;
            }
            continue;
        };

        // 通知（无 id）：不回复。initialized/cancelled 等静默忽略。
        let Some(id) = id else {
            continue;
        };

        match dispatch(&db, &method, &params).await {
            Ok(result) => write_response(&mut out, id, result).await?,
            Err((code, message)) => write_error(&mut out, id, code, message).await?,
        }
    }
    Ok(())
}

async fn dispatch(
    db: &std::sync::Arc<Database>,
    method: &str,
    params: &Value,
) -> Result<Value, (i64, String)> {
    match method {
        "initialize" => Ok(json!({
            "protocolVersion": PROTOCOL_VERSION,
            "capabilities": { "tools": {} },
            "serverInfo": {
                "name": "ducker-mcp",
                "version": env!("CARGO_PKG_VERSION")
            }
        })),
        "ping" => Ok(json!({})),
        "tools/list" => Ok(json!({ "tools": tools::definitions() })),
        "tools/call" => handle_tools_call(db, params).await,
        other => Err((-32601, format!("Method not found: {other}"))),
    }
}

async fn handle_tools_call(
    db: &std::sync::Arc<Database>,
    params: &Value,
) -> Result<Value, (i64, String)> {
    let name = params
        .get("name")
        .and_then(|v| v.as_str())
        .ok_or_else(|| (-32602, "missing params.name".to_string()))?;
    let arguments = params.get("arguments").cloned().unwrap_or(json!({}));
    // 审计轨迹：谁在什么时刻调用了什么工具（排查异常触发时的关键证据）
    log::info!("MCP tools/call: {name}");

    match tools::call(db, name, &arguments).await {
        Ok(value) => {
            let text = serde_json::to_string_pretty(&value).unwrap_or_else(|_| "{}".into());
            Ok(json!({ "content": [ { "type": "text", "text": text } ] }))
        }
        Err(message) => Ok(json!({
            "content": [ { "type": "text", "text": message } ],
            "isError": true
        })),
    }
}

async fn write_response(
    out: &mut tokio::io::Stdout,
    id: Value,
    result: Value,
) -> anyhow::Result<()> {
    use tokio::io::AsyncWriteExt;
    let frame = json!({ "jsonrpc": "2.0", "id": id, "result": result });
    out.write_all(frame.to_string().as_bytes()).await?;
    out.write_all(b"\n").await?;
    out.flush().await?;
    Ok(())
}

async fn write_error(
    out: &mut tokio::io::Stdout,
    id: Value,
    code: i64,
    message: String,
) -> anyhow::Result<()> {
    use tokio::io::AsyncWriteExt;
    let frame = json!({
        "jsonrpc": "2.0",
        "id": id,
        "error": { "code": code, "message": message }
    });
    out.write_all(frame.to_string().as_bytes()).await?;
    out.write_all(b"\n").await?;
    out.flush().await?;
    Ok(())
}
