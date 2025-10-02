use anyhow::Result;
use rusqlite::params;

use crate::{
    schema::{PeriodicTaskData, PeriodicTaskRecord, TaskRecord},
    store::{db::Database, module::PeriodicTaskManager},
    utils::{
        logging::Type,
        date::calculate_next_period
    },
    logging
};


impl PeriodicTaskManager for Database {
    fn create_periodic_task(&self, task: &PeriodicTaskData) -> Result<PeriodicTaskRecord> {
        // 直接创建带有periodic字段的任务，避免先创建再更新的冗余操作
        let conn = self.conn.write();
        let actions = serde_json::to_string(&task.task.actions)?;
        let task_record = TaskRecord::from(task.task.clone());
        
        // 在创建时就设置periodic字段为任务ID
        let mut stmt = conn.prepare(
            "INSERT INTO tasks (id, value, auto, parent_id, periodic, name, actions, created_at, due_to, reminder) 
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
        )?;
        // 防止检查定时任务时，启动时任务也被当作自动任务，导致期望外的执行
        let auto = if task.interval == 0 || task.interval == 100 {
            false
        } else {
            true
        };

        match stmt.insert(params![
            &task_record.id,
            &task_record.value,
            auto,
            &task_record.parent_id,
            Some(&task_record.id), // 直接设置periodic为任务ID
            &task_record.name,
            &actions,
            &task_record.created_at,
            &task_record.due_to,
            &task_record.reminder
        ]) {
            Ok(id) => {
                logging!(info, Type::Database, "创建周期性任务成功: {id}");
            }
            Err(e) => {
                logging!(
                    error,
                    Type::Database,
                    true,
                    "创建周期性任务失败: {:?}, 错误: {:?}",
                    task_record.id,
                    e
                );
                return Err(anyhow::anyhow!("创建周期性任务失败"));
            }
        };
        
        // 继续使用同一个连接创建periodic_tasks记录
        let mut stmt = conn.prepare(
            "INSERT INTO periodic_tasks (id, name, interval, next_period, last_period) 
             VALUES (?1, ?2, ?3, ?4, ?5)",
        )?;
        let next_period = calculate_next_period(task_record.due_to, task.interval as u8);
        let row_id = stmt.insert(params![
            &task_record.id,
            &task.name,
            task.interval.clone() as u8,
            Some(next_period),
            Some(task_record.due_to),
        ])?;

        logging!(debug, Type::Database, "创建周期性任务成功: {}", row_id);

        Ok(PeriodicTaskRecord {
            id: task_record.id.clone(),
            name: task.name.clone(),
            interval: task.interval.clone() as u8,
            last_period: Some(task_record.due_to as u64),
            next_period: Some(next_period as u64),
        })
    }

    // 什么时候调用才会让系统正常运行呢？
    // 当一个周期性任务被触发时，系统会调用这个函数来更新下一个周期任务的触发时间。
    fn update_periodic_task_last_period(&self, id: &str, next_period: Option<i64>) -> Result<()> {
        let conn = self.conn.write();

        // 获取当前的 next_period 和 interval
        let (current_next_period,current_last_period, interval): (Option<i64>, Option<i64>, u8) = conn.query_row(
            "SELECT next_period, last_period, interval FROM periodic_tasks WHERE id = ?1",
            params![id],
            |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)),
        )?;
        
        // 计算新的 next_period
        let new_next_period;
        let new_last_period = if let Some(n) = next_period {
            // 如果传入新的 next_period,表示是特殊情况： 进行了日期自适应调整 —— 不更新last_period，只更新next_period
            new_next_period = n;
            current_last_period.unwrap()
        } else {
            new_next_period = calculate_next_period(current_next_period.unwrap(), interval);
            current_next_period.unwrap()
        };

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

    fn get_periodic_task(&self, id: &str) -> Result<PeriodicTaskRecord> {
        let conn = self.conn.read();
        let mut stmt = conn.prepare(
            "SELECT id, name, interval, last_period, next_period 
             FROM periodic_tasks WHERE id = ?1",
        )?;

        let mut rows =
            stmt.query_map([id], |row| Self::build_periodic_task_record_from_row(row))?;

        match rows.next() {
            Some(task) => Ok(task?),
            None => Err(anyhow::anyhow!("周期性任务不存在: {}", id)),
        }
    }

    fn update_periodic_task(
        &self,
        periodic_id: &str,
        task: &PeriodicTaskData,
    ) -> Result<PeriodicTaskRecord> {
        let current_periodic_task = self.get_periodic_task(periodic_id)?;
        let conn = self.conn.write();
        let query = "UPDATE periodic_tasks 
                     SET name = ?1, interval = ?2, last_period = ?3, next_period = ?4
                     WHERE id = ?5";
        conn.execute(
            query,
            params![
                task.name.as_str(),
                task.interval,
                current_periodic_task.last_period,  
                current_periodic_task.next_period,
                periodic_id
            ],
        )?;
        Ok(PeriodicTaskRecord {
            id: periodic_id.to_string(),
            name: task.name.clone(),
            interval: task.interval,
            last_period: current_periodic_task.last_period,
            next_period: current_periodic_task.next_period,
        })
    }
}
