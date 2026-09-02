// GUI 侧执行入口：组合服务核心 + 托盘刷新。
// 重试/超时/序列编排/失败对话框/使用计数统一在 service::execute，
// 本文件只保留 GUI 特有的组合，不再有第二份执行逻辑。
use crate::schema::{Action, AppState};
use crate::store::module::ActionManager;
use tauri::Manager;

/// 执行动作（GUI）：成功后刷新托盘“常用动作”菜单。
/// 使用次数由 service::execute::run_sequence 统一记录，这里不再重复计数。
pub async fn execute_action(action: Action) -> Result<String, String> {
    let res = crate::service::execute::execute_single_action(action).await;
    if res.is_ok() {
        let _ = crate::core::tray::Tray::global().update_menu();
    }
    res
}

pub async fn execute_action_by_id(
    app_handle: &tauri::AppHandle,
    id: &str,
) -> Result<String, String> {
    let state = app_handle.state::<AppState>();
    let action = {
        let db = state.db.lock();
        db.get_action(id).map_err(|e| e.to_string())?
    };
    execute_action(Action::from(action)).await
}
