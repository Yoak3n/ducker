use std::collections::HashMap;

use crate::{
    schema::{Action, ActionRecord, ActionType},
    store::{db::Database, module::ActionManager},
    utils::help::random_string,
};
use anyhow::Result;

impl ActionManager for Database {
    fn create_action(&self, action: &Action) -> Result<ActionRecord> {
        let conn = self.conn.write();
        let mut args_text = String::new();
        if let Some(args) = &action.args {
            args_text = args.join(",");
        }
        let action_id = format!("act{}", random_string(6));
        // let data =ActionData::from_action(&action);
        let data = action.clone();
        let typ: ActionType = ActionType::try_from(data.typ.as_str())?;

        conn.execute(
            "INSERT INTO actions (id, name, desc, command, args, type, wait, retry, timeout)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
            (
                &action_id,
                &action.name,
                &action.desc,
                &action.command,
                &args_text,
                &(typ.clone() as i8),
                data.wait,
                data.retry,
                data.timeout,
            ),
        )?;
        let record = ActionRecord {
            id: action_id.clone(),
            name: data.name,
            desc: data.desc,
            wait: data.wait,
            command: data.command,
            args: args_text.clone(),
            typ,
            retry: data.retry,
            timeout: data.timeout,
        };
        Ok(record)
    }

    fn update_action(&self, id: &str, action: &Action) -> Result<ActionRecord> {
        let conn = self.conn.write();
        let mut args_text = String::new();
        if let Some(args) = &action.args {
            args_text = args.join(",");
        }
        conn.execute(
            "UPDATE actions SET name = ?1, desc = ?2, command = ?3, args = ?4, type = ?5,wait = ?6, retry = ?7, timeout =?8
            WHERE id = ?9",
            (
                &action.name,
                &action.desc,
                &action.command,
                &args_text,
                (ActionType::try_from(action.typ.as_str())? as u8),
                &action.wait,
                &action.retry,
                &action.timeout,
                id
            ))?;
        let record = ActionRecord {
            id: id.to_string(),
            name: action.name.clone(),
            desc: action.desc.clone(),
            wait: action.wait,
            command: action.command.clone(),
            args: args_text.clone(),
            typ: ActionType::try_from(action.typ.as_str())?,
            retry: action.retry,
            timeout: action.timeout,
        };
        Ok(record)
    }

    fn delete_action(&self, id: &str) -> Result<()> {
        let conn = self.conn.write();
        conn.execute("DELETE FROM actions WHERE id = ?1", [id])?;
        Ok(())
    }

    fn get_action(&self, id: &str) -> Result<ActionRecord> {
        let conn = self.conn.read();
        let mut stmt = conn.prepare(
            "SELECT 
            id, name, desc, command, args, type, wait, retry, timeout 
            FROM actions WHERE id = ?1",
        )?;
        let action = stmt.query_row([id], |row| {
            let id = row.get(0)?;
            let name = row.get(1)?;
            let desc = row.get(2)?;
            let command = row.get(3)?;
            let args_text: String = row.get(4)?;
            let typ_number: u8 = row.get(5)?;
            let typ = ActionType::try_from(typ_number).unwrap_or(ActionType::Command);
            let wait = row.get(6)?;
            let retry: Option<usize> = row.get(7)?;
            let timeout: Option<u64> = row.get(8)?;
            Ok(ActionRecord {
                id,
                name,
                desc,
                wait,
                command,
                args: args_text,
                typ,
                retry,
                timeout,
            })
        })?;
        Ok(action)
    }

    fn get_actions(&self, ids: &[String]) -> Result<Vec<ActionRecord>> {
        if ids.is_empty() {
            return Ok(Vec::new());
        }
        let conn = self.conn.read();
        let placeholders = ids.iter().map(|_| "?").collect::<Vec<_>>().join(",");
        let query = format!(
            "SELECT 
            id, name, desc, command, args, type, wait, retry, timeout
            FROM actions WHERE id IN ({})",
            placeholders
        );

        let mut stmt = conn.prepare(&query)?;
        let params = ids
            .iter()
            .map(|id| id as &dyn rusqlite::ToSql)
            .collect::<Vec<_>>();

        let action_iter = stmt.query_map(params.as_slice(), |row| {
            let id = row.get(0)?;
            let name = row.get(1)?;
            let desc = row.get(2)?;
            let command = row.get(3)?;
            let args_text: String = row.get(4)?;
            let typ_number: u8 = row.get(5)?;
            let typ = ActionType::try_from(typ_number).unwrap_or(ActionType::Command);
            let wait = row.get(6)?;
            let retry: Option<usize> = row.get(7)?;
            let timeout: Option<u64> = row.get(8)?;
            Ok(ActionRecord {
                id,
                name,
                desc,
                command,
                args: args_text,
                typ,
                wait,
                retry,
                timeout,
            })
        })?;

        // 将查询结果存储为map，以id为key
        let mut action_map = HashMap::new();
        for action in action_iter {
            let action = action?;
            action_map.insert(action.id.clone(), action);
        }

        // 根据ids的顺序返回对应的actions数组
        let mut actions = Vec::with_capacity(ids.len());
        for id in ids {
            if let Some(action) = action_map.remove(id) {
                actions.push(action);
            }
        }

        Ok(actions)
    }

    fn get_all_actions(&self) -> anyhow::Result<Vec<ActionRecord>> {
        let conn = self.conn.read();
        let mut stmt = conn.prepare(
            "SELECT 
            id, name, desc, command, args, type, wait, retry, timeout 
            FROM actions",
        )?;

        let action_iter = stmt.query_map([], |row| {
            let id = row.get(0)?;
            let name = row.get(1)?;
            let desc = row.get(2)?;
            let command = row.get(3)?;
            let args_text: String = row.get(4)?;
            let typ_number: u8 = row.get(5)?;
            let typ = ActionType::try_from(typ_number).unwrap_or(ActionType::Command);
            let wait = row.get(6)?;
            let retry: Option<usize> = row.get(7)?;
            let timeout: Option<u64> = row.get(8)?;
            Ok(ActionRecord {
                id,
                name,
                desc,
                command,
                args: args_text,
                typ,
                wait,
                retry,
                timeout,
            })
        })?;

        let mut actions = Vec::new();
        for action in action_iter {
            actions.push(action?);
        }
        Ok(actions)
    }
}
