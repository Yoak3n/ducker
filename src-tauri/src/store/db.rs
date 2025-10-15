use anyhow::{Result,anyhow};
use chrono::Local;
use parking_lot::RwLock;
use rusqlite::Connection;
use tauri::Emitter;
use std::path::PathBuf;

use super::module::*;
use crate::{
    logging,get_app_handle,
    schema::{
        PeriodicTaskRecord, TaskData,
        TaskRecord,
    },
    utils::{
        date::{
            calculate_next_period, 
            calculate_next_period_from_now, 
            next_month, to_datetime_str
        }, 
        help::random_string, 
        logging::Type
    },
};
pub struct Database {
    pub conn: RwLock<Connection>,
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
                periodic TEXT,
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

    pub fn build_task_record_from_row(row: &rusqlite::Row) -> rusqlite::Result<TaskRecord> {
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
            periodic: row.get(10)?,
        })
    }

    pub fn build_periodic_task_record_from_row(
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

    pub fn on_task_completed(&self, task_id: &str) -> Result<()> {
        // 更新任务为已完成
        logging!(info, Type::Database, true, "调用任务 {} 完成后回调", task_id);
        let task = self.get_task(task_id)?;
        // 如果是周期性任务，创建下一个周期
        if let Some(_) = &task.periodic {
            match self.create_next_periodic_task(&task) {
                Ok(_) => {
                    logging!(info, Type::Database, true, "创建下一个周期性任务成功");
                    // 刷新任务列表，会不会有种不该在这里的感觉？
                    let app_handle = get_app_handle!();
                    app_handle.emit("task-changed", None::<()>)?;
                },
                Err(e) => {
                    logging!(error, Type::Database, true, "创建下一个周期性任务失败: {:?}", e);
                }
            }
        }else {
            logging!(info, Type::Database, true, "任务 {} 不是周期性任务，无需创建下一个周期", task_id);
        }
        Ok(())
    }

    pub fn create_next_periodic_task(&self, current_periodic_task: &TaskRecord) -> Result<PeriodicTaskRecord> {
        let current_periodic_task_id = current_periodic_task.clone().periodic.unwrap();
        // 获取当前的 next_period 和 interval
        let res = self.get_periodic_task(&current_periodic_task_id)?;
        let current_last_period = res.last_period;
        let current_next_period = res.next_period;
        let interval = res.interval;

        // 计算新的 next_period
        let mut calculated_next_period = calculate_next_period(current_periodic_task.due_to, interval);
        // 用来标记是否需要特殊处理，如日期天数不一致
        let mut special_flag = false;
        // 如果计算出的时间与当前 next_period 相同，意味着需要创建下一个周期任务，否则意味着是已过期的周期任务,阻止创建下一个任务
        let next_task = if Some(calculated_next_period as u64) == current_next_period {
                // 只对月度任务进行日期天数一致性检查
                if interval == 30 {
                    // 检查current_last_period, current_next_period的日期的天数是否一致 
                    if let (Some(last_period), Some(next_period)) = (current_last_period, current_next_period) {
                        use chrono::{Datelike, TimeZone};
                        let last_dt = Local.timestamp_opt(last_period as i64, 0).single().unwrap();
                        let next_dt = Local.timestamp_opt(next_period as i64, 0).single().unwrap();
                        // 检查两个日期的天数是否一致
                        if last_dt.day() != next_dt.day() {        
                            // 如果天数不一致，说明可能是月末日期调整（如1月31日->2月28日），则使用last_period的日期推到下下月
                            special_flag = true;
                            // 需要重新计算下一个周期，确保使用正确的目标日期
                            logging!(
                                debug,
                                Type::Database,
                                "月度任务日期不一致: last_period={} ({}日), next_period={} ({}日)",
                                last_period,
                                last_dt.day(),
                                next_period,
                                next_dt.day()
                            );
                            let after_next_month = next_month(next_month(last_dt));
                            calculated_next_period = after_next_month.timestamp();
                        }
                    }
                }
                // 再检查是否早于当前时间，重新计算
                let next_period = calculate_next_period_from_now(calculated_next_period, interval);
                calculated_next_period = next_period;
                TaskData{
                    id: format!("task{}", random_string(6)).into(),
                    completed: false,
                    parent_id: current_periodic_task.parent_id.clone(),
                    name: current_periodic_task.name.clone(),
                    auto: current_periodic_task.auto,
                    actions: current_periodic_task.actions.clone(),
                    created_at: to_datetime_str(current_periodic_task.created_at).into(),
                    due_to: to_datetime_str(next_period).into(),
                    reminder: current_periodic_task.reminder.map(|reminder| to_datetime_str(next_period - (current_periodic_task.due_to - reminder)).into()),
                    value: current_periodic_task.value.into(),
                    periodic: current_periodic_task_id.clone().into(),
                }
            } else {
                return Err(anyhow!("周期性任务已过期，无法创建下一个周期任务"));
            };
        // 创建下一个周期任务实体
        self.create_task(&next_task)?;
        // 更新周期性任务记录
        self.update_periodic_task_last_period(&current_periodic_task_id, if special_flag {Some(calculated_next_period)}else{None})?;
        Ok(PeriodicTaskRecord {
            id: current_periodic_task_id,
            name: next_task.name.clone(),   
            interval,
            last_period: Some(calculated_next_period as u64),
            next_period: Some(calculated_next_period as u64),
        })
    }
    
}
