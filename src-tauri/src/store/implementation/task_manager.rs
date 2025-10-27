use anyhow::Result;
use rusqlite::params;

use crate::{
    schema::{TaskData, TaskRecord},
    store::{db::Database, module::{TaskManager, PeriodicTaskManager}},
    utils::logging::Type,
    logging
};

impl TaskManager for Database {
    fn create_task(&self, task: &TaskData) -> Result<TaskRecord> {
        let conn = self.conn.write();
        let actions = serde_json::to_string(&task.actions)?;
        let record = TaskRecord::from(task.clone());
        let mut stmt = conn.prepare(
            "
        INSERT INTO tasks (id, value, auto, parent_id, periodic, name, actions, created_at, due_to, reminder) 
        VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
        )?;

        match stmt.insert(params![
            &record.id,
            &record.value,
            &record.auto,
            &record.parent_id,
            &record.periodic,
            &record.name,
            &actions,
            &record.created_at,
            &record.due_to,
            &record.reminder
        ]) {
            Ok(id) => {
                logging!(info, Type::Database,true, "创建任务成功: {id}");
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
            SET name = ?1, value = ?2, actions = ?3, due_to = ?4, reminder = ?5, completed = ?6, auto = ?7, parent_id = ?8, periodic = ?9
            WHERE id = ?10",
            params![
                &task.name,
                &task.value,
                &actions,
                &record.due_to,
                &record.reminder,
                &record.completed,
                &record.auto,
                &record.parent_id,
                &record.periodic,
                id],
        )?;
        Ok(record)
    }

    fn update_task_status(&self, id: &str, completed: bool) -> Result<bool> {
        {
            let conn = self.conn.write();
            conn.execute(
                "UPDATE tasks 
                SET completed = ?1
                WHERE id = ?2",
                params![&completed, id],
            )?;
        }
        if completed {
            logging!(info, Type::Database, true, "任务 {} 已完成", id);
            self.on_task_completed(id)?;
        }
        Ok(true)
    }

    fn delete_task(&self, id: &str) -> Result<()> {
        // 首先检查任务是否存在periodic字段
        let task = self.get_task(id)?;
        
        // 如果任务有periodic字段，先删除对应的周期规则
        if let Some(periodic_id) = &task.periodic {
            // 使用PeriodicTaskManager trait来删除周期规则
            if let Err(e) = self.delete_periodic_task(periodic_id) {
                logging!(
                    warn,
                    Type::Database,
                    "删除周期规则失败，但继续删除任务: periodic_id={}, error={}",
                    periodic_id,
                    e
                );
            } else {
                logging!(
                    info,
                    Type::Database,
                    "成功删除任务关联的周期规则: periodic_id={}",
                    periodic_id
                );
            }
        }
        
        // 删除任务本身
        let conn = self.conn.write();
        conn.execute("DELETE FROM tasks WHERE id = ?1", [id])?;
        
        logging!(info, Type::Database, "成功删除任务: id={}", id);
        Ok(())
    }

    fn get_task(&self, id: &str) -> Result<TaskRecord> {
        let conn = self.conn.read();
        let mut stmt = conn.prepare(
            "
        SELECT id, completed, parent_id, name, auto, actions, created_at, due_to, reminder, value, periodic 
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
            "SELECT id, completed, parent_id, name, auto, actions, created_at, due_to, reminder, value, periodic 
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
            "SELECT id, completed, parent_id, name, auto, actions, created_at, due_to, reminder, value, periodic 
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
        SELECT id, completed, parent_id, name, auto, actions, created_at, due_to, reminder, value, periodic 
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
            "SELECT id, completed, parent_id, name, auto, actions, created_at, due_to, reminder, value, periodic 
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
            "SELECT id, completed, parent_id, name, auto, actions, created_at, due_to, reminder, value, periodic 
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
            "SELECT id, completed, parent_id, name, auto, actions, created_at, due_to, reminder, value, periodic 
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