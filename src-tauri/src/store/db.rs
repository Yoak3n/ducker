use anyhow::Result;
use parking_lot::RwLock;
use rusqlite::{params, Connection};
use std::collections::VecDeque;
use std::path::PathBuf;

use super::module::*;
use crate::{
    logging,
    schema::{
        Action, ActionRecord, ActionType, PeriodicTaskData, PeriodicTaskRecord, TaskData,
        TaskRecord,
    },
    utils::{help::random_string, logging::Type},
};
pub struct Database {
    conn: RwLock<Connection>,
}
unsafe impl Send for Database {}
unsafe impl Sync for Database {}
impl Database {
    pub fn new(db_path: PathBuf) -> Result<Self> {
        let conn = Connection::open(db_path.join("ducker.db"))?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS tasks (
                id TEXT PRIMARY KEY,
                value REAL DEFAULT 0,
                completed INTEGER DEFAULT 0,
                auto INTEGER DEFAULT 0,
                parent_id TEXT,
                name TEXT NOT NULL,
                actions TEXT,
                created_at INTEGER,
                due_to INTEGER,
                reminder INTEGER
            )",
            [],
        )?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS actions (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL UNIQUE,
                desc TEXT,
                command TEXT NOT NULL,
                args TEXT,
                type INTEGER NOT NULL,
                wait INTEGER NOT NULL DEFAULT 0,
                retry INTEGER  DEFAULT 0,
                timeout INTEGER
            )",
            [],
        )?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS periodic_tasks (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                interval INTEGER NOT NULL,
                last_period INTEGER,
                next_period INTEGER
            )",
            [],
        )?;

        // 创建索引以提升查询性能
        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_tasks_completed ON tasks(completed)",
            [],
        )?;
        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_tasks_parent_id ON tasks(parent_id)",
            [],
        )?;
        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_tasks_created_at ON tasks(created_at)",
            [],
        )?;
        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_actions_name ON actions(name)",
            [],
        )?;
        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_periodic_tasks_last_period ON periodic_tasks(last_period)",
            [],
        )?;

        Ok(Self {
            conn: RwLock::new(conn),
        })
    }

    pub fn none() -> Self {
        let conn = Connection::open_in_memory().unwrap();
        Self {
            conn: RwLock::new(conn),
        }
    }

    fn build_task_record_from_row(row: &rusqlite::Row) -> rusqlite::Result<TaskRecord> {
        let actions_json: String = row.get(5)?;
        let actions: Vec<String> = serde_json::from_str(&actions_json).unwrap_or_else(|e| {
            logging!(warn, Type::Database, "JSON反序列化失败，使用默认值: {e}");
            Vec::new()
        });

        Ok(TaskRecord {
            id: row.get(0)?,
            completed: row.get(1)?,
            parent_id: row.get(2)?,
            name: row.get(3)?,
            auto: row.get(4)?,
            actions,
            created_at: row.get(6)?,
            due_to: row.get(7)?,
            reminder: row.get(8)?,
            value: row.get(9)?,
        })
    }

    fn build_periodic_task_record_from_row(
        row: &rusqlite::Row,
    ) -> rusqlite::Result<PeriodicTaskRecord> {
        Ok(PeriodicTaskRecord {
            id: row.get(0)?,
            name: row.get(1)?,
            interval: row.get(2)?,
            last_period: row.get(3)?,
            next_period: row.get(4)?,
        })
    }
}

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

        let mut actions = VecDeque::new();
        for action in action_iter {
            actions.push_back(action?);
        }
        Ok(actions.into())
    }

    fn get_all_actions(&self) -> anyhow::Result<Vec<crate::schema::ActionRecord>> {
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

impl TaskManager for Database {
    fn create_task(&self, task: &TaskData) -> Result<TaskRecord> {
        let conn = self.conn.write();
        let actions = serde_json::to_string(&task.actions)?;
        let record = TaskRecord::from(task.clone());
        let mut stmt = conn.prepare(
            "
        INSERT INTO tasks (id, value, auto, parent_id, name, actions, created_at, due_to, reminder) 
        VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9  )",
        )?;

        match stmt.insert(params![
            &record.id,
            &record.value,
            &record.auto,
            &record.parent_id,
            &record.name,
            &actions,
            &record.created_at,
            &record.due_to,
            &record.reminder
        ]) {
            Ok(id) => {
                logging!(info, Type::Database, "创建任务成功: {id}");
            }
            Err(e) => {
                logging!(
                    error,
                    Type::Database,
                    true,
                    "创建任务失败: {:?}, 错误: {:?}",
                    record.id,
                    e
                );
                return Err(anyhow::anyhow!("创建任务失败"));
            }
        };

        Ok(record)
    }

    fn update_task(&self, id: &str, task: &TaskData) -> Result<TaskRecord> {
        let conn = self.conn.write();
        let actions = serde_json::to_string(&task.actions)?;
        let record = TaskRecord::from(task.clone());
        conn.execute(
            "UPDATE tasks 
            SET name = ?1, value = ?2, actions = ?3, due_to = ?4, reminder = ?5, completed = ?6, auto = ?7, parent_id = ?8
            WHERE id = ?9",
            params![
                &task.name,
                &task.value,
                &actions,
                &record.due_to,
                &record.reminder,
                &record.completed,
                &record.auto,
                &record.parent_id,
                id],
        )?;
        Ok(record)
    }

    fn update_task_status(&self, id: &str, completed: bool) -> Result<bool> {
        let conn = self.conn.write();
        conn.execute(
            "UPDATE tasks 
            SET completed = ?1
            WHERE id = ?2",
            params![&completed, id],
        )?;
        Ok(true)
    }

    fn delete_task(&self, id: &str) -> Result<()> {
        let conn = self.conn.write();
        conn.execute("DELETE FROM tasks WHERE id = ?1", [id])?;
        Ok(())
    }

    fn get_task(&self, id: &str) -> Result<TaskRecord> {
        let conn = self.conn.read();
        let mut stmt = conn.prepare(
            "
        SELECT id, completed, parent_id, name, auto, actions, created_at, due_to, reminder, value 
        FROM tasks WHERE id = ?1",
        )?;
        let task = stmt.query_row([id], |row| Self::build_task_record_from_row(row))?;
        Ok(task)
    }

    fn get_tasks(&self, ids: &[String]) -> Result<Vec<TaskRecord>> {
        if ids.is_empty() {
            return Ok(Vec::new());
        }
        let conn = self.conn.read();
        let placeholders = ids.iter().map(|_| "?").collect::<Vec<_>>().join(",");
        let query = format!(
            "SELECT id, completed, parent_id, name, auto, actions, created_at, due_to, reminder, value 
            FROM tasks 
            WHERE id IN ({})",
            placeholders
        );

        let mut stmt = conn.prepare(&query)?;
        let params = ids
            .iter()
            .map(|id| id as &dyn rusqlite::ToSql)
            .collect::<Vec<_>>();

        let task_iter = stmt.query_map(params.as_slice(), |row| {
            Self::build_task_record_from_row(row)
        })?;
        let mut tasks = Vec::new();

        for task_result in task_iter {
            if let Ok(task) = task_result {
                tasks.push(task);
            }
        }

        Ok(tasks)
    }

    fn get_tasks_by_status(&self, completed: bool) -> Result<Vec<TaskRecord>> {
        let conn = self.conn.read();
        let mut stmt = conn.prepare(
            "SELECT id, completed, parent_id, name, auto, actions, created_at, due_to, reminder, value 
            FROM tasks WHERE completed = ?1",
        )?;
        let tasks = stmt.query_map([completed], |row| Self::build_task_record_from_row(row))?;

        let mut result = Vec::new();
        for task in tasks {
            result.push(task?);
        }
        Ok(result)
    }

    fn get_tasks_by_parent_id(&self, parent_id: &str) -> Result<Vec<TaskRecord>> {
        let conn = self.conn.read();
        let mut stmt = conn.prepare(
            "
        SELECT id, completed, parent_id, name, auto, actions, created_at, due_to, reminder, value 
        FROM tasks 
        WHERE parent_id = ?1",
        )?;
        let tasks = stmt.query_map([parent_id], |row| Self::build_task_record_from_row(row))?;

        let mut result = Vec::new();
        for task in tasks {
            result.push(task?);
        }
        Ok(result)
    }

    fn get_tasks_by_date_range(&self, start_date: i64, end_date: i64) -> Result<Vec<TaskRecord>> {
        let conn = self.conn.read();
        let mut stmt = conn.prepare(
            "SELECT id, completed, parent_id, name, auto, actions, created_at, due_to, reminder, value 
            FROM tasks 
            WHERE due_to BETWEEN ?1 AND ?2
            ORDER BY due_to DESC",
        )?;
        let tasks = stmt.query_map([start_date, end_date], |row| {
            Self::build_task_record_from_row(row)
        })?;

        let mut result = Vec::new();
        for task in tasks {
            result.push(task?);
        }
        Ok(result)
    }

    fn get_uncompleted_tasks_by_date_range(
        &self,
        start_date: i64,
        end_date: i64,
    ) -> Result<Vec<TaskRecord>> {
        let conn = self.conn.read();
        let mut stmt = conn.prepare(
            "SELECT id, completed, parent_id, name, auto, actions, created_at, due_to, reminder, value 
            FROM tasks 
            WHERE (due_to BETWEEN ?1 AND ?2) AND completed = 0
            ORDER BY due_to DESC",
        )?;
        let tasks = stmt.query_map([start_date, end_date], |row| {
            Self::build_task_record_from_row(row)
        })?;

        let mut result = Vec::new();
        for task in tasks {
            result.push(task?);
        }
        Ok(result)
    }

    fn get_all_tasks(&self) -> Result<Vec<TaskRecord>> {
        let conn = self.conn.read();
        let mut stmt = conn.prepare(
            "SELECT id, completed, parent_id, name, auto, actions, created_at, due_to, reminder, value 
            FROM tasks",
        )?;
        let tasks = stmt.query_map([], |row| Self::build_task_record_from_row(row))?;

        let mut result = Vec::new();
        for task in tasks {
            result.push(task?);
        }
        Ok(result)
    }
}

impl PeriodicTaskManager for Database {
    fn create_periodic_task(&self, task: &PeriodicTaskData) -> Result<PeriodicTaskRecord> {
        // 创建任务
        let task_record = self.create_task(&task.task)?;
        let conn = self.conn.write();
        let mut stmt = conn.prepare(
            "INSERT INTO periodic_tasks (id, name, interval, next_period) 
             VALUES (?1, ?2, ?3, ?4)",
        )?;
        let next_period = task.interval.clone() as i64 * 3600 * 24 + &task_record.due_to;
        let row_id = stmt.insert(params![
            &task_record.id,
            &task.name,
            task.interval.clone() as u8,
            Some(next_period),
        ])?;

        logging!(debug, Type::Database, "创建周期性任务成功: {}", row_id);

        Ok(PeriodicTaskRecord {
            id: task_record.id.clone(),
            name: task.name.clone(),
            interval: task.interval.clone() as u8,
            last_period: None,
            next_period: Some(next_period as u64),
        })
    }

    fn update_periodic_task(
        &self,
        id: &str,
        task: &PeriodicTaskData,
    ) -> Result<PeriodicTaskRecord> {
        let conn = self.conn.write();
        // 更新任务
        let task_record = self.update_task(id, &task.task)?;

        conn.execute(
            "UPDATE periodic_tasks 
             SET name = ?1, interval = ?2
             WHERE id = ?3",
            params![&task_record.name, task.interval.clone() as u8, id],
        )?;

        logging!(debug, Type::Database, "更新周期性任务成功: {}", id);

        Ok(PeriodicTaskRecord {
            id: id.to_string(),
            name: task.name.clone(),
            interval: task.interval.clone() as u8,
            last_period: None,
            next_period: None,
        })
    }

    fn update_periodic_task_last_period(&self, id: &str) -> Result<()> {
        let conn = self.conn.write();

        // 获取当前的 next_period 和 interval
        let (current_next_period, interval): (Option<i64>, i64) = conn.query_row(
            "SELECT next_period, interval FROM periodic_tasks WHERE id = ?1",
            params![id],
            |row| Ok((row.get(0)?, row.get(1)?)),
        )?;

        // 计算新的 next_period
        let new_last_period = current_next_period.unwrap_or_else(|| {
            let task_due_to: i64 = conn
                .query_row(
                    "SELECT due_to FROM tasks WHERE id = ?1",
                    params![id],
                    |row| Ok(row.get(0)?),
                )
                .unwrap();
            task_due_to
        });
        let new_next_period = new_last_period + interval * 3600 * 24;

        conn.execute(
            "UPDATE periodic_tasks 
             SET last_period = ?1, next_period = ?2
             WHERE id = ?3",
            params![new_last_period, new_next_period, id],
        )?;

        logging!(
            debug,
            Type::Database,
            "更新周期性任务时间成功: {} (last_period: {}, next_period: {})",
            id,
            new_last_period,
            new_next_period
        );

        Ok(())
    }

    fn update_periodic_tasks_last_run(&self, ids: &[String]) -> Result<()> {
        // 专为`启动时任务`而设，将当前时间记录下来
        if ids.is_empty() {
            return Ok(());
        }
        let conn = self.conn.write();
        let now = chrono::Utc::now().timestamp();
        let placeholders = ids.iter().map(|_| "?").collect::<Vec<_>>().join(",");
        let query = format!(
            "UPDATE periodic_tasks 
             SET last_period = ?1
             WHERE id IN ({})",
            placeholders
        );

        let mut params: Vec<&dyn rusqlite::ToSql> = vec![&now as &dyn rusqlite::ToSql];
        for id in ids {
            params.push(id as &dyn rusqlite::ToSql);
        }

        conn.execute(&query, params.as_slice())?;
        logging!(
            debug,
            Type::Database,
            "批量更新周期性任务最后运行时间成功，共 {} 个任务",
            ids.len()
        );
        Ok(())
    }

    fn delete_periodic_task(&self, id: &str) -> Result<()> {
        let conn = self.conn.write();

        let rows_affected =
            conn.execute("DELETE FROM periodic_tasks WHERE id = ?1", params![id])?;

        if rows_affected == 0 {
            return Err(anyhow::anyhow!("周期性任务不存在: {}", id));
        }

        logging!(info, Type::Database, "删除周期性任务成功: {}", id);
        Ok(())
    }

    fn get_enabled_periodic_tasks(&self) -> Result<Vec<PeriodicTaskRecord>> {
        let conn = self.conn.read();
        let mut stmt = conn.prepare(
            "SELECT id, name, interval, last_period, next_period 
             FROM periodic_tasks ",
        )?;

        let periodic_tasks =
            stmt.query_map([], |row| Self::build_periodic_task_record_from_row(row))?;

        let mut result = Vec::new();
        for task in periodic_tasks {
            result.push(task?);
        }

        Ok(result)
    }
}
