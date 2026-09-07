// Action 执行统一核心。
//
// 设计：单一执行核心 + 执行环境上下文（ExecContext）。
// - GUI 与 MCP（无头）共用同一套：重试/超时策略、序列编排（同步等待/异步并发）、
//   group 递归、使用计数；差异通过 ExecContext 的环境钩子注入。
// - 同步 action（wait > 0）：等待完成后才执行下一个，结束后再等待 wait 毫秒。
// - 异步 action（wait == 0）：立即启动下一个；命令类直接分离运行，其余在后台任务
//   中执行并回收结果（ActionOutcome），不丢结果。
// - 失败交互：同步 action 自动重试耗尽后，GUI 环境弹出错误对话框（重试仅一次/取消）；
//   取消或再失败则中止后续 action。无头环境不弹窗，直接失败。
// - 所有命令执行走 utils::exec_cmd：前台捕获输出且超时真正生效，不弹命令行窗口。

use std::sync::Arc;
use std::time::Duration;

use serde::Serialize;
use tokio::time::timeout;

use crate::{
    core::handle::Handle,
    logging,
    schema::{Action, ActionType, AppState},
    service::hub::Hub,
    store::{
        db::Database,
        module::{ActionManager, TaskManager},
    },
    utils::{
        date::to_datetime_str,
        exec_cmd::{execute_command, execute_command_indepent},
        logging::Type,
    },
};
use tauri::Manager;

/// 未显式配置 timeout 的普通动作（command/file/url/directory/notice）的兜底超时（秒）。
/// 从 30s 放宽到 120s：30s 会把正常耗时操作（如等游戏进入主界面）误判为失败。
const DEFAULT_TIMEOUT_SECS: u64 = 120;
const DEFAULT_GROUP_RETRIES: usize = 3;
const MAX_GROUP_DEPTH: usize = 5;
const RETRY_BUTTON_LABEL: &str = "重试";

/// 单个 action 的结构化执行结果
#[derive(Debug, Clone, Serialize)]
pub struct ActionOutcome {
    pub id: Option<String>,
    pub name: String,
    pub typ: String,
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

impl ActionOutcome {
    fn from_action(action: &Action) -> Self {
        Self {
            id: action.id.clone(),
            name: action.name.clone(),
            typ: action.typ.clone(),
            success: false,
            output: None,
            error: None,
        }
    }
}

/// 执行环境上下文：GUI / 无头（MCP）的差异都收敛在这里
#[derive(Clone)]
pub enum ExecContext {
    /// Tauri GUI 进程（人工触发）：opener 打开路径、系统通知、失败弹重试对话框
    Gui,
    /// Tauri GUI 进程（定时/自动触发）：行为同 Gui，但失败不弹人工重试对话框，
    /// 避免无人值守的定时任务因等待点击而挂起
    GuiAuto,
    /// 无头进程（ducker-mcp）：数据库由进程自持
    Headless(Arc<Database>),
}

impl ExecContext {
    pub fn gui() -> Self {
        Self::Gui
    }

    pub fn gui_auto() -> Self {
        Self::GuiAuto
    }

    pub fn headless(db: Arc<Database>) -> Self {
        Self::Headless(db)
    }

    /// 借用数据库执行一次可能失败的操作（不跨 await 持锁）
    fn with_db<T>(&self, f: impl FnOnce(&Database) -> Result<T, String>) -> Result<T, String> {
        match self {
            Self::Headless(db) => f(db),
            Self::Gui | Self::GuiAuto => {
                let handle = Handle::global()
                    .app_handle()
                    .ok_or_else(|| "app handle not initialized".to_string())?;
                let state = handle.state::<AppState>();
                let db = state.db.lock();
                f(&db)
            }
        }
    }

    /// 用系统默认方式打开 URL/目录/文件
    async fn open_target(&self, target: String) -> Result<String, String> {
        match self {
            Self::Gui | Self::GuiAuto => {
                use tauri_plugin_opener::OpenerExt;
                let handle = Handle::global()
                    .app_handle()
                    .ok_or_else(|| "app handle not initialized".to_string())?;
                let opener = handle.opener();
                // 先按 URL 处理，失败再按路径处理
                if opener.open_url(&target, None::<&str>).is_err() {
                    opener
                        .open_path(&target, None::<&str>)
                        .map_err(|e| e.to_string())?;
                }
                Ok(format!("open: ok ({target})"))
            }
            Self::Headless(_) => {
                use std::process::Stdio;
                #[cfg(target_os = "windows")]
                {
                    let mut cmd = tokio::process::Command::new("cmd");
                    cmd.args(["/C", "start", "", &target]);
                    cmd.creation_flags(0x08000000); // CREATE_NO_WINDOW
                    cmd.stdout(Stdio::null()).stderr(Stdio::null()).stdin(Stdio::null());
                    cmd.spawn().map_err(|e| format!("打开失败: {e}"))?;
                    Ok(format!("open: ok ({target})"))
                }
                #[cfg(not(target_os = "windows"))]
                {
                    #[cfg(target_os = "macos")]
                    let program = "open";
                    #[cfg(all(unix, not(target_os = "macos")))]
                    let program = "xdg-open";
                    let status = tokio::process::Command::new(program)
                        .arg(&target)
                        .stdout(Stdio::null())
                        .stderr(Stdio::null())
                        .status()
                        .await
                        .map_err(|e| format!("打开失败: {e}"))?;
                    if status.success() {
                        Ok(format!("open: ok ({target})"))
                    } else {
                        Err(format!("打开失败: {program} 退出码 {:?}", status.code()))
                    }
                }
            }
        }
    }

    /// 系统通知（Notice 类型）
    fn send_notice(&self, action: &Action) -> Result<String, String> {
        let title = if action.command.trim().is_empty() {
            "Ducker"
        } else {
            &action.command
        };
        let body = action
            .args
            .as_ref()
            .and_then(|args| args.first())
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .unwrap_or_else(|| "通知消息".to_string());

        match self {
            Self::Gui | Self::GuiAuto => {
                use tauri_plugin_notification::NotificationExt;
                let handle = Handle::global()
                    .app_handle()
                    .ok_or_else(|| "app handle not initialized".to_string())?;
                handle
                    .notification()
                    .builder()
                    .title(title)
                    .body(&body)
                    .show()
                    .map_err(|e| e.to_string())?;
                Ok(format!("notice: sent '{title}' / '{body}'"))
            }
            Self::Headless(_) => {
                // 无头进程（ducker-mcp）：notify-rust 直接发 Windows 原生 toast，
                // 不依赖 Tauri。显式传 ducker 的 AUMID（安装器开始菜单快捷方式已注册），
                // 否则 notify-rust 默认回退 PowerShell 的身份（错误的图标与应用名）。
                // 发送失败降级为说明文字，不让通知失败阻塞动作执行。
                let summary = format!("{title}: {body}");
                let result = notify_rust::Notification::new()
                    .app_id(crate::utils::dirs::APP_ID)
                    .summary(&summary)
                    .body(&body)
                    .timeout(notify_rust::Timeout::Milliseconds(6000))
                    .show();
                match result {
                    Ok(_) => Ok(format!("notice: sent (headless toast) '{summary}'")),
                    Err(e) => Ok(format!("notice: 无头通知发送失败（{e}），内容为 '{summary}'")),
                }
            }
        }
    }

    /// 失败交互：返回 true 表示用户选择重试（仅提供一次）。
    /// 无头与定时/自动触发环境不弹窗。
    async fn prompt_retry(&self, action: &Action, error: &str) -> bool {
        match self {
            Self::Headless(_) => false,
            Self::GuiAuto => false,
            Self::Gui => {
                let Some(handle) = Handle::global().app_handle() else {
                    return false;
                };
                let name = action.name.clone();
                let err = error.to_string();
                let task = tokio::task::spawn_blocking(move || {
                    use tauri_plugin_dialog::{DialogExt, MessageDialogButtons, MessageDialogKind};
                    handle
                        .dialog()
                        .message(format!("动作「{}」执行失败：\n{}", name, err))
                        .title("Action 执行失败")
                        .kind(MessageDialogKind::Error)
                        .buttons(MessageDialogButtons::OkCancelCustom(
                            RETRY_BUTTON_LABEL.to_string(),
                            "取消".to_string(),
                        ))
                        .blocking_show_with_result()
                });
                match task.await {
                    Ok(result) => match result {
                        tauri_plugin_dialog::MessageDialogResult::Yes
                        | tauri_plugin_dialog::MessageDialogResult::Ok => true,
                        tauri_plugin_dialog::MessageDialogResult::Custom(s) => {
                            s == RETRY_BUTTON_LABEL
                        }
                        _ => false,
                    },
                    Err(e) => {
                        logging!(error, Type::Service, "执行失败对话框异常: {e}");
                        false
                    }
                }
            }
        }
    }

    /// 成功后记录使用次数（两个环境一致，仅此一处计数）
    fn record_usage(&self, action: &Action) {
        if let Some(id) = &action.id {
            if let Err(e) =
                self.with_db(|db| db.update_action_count(id).map_err(|e| e.to_string()))
            {
                logging!(
                    error,
                    Type::Service,
                    "更新动作 {} 使用次数失败: {e}",
                    action.name
                );
            }
        }
    }
}

/// 执行单个 action 的原始逻辑（单次尝试，不含重试/超时）
async fn run_once(ctx: &ExecContext, action: Action, depth: usize) -> Result<String, String> {
    let t = ActionType::try_from(action.typ.as_str()).map_err(|_| "未知操作类型".to_string())?;
    match t {
        ActionType::Command => execute_command(action.command, action.args).await,
        ActionType::Url | ActionType::Directory | ActionType::File => {
            ctx.open_target(action.command).await
        }
        ActionType::Notice => ctx.send_notice(&action),
        ActionType::Group => {
            if depth >= MAX_GROUP_DEPTH {
                return Err(format!(
                    "group 嵌套超过上限（{MAX_GROUP_DEPTH} 层），可能存在循环引用"
                ));
            }
            let action_ids: Vec<String> = action
                .args
                .unwrap_or_default()
                .iter()
                .flat_map(|arg| arg.split(','))
                .map(|id| id.trim().to_string())
                .filter(|id| !id.is_empty())
                .collect();
            if action_ids.is_empty() {
                return Ok("group: no actions to execute".to_string());
            }
            let sub_actions: Vec<Action> = ctx
                .with_db(|db| {
                    db.get_actions(&action_ids)
                        .map_err(|e| format!("Failed to get actions: {e}"))
                })?
                .into_iter()
                .map(Action::from)
                .collect();
            if sub_actions.is_empty() {
                return Ok("group: no valid actions found".to_string());
            }

            let (mut ok, mut fail) = (0usize, 0usize);
            let mut details = Vec::new();
            for sub in sub_actions {
                // group 成员强制同步：等待完成；未配置 timeout 时用宽松默认（见 run_with_retries），
                // 未配置 retry 时按 DEFAULT_GROUP_RETRIES 重试
                let result = Box::pin(run_with_retries(ctx, sub.clone(), depth + 1, DEFAULT_GROUP_RETRIES)).await;
                match result {
                    Ok(out) => {
                        details.push(format!("✓ {}: {}", sub.name, out));
                        ok += 1;
                    }
                    Err(e) => {
                        details.push(format!("✗ {}: {}", sub.name, e));
                        fail += 1;
                    }
                }
                if sub.wait > 0 {
                    tokio::time::sleep(Duration::from_millis(sub.wait as u64)).await;
                }
            }
            let summary = format!(
                "group: executed {} actions (success: {}, failed: {})",
                ok + fail,
                ok,
                fail
            );
            if fail > 0 {
                Ok(format!("{summary}\nDetails:\n{}", details.join("\n")))
            } else {
                Ok(summary)
            }
        }
    }
}

/// 带自动重试与超时的单 action 执行（策略收敛在此，替代原先三处重复实现）
async fn run_with_retries(
    ctx: &ExecContext,
    action: Action,
    depth: usize,
    default_retries: usize,
) -> Result<String, String> {
    let max_retries = action.retry.unwrap_or(default_retries);
    // group 是编排容器，嵌套成员的超时各自管理，不在容器层再套一层墙钟，
    // 否则「起手式」这类要等待 MAA 启动到主界面的组合动作会在 group 层被误判超时。
    let outer_secs = if action.typ == "group" {
        None
    } else {
        Some(action.timeout.unwrap_or(DEFAULT_TIMEOUT_SECS))
    };
    let mut last_error = String::new();

    for attempt in 0..=max_retries {
        if attempt > 0 {
            logging!(
                info,
                Type::Service,
                true,
                "动作 {} 执行失败，正在重试 ({}/{}): {}",
                action.name,
                attempt,
                max_retries,
                last_error
            );
        }
        let result = match outer_secs {
            Some(timeout_secs) => {
                match timeout(
                    Duration::from_secs(timeout_secs),
                    run_once(ctx, action.clone(), depth),
                )
                .await
                {
                    Ok(Ok(out)) => Ok(out),
                    Ok(Err(e)) => Err(e),
                    Err(_) => Err(format!("任务执行超时（{timeout_secs}秒）")),
                }
            }
            // group 不设外层超时：成员各自有超时上限
            None => run_once(ctx, action.clone(), depth).await,
        };
        match result {
            Ok(out) => return Ok(out),
            Err(e) => last_error = e,
        }
        if attempt < max_retries {
            tokio::time::sleep(Duration::from_millis(1000)).await;
        }
    }
    Err(last_error)
}

/// 执行一批 action 并返回全部结构化结果。
///
/// - wait > 0：同步，等待完成（含自动重试）；失败时 GUI 弹窗提供一次重试，
///   用户取消或重试仍失败则根据 abort_on_error 决定是否中止后续 action。
/// - wait == 0：异步，立即执行下一个；命令类（无重试/超时配置）分离运行，
///   其余在后台任务执行并回收结果。
///
/// 成功的 action 统一在此记录使用次数（无论同步还是异步、无论环境）。
pub async fn run_sequence(
    ctx: &ExecContext,
    actions: Vec<Action>,
    abort_on_error: bool,
) -> Vec<ActionOutcome> {
    let mut outcomes: Vec<ActionOutcome> = Vec::with_capacity(actions.len());
    let mut pending: Vec<tokio::task::JoinHandle<ActionOutcome>> = Vec::new();
    let mut aborted = false;

    for action in actions {
        if aborted {
            let mut o = ActionOutcome::from_action(&action);
            o.error = Some("前序动作失败，已跳过".to_string());
            outcomes.push(o);
            continue;
        }

        if action.wait == 0 {
            // 异步：命令类且无重试/超时配置 → 分离运行，立即返回
            if action.typ == "command"
                && action.retry.unwrap_or(0) == 0
                && action.timeout.is_none()
            {
                let mut o = ActionOutcome::from_action(&action);
                match execute_command_indepent(action.command.clone(), action.args.clone()) {
                    Ok(msg) => {
                        o.success = true;
                        o.output = Some(msg);
                        ctx.record_usage(&action);
                    }
                    Err(e) => o.error = Some(e),
                }
                outcomes.push(o);
                continue;
            }
            // 其余异步动作在后台执行并回收结果
            let ctx_clone = ctx.clone();
            let action_clone = action.clone();
            pending.push(tokio::spawn(async move {
                let mut o = ActionOutcome::from_action(&action_clone);
                match run_with_retries(&ctx_clone, action_clone.clone(), 0, 0).await {
                    Ok(out) => {
                        o.success = true;
                        o.output = Some(out);
                        ctx_clone.record_usage(&action_clone);
                    }
                    Err(e) => {
                        o.error = Some(e.clone());
                        logging!(
                            error,
                            Type::Service,
                            true,
                            "异步动作 {} 执行失败: {}",
                            action_clone.name,
                            e
                        );
                    }
                }
                o
            }));
            outcomes.push(ActionOutcome::from_action(&action));
            continue;
        }

        // 同步：等待完成后执行下一个
        let mut outcome = ActionOutcome::from_action(&action);
        match run_with_retries(ctx, action.clone(), 0, 0).await {
            Ok(out) => {
                outcome.success = true;
                outcome.output = Some(out);
                ctx.record_usage(&action);
            }
            Err(first_error) => {
                logging!(error, Type::Service, true, "动作 {} 执行失败: {}", action.name, first_error);
                // GUI 环境给一次人工重试机会
                if ctx.prompt_retry(&action, &first_error).await {
                    match run_once(ctx, action.clone(), 0).await {
                        Ok(out) => {
                            outcome.success = true;
                            outcome.output = Some(out);
                            ctx.record_usage(&action);
                        }
                        Err(e) => outcome.error = Some(e),
                    }
                } else {
                    outcome.error = Some(first_error);
                }
            }
        }

        if !outcome.success {
            if abort_on_error {
                aborted = true;
            }
        } else {
            logging!(info, Type::Service, true, "动作 {} 执行成功", action.name);
        }
        // 同步动作完成后等待 wait 毫秒再执行下一个
        if action.wait > 0 {
            tokio::time::sleep(Duration::from_millis(action.wait as u64)).await;
        }
        outcomes.push(outcome);
    }

    // 回收异步动作的结果（不改变同步结果的推进语义）
    for handle in pending {
        match handle.await {
            Ok(o) => {
                if let Some(slot) = outcomes.iter_mut().find(|slot| !slot.success && slot.output.is_none() && slot.error.is_none() && slot.id == o.id) {
                    *slot = o;
                } else {
                    outcomes.push(o);
                }
            }
            Err(e) => logging!(error, Type::Service, "异步动作结果回收失败: {e}"),
        }
    }

    outcomes
}

// ---------- 兼容旧调用方的薄封装 ----------

/// 顺序执行一批动作；任一同步动作最终失败（含用户取消重试）则返回 Err
pub async fn execute_plural_actions(actions: Vec<Action>) -> Result<String, String> {
    let outcomes = run_sequence(&ExecContext::gui(), actions, true).await;
    finalize_outcomes(outcomes)
}

/// 定时/自动触发场景：行为同 execute_plural_actions，但失败不弹人工重试对话框
pub async fn execute_plural_actions_auto(actions: Vec<Action>) -> Result<String, String> {
    let outcomes = run_sequence(&ExecContext::gui_auto(), actions, true).await;
    finalize_outcomes(outcomes)
}

/// 执行单个动作
pub async fn execute_single_action(action: Action) -> Result<String, String> {
    execute_plural_actions(vec![action]).await
}

fn finalize_outcomes(outcomes: Vec<ActionOutcome>) -> Result<String, String> {
    let mut output = String::new();
    for o in &outcomes {
        if let Some(err) = &o.error {
            return Err(format!("动作 {} 执行失败: {err}", o.name));
        }
        if let Some(out) = &o.output {
            output.push_str(out);
        }
    }
    Ok(output)
}

// ---------- 定时任务链路（GUI 专用，保持原逻辑） ----------

pub async fn marked_tasks_completed(tasks_ids: Vec<String>) -> anyhow::Result<()> {
    logging!(info, Type::Database, "开始更新任务 {} 的状态为已完成", tasks_ids.join(","));
    let app_handle = crate::get_app_handle!();
    let state = app_handle.state::<AppState>();
    {
        let db = state.db.lock();
        for task_id in tasks_ids {
            db.update_task_status(&task_id, true)?;
        }
        logging!(info, Type::Database, true, "更新任务的状态为已完成");
    }
    Ok(())
}

pub async fn execute_tasks(id: &str, ts: i64) -> Result<String, String> {
    let tasks = Hub::global().get_schedule(id, ts).unwrap_or_default();
    if tasks.is_empty() {
        // 到点触发但调度表里已查不到：多为错过后调度表被每分钟刷新清掉，
        // 与 timer 的「到点触发」日志对照即可定位错过的时间窗
        logging!(
            info,
            Type::Service,
            "定时触发但调度表无匹配: id={}, ts={}（任务可能已错过执行窗口）",
            id,
            to_datetime_str(ts)
        );
        return Ok("".to_string());
    }
    logging!(
        info,
        Type::Service,
        true,
        "定时任务执行开始: id={}, 计划时间={}, 任务数={}",
        id,
        to_datetime_str(ts),
        tasks.len()
    );
    let mut tasks_ids = Vec::new();
    let mut out_tasks = String::new();
    let mut first_error: Option<String> = None;

    for task in tasks {
        let actions = task.actions.clone().unwrap_or_default();
        match execute_plural_actions_auto(actions).await {
            Ok(out) => out_tasks += &out,
            Err(e) => {
                // 动作失败只记录并在最终结果里上报，不阻断任务实例的完成推进：
                // 定时触发的周期任务无论成败都应进入下一周期，
                // 否则状态停在未完成、next_period 不再前进，链条中断、次日不再生成实例。
                logging!(
                    error,
                    Type::Service,
                    true,
                    "定时任务 {} 动作执行失败: {}",
                    task.name,
                    e
                );
                if first_error.is_none() {
                    first_error = Some(format!("任务 {}: {}", task.name, e));
                }
            }
        }
        tasks_ids.push(task.id);
    }

    // 无论动作成败，都标记本次触发的实例完成；
    // 周期性实例在此完成回调里创建下一次实例。
    if let Err(e) = marked_tasks_completed(tasks_ids).await {
        logging!(error, Type::Database, true, "更新任务状态失败: {}", e);
    }

    // 有失败时返回 Err 以便上层弹出失败通知，但任务状态已经推进，周期不再断链。
    match first_error {
        Some(e) => Err(format!("定时任务执行失败:{}", e)),
        None => Ok(out_tasks),
    }
}
