// MCP tools：task 与 action 两个 domain 的读写接口。
// 全部复用 store 层 trait（TaskManager / ActionManager / PeriodicTaskManager），不直接写 SQL。

use std::sync::Arc;

use serde_json::{json, Value};

use crate::{
    schema::{task_view_from_record, Action, PeriodicTask, PeriodicTaskData, TaskData},
    store::{
        db::Database,
        module::{ActionManager, PeriodicTaskManager, TaskManager},
    },
    utils::date::to_datetime_str,
};

pub fn definitions() -> Vec<Value> {
    vec![
        json!({
            "name": "task_list",
            "description": "List ducker tasks. Each task includes its actions and child tasks. Optionally filter by completion status.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "completed": { "type": "boolean", "description": "Filter by completion status. Omit to list all tasks." }
                }
            }
        }),
        json!({
            "name": "task_get",
            "description": "Get a single ducker task by id, with actions and children.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "id": { "type": "string", "description": "Task id" }
                },
                "required": ["id"]
            }
        }),
        json!({
            "name": "task_create",
            "description": "Create a ducker task. Set periodic_interval to make it a periodic task (0=OnStart, 1=Daily, 7=Weekly, 30=Monthly, 100=OnceStarted). Dates accept 'YYYY-MM-DD HH:MM:SS' or RFC3339 in local time.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "name": { "type": "string", "description": "Task name" },
                    "value": { "type": "number", "description": "Task value weight, default 0" },
                    "due_to": { "type": "string", "description": "Due date, e.g. '2026-09-03 18:00:00'. Default: 12 hours from now." },
                    "reminder": { "type": "string", "description": "Reminder time, same format as due_to" },
                    "parent_id": { "type": "string", "description": "Parent task id for subtasks" },
                    "auto": { "type": "boolean", "description": "Whether the task auto-executes its actions when due" },
                    "action_ids": { "type": "array", "items": { "type": "string" }, "description": "Action ids to attach" },
                    "periodic_interval": { "type": "integer", "description": "Make periodic: 0=OnStart, 1=Daily, 7=Weekly, 30=Monthly, 100=OnceStarted" },
                    "periodic_name": { "type": "string", "description": "Display name of the periodic rule; defaults to task name" }
                },
                "required": ["name"]
            }
        }),
        json!({
            "name": "task_update",
            "description": "Update a ducker task. Fields left out keep their current value. Use task_complete to change completion status (it also advances periodic tasks).",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "id": { "type": "string", "description": "Task id" },
                    "name": { "type": "string" },
                    "value": { "type": "number" },
                    "due_to": { "type": "string" },
                    "reminder": { "type": "string", "description": "Pass null to clear the reminder" },
                    "parent_id": { "type": "string", "description": "Pass null to detach from parent" },
                    "auto": { "type": "boolean" },
                    "action_ids": { "type": "array", "items": { "type": "string" } }
                },
                "required": ["id"]
            }
        }),
        json!({
            "name": "task_complete",
            "description": "Mark a ducker task completed (or uncomplete). Completing a periodic task automatically creates its next instance.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "id": { "type": "string", "description": "Task id" },
                    "completed": { "type": "boolean", "description": "Target status, default true" }
                },
                "required": ["id"]
            }
        }),
        json!({
            "name": "task_delete",
            "description": "Delete a ducker task. If it belongs to a periodic rule, the rule is deleted too.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "id": { "type": "string", "description": "Task id" }
                },
                "required": ["id"]
            }
        }),
        json!({
            "name": "periodic_list",
            "description": "List all periodic task rules of ducker, each including its current task instance.",
            "inputSchema": { "type": "object", "properties": {} }
        }),
        json!({
            "name": "action_list",
            "description": "List all ducker actions (reusable task actions: open url/directory/file, run command, notice, group).",
            "inputSchema": { "type": "object", "properties": {} }
        }),
        json!({
            "name": "action_get",
            "description": "Get a single ducker action by id.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "id": { "type": "string", "description": "Action id" }
                },
                "required": ["id"]
            }
        }),
        json!({
            "name": "action_create",
            "description": "Create a ducker action. type is one of: url, directory, file, command, notice, group. For notice, command is the title and args are body lines. For group, args are member action ids.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "name": { "type": "string" },
                    "type": { "type": "string", "description": "url | directory | file | command | notice | group" },
                    "command": { "type": "string", "description": "Target: url/path/program, or notice title" },
                    "args": { "type": "array", "items": { "type": "string" }, "description": "Arguments (command args, notice body lines, or group member ids)" },
                    "desc": { "type": "string" },
                    "wait": { "type": "integer", "description": "Milliseconds to wait after execution; wait > 0 makes the action synchronous (sequencer waits for completion), default 0 = async" },
                    "retry": { "type": "integer" },
                    "timeout": { "type": "integer", "description": "Timeout in seconds" }
                },
                "required": ["name", "type", "command"]
            }
        }),
        json!({
            "name": "action_delete",
            "description": "Delete a ducker action by id.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "id": { "type": "string", "description": "Action id" }
                },
                "required": ["id"]
            }
        }),
        json!({
            "name": "action_execute",
            "description": "Execute a ducker action by id. url/directory/file open with the system default handler; command runs and returns stdout; group executes its members. Notice types are skipped in the headless MCP process. The action's usage count is incremented on success.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "id": { "type": "string", "description": "Action id" }
                },
                "required": ["id"]
            }
        }),
    ]
}

pub async fn call(db: &Arc<Database>, name: &str, args: &Value) -> Result<Value, String> {
    match name {
        "task_list" => task_list(db, args),
        "task_get" => task_get(db, args),
        "task_create" => task_create(db, args),
        "task_update" => task_update(db, args),
        "task_complete" => task_complete(db, args),
        "task_delete" => task_delete(db, args),
        "periodic_list" => periodic_list(db),
        "action_list" => action_list(db),
        "action_get" => action_get(db, args),
        "action_create" => action_create(db, args),
        "action_delete" => action_delete(db, args),
        "action_execute" => {
            super::exec::execute_action_by_id(Arc::clone(db), &require_str(args, "id")?).await
        }
        other => Err(format!("Unknown tool: {other}")),
    }
}

// ---------- 参数提取小工具 ----------

fn arg_str<'a>(args: &'a Value, key: &str) -> Option<&'a str> {
    args.get(key).and_then(Value::as_str)
}
fn arg_owned(args: &Value, key: &str) -> Option<String> {
    arg_str(args, key).map(str::to_owned)
}
fn arg_bool(args: &Value, key: &str) -> Option<bool> {
    args.get(key).and_then(Value::as_bool)
}
fn arg_f64(args: &Value, key: &str) -> Option<f64> {
    args.get(key).and_then(Value::as_f64)
}
fn arg_u8(args: &Value, key: &str) -> Option<u8> {
    args.get(key).and_then(Value::as_u64).map(|v| v as u8)
}
fn arg_usize(args: &Value, key: &str) -> Option<usize> {
    args.get(key).and_then(Value::as_u64).map(|v| v as usize)
}
fn arg_u64(args: &Value, key: &str) -> Option<u64> {
    args.get(key).and_then(Value::as_u64)
}
fn arg_str_vec(args: &Value, key: &str) -> Option<Vec<String>> {
    args.get(key).and_then(Value::as_array).map(|arr| {
        arr.iter()
            .filter_map(|v| v.as_str().map(str::to_owned))
            .collect()
    })
}
fn require_str(args: &Value, key: &str) -> Result<String, String> {
    arg_owned(args, key).ok_or_else(|| format!("missing required argument: {key}"))
}
fn to_json<T: serde::Serialize>(value: &T) -> Result<Value, String> {
    serde_json::to_value(value).map_err(|e| format!("serialize error: {e}"))
}

// ---------- task domain ----------

fn task_list(db: &Database, args: &Value) -> Result<Value, String> {
    let records = db.get_all_tasks().map_err(|e| e.to_string())?;
    let mut views = Vec::new();
    for record in &records {
        if let Some(completed) = arg_bool(args, "completed") {
            if record.completed != completed {
                continue;
            }
        }
        views.push(task_view_from_record(record, db).map_err(|e| e.to_string())?);
    }
    to_json(&views)
}

fn task_get(db: &Database, args: &Value) -> Result<Value, String> {
    let id = require_str(args, "id")?;
    let record = db.get_task(&id).map_err(|e| e.to_string())?;
    let view = task_view_from_record(&record, db).map_err(|e| e.to_string())?;
    to_json(&view)
}

fn validate_interval(interval: u8) -> Result<(), String> {
    match interval {
        0 | 1 | 7 | 30 | 100 => Ok(()),
        other => Err(format!(
            "invalid periodic_interval {other}, expected one of 0(OnStart)/1(Daily)/7(Weekly)/30(Monthly)/100(OnceStarted)"
        )),
    }
}

fn task_create(db: &Database, args: &Value) -> Result<Value, String> {
    let task = TaskData {
        id: None,
        name: require_str(args, "name")?,
        value: arg_f64(args, "value"),
        completed: false,
        auto: arg_bool(args, "auto").unwrap_or(false),
        parent_id: arg_owned(args, "parent_id"),
        periodic: None,
        actions: arg_str_vec(args, "action_ids").unwrap_or_default(),
        created_at: None,
        due_to: arg_owned(args, "due_to"),
        reminder: arg_owned(args, "reminder"),
    };

    if let Some(interval) = arg_u8(args, "periodic_interval") {
        validate_interval(interval)?;
        let data = PeriodicTaskData {
            name: arg_owned(args, "periodic_name").unwrap_or_else(|| task.name.clone()),
            interval,
            task,
        };
        let record = db.create_periodic_task(&data).map_err(|e| e.to_string())?;
        return Ok(json!({
            "id": record.id,
            "name": record.name,
            // 0=OnStart 1=Daily 7=Weekly 30=Monthly 100=OnceStarted
            "interval": record.interval,
            "last_period": record.last_period,
            "next_period": record.next_period
        }));
    }
    let record = db.create_task(&task).map_err(|e| e.to_string())?;
    to_json(&record)
}

fn task_update(db: &Database, args: &Value) -> Result<Value, String> {
    let id = require_str(args, "id")?;
    let record = db.get_task(&id).map_err(|e| e.to_string())?;

    let data = TaskData {
        id: Some(id.clone()),
        name: arg_owned(args, "name").unwrap_or(record.name.clone()),
        value: arg_f64(args, "value").or(Some(record.value)),
        completed: record.completed,
        auto: arg_bool(args, "auto").unwrap_or(record.auto),
        // 显式传 null 清除，缺省保留原值
        parent_id: args
            .get("parent_id")
            .map(|v| v.as_str().map(str::to_owned))
            .unwrap_or(record.parent_id.clone()),
        periodic: record.periodic.clone(),
        actions: arg_str_vec(args, "action_ids").unwrap_or(record.actions.clone()),
        created_at: Some(to_datetime_str(record.created_at)),
        due_to: arg_owned(args, "due_to").or(Some(to_datetime_str(record.due_to))),
        reminder: args
            .get("reminder")
            .map(|v| v.as_str().map(str::to_owned))
            .unwrap_or(record.reminder.map(to_datetime_str)),
    };

    let updated = db.update_task(&id, &data).map_err(|e| e.to_string())?;
    to_json(&updated)
}

fn task_complete(db: &Database, args: &Value) -> Result<Value, String> {
    let id = require_str(args, "id")?;
    let completed = arg_bool(args, "completed").unwrap_or(true);
    db.update_task_status(&id, completed)
        .map_err(|e| e.to_string())?;
    Ok(json!({ "id": id, "completed": completed }))
}

fn task_delete(db: &Database, args: &Value) -> Result<Value, String> {
    let id = require_str(args, "id")?;
    db.delete_task(&id).map_err(|e| e.to_string())?;
    Ok(json!({ "id": id, "deleted": true }))
}

fn periodic_list(db: &Database) -> Result<Value, String> {
    let records = db.get_enabled_periodic_tasks().map_err(|e| e.to_string())?;
    let mut result = Vec::new();
    for record in &records {
        match PeriodicTask::build(record, db) {
            Ok(task) => result.push(to_json(&task)?),
            Err(e) => result.push(json!({ "id": record.id, "error": e.to_string() })),
        }
    }
    Ok(Value::Array(result))
}

// ---------- action domain ----------

fn action_list(db: &Database) -> Result<Value, String> {
    let records = db.get_all_actions().map_err(|e| e.to_string())?;
    let views: Vec<Action> = records.into_iter().map(Action::from).collect();
    to_json(&views)
}

fn action_get(db: &Database, args: &Value) -> Result<Value, String> {
    let id = require_str(args, "id")?;
    let record = db.get_action(&id).map_err(|e| e.to_string())?;
    to_json(&Action::from(record))
}

fn action_create(db: &Database, args: &Value) -> Result<Value, String> {
    let action = Action {
        id: None,
        name: require_str(args, "name")?,
        desc: arg_owned(args, "desc").unwrap_or_default(),
        wait: arg_usize(args, "wait").unwrap_or(0),
        typ: require_str(args, "type")?,
        retry: arg_usize(args, "retry"),
        timeout: arg_u64(args, "timeout"),
        command: require_str(args, "command")?,
        args: arg_str_vec(args, "args"),
        count: None,
    };
    let record = db.create_action(&action).map_err(|e| e.to_string())?;
    to_json(&Action::from(record))
}

fn action_delete(db: &Database, args: &Value) -> Result<Value, String> {
    let id = require_str(args, "id")?;
    db.delete_action(&id).map_err(|e| e.to_string())?;
    Ok(json!({ "id": id, "deleted": true }))
}
