// MCP 侧执行入口：完全复用 service::execute 统一核心，
// 无头环境差异（系统打开器、通知跳过、不弹窗）由 ExecContext::Headless 收敛。
use std::sync::Arc;

use serde_json::json;

use crate::{
    schema::Action,
    service::execute::{run_sequence, ExecContext},
    store::{
        db::Database,
        module::ActionManager,
    },
};

/// 取库中 action → 执行 → 返回结构化结果。
/// 使用计数由 service 核心（run_sequence）统一记录。
pub async fn execute_action_by_id(
    db: Arc<Database>,
    id: &str,
) -> Result<serde_json::Value, String> {
    let record = db
        .get_action(id)
        .map_err(|e| e.to_string())?;
    let action = Action::from(record);
    let name = action.name.clone();
    let typ = action.typ.clone();

    let ctx = ExecContext::headless(db);
    let outcomes = run_sequence(&ctx, vec![action], false).await;
    let outcome = outcomes
        .first()
        .ok_or_else(|| "no execution outcome".to_string())?;

    Ok(json!({
        "id": id,
        "name": name,
        "type": typ,
        "success": outcome.success,
        "output": outcome.output,
        "error": outcome.error
    }))
}
