use crate::{
    feat::action::execute_action,
    logging,
    schema::{Action, AppState},
    service::execute::execute_plural_actions,
    store::module::ActionManager,
    utils::logging::Type,
};

use anyhow::Result;
use tauri::{AppHandle, State};

#[tauri::command]
pub async fn execute_single_action(action: Action) -> Result<String, String> {
    execute_action(action).await
}

/// 顺序执行一批动作。序列编排（同步等待/异步并发）、重试、失败对话框、
/// 使用计数全部收敛在 service::execute；任一同步动作最终失败则中止并返回 Err。
#[tauri::command]
pub async fn execute_actions(_state: State<'_, AppState>, actions: Vec<Action>) -> Result<(), String> {
    execute_plural_actions(actions).await.map(|_| ())
}

#[tauri::command]
pub async fn create_action(state: State<'_, AppState>, action: Action) -> Result<String, String> {
    let db = state.db.lock();

    let res = db.create_action(&action);
    match res {
        Ok(data) => Ok(data.id),
        Err(e) => {
            logging!(error, Type::Service, true, "创建action失败: {:?}", e);
            Err(e.to_string())
        }
    }
}

#[tauri::command]
pub async fn get_action(state: State<'_, AppState>, id: &str) -> Result<Action, String> {
    let db = state.db.lock();

    let res = db.get_action(id);
    match res {
        Ok(data) => {
            let view = Action::from(data);
            Ok(view)
        }
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
pub async fn update_action(
    state: State<'_, AppState>,
    id: &str,
    action: Action,
) -> Result<Action, String> {
    let db = state.db.lock();

    let res = db.update_action(id, &action);
    match res {
        Ok(data) => {
            let view = Action::from(data);
            Ok(view)
        }
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
pub async fn delete_action(state: State<'_, AppState>, id: &str) -> Result<(), String> {
    let db = state.db.lock();

    let res = db.delete_action(id);
    match res {
        Ok(_) => Ok(()),
        Err(e) => Err(e.to_string()),
    }
}

use tauri_plugin_dialog::DialogExt;
#[tauri::command]
pub async fn select_file(app: AppHandle, file: bool) -> Result<String, String> {
    if file {
        let file_path = app.dialog().file().blocking_pick_file();
        if let Some(file) = file_path {
            return Ok(file.to_string());
        } else {
            return Err("未选择文件".to_string());
        }
    } else {
        let file_path = app.dialog().file().blocking_pick_folder();
        if let Some(file) = file_path {
            return Ok(file.to_string());
        } else {
            return Err("未选择文件夹".to_string());
        }
    }
}

#[tauri::command]
pub async fn get_all_actions(state: State<'_, AppState>) -> Result<Vec<Action>, String> {
    let res = {
        let db = state.db.lock();
        db.get_all_actions()
    };
    match res {
        Ok(data) => {
            let views = Vec::from_iter(data.into_iter().map(Action::from));
            Ok(views)
        }
        Err(e) => Err(e.to_string()),
    }
}

// keep Result import used (create_action error type)
#[allow(unused_imports)]
use anyhow::Result as _AnyhowResult;
#[allow(unused_imports)]
use anyhow as _anyhow;
