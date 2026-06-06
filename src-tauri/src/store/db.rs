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
                timeout INTEGER,
                count INTEGER DEFAULT 0
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
        let current_next_period = res
            .next_period
            .ok_or_else(|| anyhow!("周期性任务缺少 next_period: {}", current_periodic_task_id))?;
        let period = res.interval;

        let current_due_to = current_periodic_task.due_to as u64;
        let expected_from_current = calculate_next_period(current_periodic_task.due_to, period) as u64;

        // 允许两种合法状态：
        // 1. 普通状态：current_task.due_to + interval == current_next_period
        // 2. 特殊状态：current_task.due_to == current_next_period（过期补期或月度日期锚点调整后）
        let (mut created_due_to, expected_normal_due) = if expected_from_current == current_next_period {
            (current_next_period as i64, current_next_period as i64)
        } else if current_due_to == current_next_period {
            (
                calculate_next_period(current_next_period as i64, period),
                calculate_next_period(current_next_period as i64, period),
            )
        } else {
            return Err(anyhow!("周期性任务已过期，无法创建下一个周期任务"));
        };

        // 月度任务保留 last_period 的日期锚点。
        // 例如 1/30 -> 2/28，在完成 2/28 时根据 1/30 推导出 3/30。
        if period == 30 && current_due_to == current_next_period {
            if let Some(last_period) = current_last_period {
                use chrono::{Datelike, TimeZone};
                let last_dt = Local.timestamp_opt(last_period as i64, 0).single().unwrap();
                let next_dt = Local.timestamp_opt(current_next_period as i64, 0).single().unwrap();
                if last_dt.day() != next_dt.day() {
                    logging!(
                        debug,
                        Type::Database,
                        "月度任务日期锚点生效: last_period={} ({}日), next_period={} ({}日)",
                        last_period,
                        last_dt.day(),
                        current_next_period,
                        next_dt.day()
                    );
                    let after_next_month = next_month(next_month(last_dt));
                    created_due_to = after_next_month.timestamp();
                }
            }
        }

        // 如果原本应创建的时间点已经过去，则直接跳到未来最近的一次。
        let future_due_to = calculate_next_period_from_now(created_due_to, period);
        if future_due_to > created_due_to {
            created_due_to = future_due_to;
        }

        let next_task = TaskData{
            id: format!("task{}", random_string(6)).into(),
            completed: false,
            parent_id: current_periodic_task.parent_id.clone(),
            name: current_periodic_task.name.clone(),
            auto: current_periodic_task.auto,
            actions: current_periodic_task.actions.clone(),
            created_at: to_datetime_str(current_periodic_task.created_at).into(),
            due_to: to_datetime_str(created_due_to).into(),
            reminder: current_periodic_task.reminder.map(|reminder| to_datetime_str(created_due_to - (current_periodic_task.due_to - reminder)).into()),
            value: current_periodic_task.value.into(),
            periodic: current_periodic_task_id.clone().into(),
        };
        // 创建下一个周期任务实体
        self.create_task(&next_task)?;
        // 更新周期性任务记录
        let synced_period = if created_due_to == expected_normal_due {
            None
        } else {
            Some(created_due_to)
        };
        self.update_periodic_task_last_period(&current_periodic_task_id, synced_period)?;
        let upcoming_period = calculate_next_period(created_due_to, period);
        Ok(PeriodicTaskRecord {
            id: current_periodic_task_id,
            name: next_task.name.clone(),   
            interval: period,
            last_period: Some(created_due_to as u64),
            next_period: Some(upcoming_period as u64),
        })
    }
    
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::store::module::{PeriodicTaskManager, TaskManager};
    use chrono::{Duration, TimeZone};
    use std::{fs, path::PathBuf};

    struct TestDb {
        path: PathBuf,
        db: Database,
    }

    impl TestDb {
        fn new() -> Self {
            let unique = format!(
                "ducker-periodic-test-{}",
                Local::now()
                    .timestamp_nanos_opt()
                    .unwrap_or_else(|| Local::now().timestamp_micros() * 1000)
            );
            let path = std::env::temp_dir().join(unique);
            fs::create_dir_all(&path).unwrap();
            let db = Database::new(path.clone()).unwrap();
            Self { path, db }
        }
    }

    impl Drop for TestDb {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.path);
        }
    }

    fn local_ts(year: i32, month: u32, day: u32, hour: u32, minute: u32, second: u32) -> i64 {
        Local
            .with_ymd_and_hms(year, month, day, hour, minute, second)
            .unwrap()
            .timestamp()
    }

    fn build_task(id: &str, periodic_id: &str, name: &str, due_to: i64) -> TaskData {
        TaskData {
            id: Some(id.to_string()),
            name: name.to_string(),
            value: Some(0.0),
            completed: false,
            auto: false,
            parent_id: None,
            periodic: Some(periodic_id.to_string()),
            actions: vec![],
            created_at: Some(to_datetime_str(due_to - 3600)),
            due_to: Some(to_datetime_str(due_to)),
            reminder: None,
        }
    }

    fn insert_periodic_rule(
        db: &Database,
        id: &str,
        name: &str,
        interval: u8,
        last_period: Option<i64>,
        next_period: Option<i64>,
    ) {
        db.conn
            .write()
            .execute(
                "INSERT INTO periodic_tasks (id, name, interval, last_period, next_period)
                 VALUES (?1, ?2, ?3, ?4, ?5)",
                rusqlite::params![id, name, interval, last_period, next_period],
            )
            .unwrap();
    }

    #[test]
    fn create_next_periodic_task_keeps_monthly_day_anchor() {
        let test_db = TestDb::new();
        let db = &test_db.db;
        let periodic_id = "periodic-monthly-anchor";

        let january_anchor = local_ts(2025, 1, 30, 9, 0, 0);
        let february_task_due = local_ts(2025, 2, 28, 9, 0, 0);
        let expected_next_due = local_ts(2025, 3, 30, 9, 0, 0);

        db.create_task(&build_task(
            "task-feb-28",
            periodic_id,
            "收尾",
            february_task_due,
        ))
        .unwrap();
        insert_periodic_rule(
            db,
            periodic_id,
            "收尾",
            30,
            Some(january_anchor),
            Some(february_task_due),
        );

        let current_task = db.get_task("task-feb-28").unwrap();
        db.create_next_periodic_task(&current_task).unwrap();

        let periodic = db.get_periodic_task(periodic_id).unwrap();
        assert_eq!(periodic.last_period, Some(january_anchor as u64));
        assert_eq!(periodic.next_period, Some(expected_next_due as u64));

        let tasks = db.get_all_tasks().unwrap();
        assert!(tasks.iter().any(|task| {
            task.id != "task-feb-28"
                && task.periodic.as_deref() == Some(periodic_id)
                && task.due_to == expected_next_due
        }));
    }

    #[test]
    fn get_enabled_periodic_tasks_returns_all_tasks() {
        let test_db = TestDb::new();
        let db = &test_db.db;

        insert_periodic_rule(db, "p-startup", "启动任务", 0, None, None);
        insert_periodic_rule(db, "p-daily", "日常任务", 1, None, None);
        insert_periodic_rule(db, "p-weekly", "周任务", 7, None, None);

        let enabled = db.get_enabled_periodic_tasks().unwrap();
        assert_eq!(enabled.len(), 3);

        let names: Vec<&str> = enabled.iter().map(|t| t.name.as_str()).collect();
        assert!(names.contains(&"启动任务"));
        assert!(names.contains(&"日常任务"));
        assert!(names.contains(&"周任务"));
    }

    #[test]
    fn get_startup_periodic_tasks_filters_by_interval() {
        let test_db = TestDb::new();
        let db = &test_db.db;

        insert_periodic_rule(db, "p-startup", "启动任务", 0, None, None);
        insert_periodic_rule(db, "p-boot", "开机任务", 100, None, None);
        insert_periodic_rule(db, "p-daily", "日常任务", 1, None, None);
        insert_periodic_rule(db, "p-weekly", "周任务", 7, None, None);

        let startup = db.get_startup_periodic_tasks().unwrap();
        assert_eq!(startup.len(), 2);

        let names: Vec<&str> = startup.iter().map(|t| t.name.as_str()).collect();
        assert!(names.contains(&"启动任务"));
        assert!(names.contains(&"开机任务"));
        assert!(!names.contains(&"日常任务"));
        assert!(!names.contains(&"周任务"));
    }

    #[test]
    fn get_periodic_task_returns_correct_record() {
        let test_db = TestDb::new();
        let db = &test_db.db;

        let last = local_ts(2025, 6, 1, 0, 0, 0);
        let next = local_ts(2025, 6, 2, 0, 0, 0);
        insert_periodic_rule(db, "p-exact", "精确查询", 1, Some(last), Some(next));

        let record = db.get_periodic_task("p-exact").unwrap();
        assert_eq!(record.name, "精确查询");
        assert_eq!(record.interval, 1);
        assert_eq!(record.last_period, Some(last as u64));
        assert_eq!(record.next_period, Some(next as u64));

        let missing = db.get_periodic_task("nonexistent");
        assert!(missing.is_err());
    }

    #[test]
    fn delete_periodic_task_removes_record() {
        let test_db = TestDb::new();
        let db = &test_db.db;

        insert_periodic_rule(db, "p-delete", "待删除", 1, None, None);
        assert!(db.get_periodic_task("p-delete").is_ok());

        db.delete_periodic_task("p-delete").unwrap();
        assert!(db.get_periodic_task("p-delete").is_err());

        let err = db.delete_periodic_task("nonexistent");
        assert!(err.is_err());
    }

    #[test]
    fn update_periodic_task_changes_name_and_interval() {
        let test_db = TestDb::new();
        let db = &test_db.db;

        insert_periodic_rule(db, "p-update", "旧名称", 1, None, None);

        let periodic_data = crate::schema::PeriodicTaskData {
            task: TaskData {
                id: Some("p-update".to_string()),
                name: "新名称".to_string(),
                value: Some(0.0),
                completed: false,
                auto: false,
                parent_id: None,
                periodic: Some("p-update".to_string()),
                actions: vec![],
                created_at: None,
                due_to: None,
                reminder: None,
            },
            name: "新名称".to_string(),
            interval: 7,
        };

        let updated = db.update_periodic_task("p-update", &periodic_data).unwrap();
        assert_eq!(updated.name, "新名称");
        assert_eq!(updated.interval, 7);

        let stored = db.get_periodic_task("p-update").unwrap();
        assert_eq!(stored.name, "新名称");
        assert_eq!(stored.interval, 7);
    }

    #[test]
    fn create_next_periodic_task_catches_up_overdue_daily_task() {
        let test_db = TestDb::new();
        let db = &test_db.db;
        let periodic_id = "periodic-daily-overdue";

        let now = Local::now().timestamp();
        let overdue_due = now - Duration::hours(25).num_seconds();
        let previous_due = overdue_due - Duration::days(1).num_seconds();
        let expected_next_due =
            calculate_next_period_from_now(calculate_next_period(overdue_due, 1), 1);

        db.create_task(&build_task(
            "task-overdue",
            periodic_id,
            "收尾",
            overdue_due,
        ))
        .unwrap();
        insert_periodic_rule(
            db,
            periodic_id,
            "收尾",
            1,
            Some(previous_due),
            Some(overdue_due),
        );

        let current_task = db.get_task("task-overdue").unwrap();
        db.create_next_periodic_task(&current_task).unwrap();

        let periodic = db.get_periodic_task(periodic_id).unwrap();
        assert_eq!(periodic.next_period, Some(expected_next_due as u64));

        let tasks = db.get_all_tasks().unwrap();
        assert!(tasks.iter().any(|task| {
            task.id != "task-overdue"
                && task.periodic.as_deref() == Some(periodic_id)
                && task.due_to == expected_next_due
        }));
    }
}
