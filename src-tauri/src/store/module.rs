use crate::schema::{Action, ActionRecord, PeriodicTaskData, PeriodicTaskRecord, TaskData, TaskRecord};
use anyhow::Result;
pub trait ActionManager {
    fn create_action(&self, action: &Action) -> Result<ActionRecord>;
    fn update_action(&self, id: &str, action: &Action) -> Result<ActionRecord>;
    fn delete_action(&self, id: &str) -> Result<()>;
    fn get_action(&self, id: &str) -> Result<ActionRecord>;
    fn get_actions(&self, ids: &[String]) -> Result<Vec<ActionRecord>>;
    fn get_all_actions(&self) -> Result<Vec<ActionRecord>>;
}
pub trait TaskManager {
    fn create_task(&self, task: &TaskData) -> Result<TaskRecord>;
    fn update_task(&self, id: &str, task: &TaskData) -> Result<TaskRecord>;
    fn update_task_status(&self, id: &str, completed: bool) -> Result<bool>;
    fn delete_task(&self, id: &str) -> Result<()>;
    fn get_task(&self, id: &str) -> Result<TaskRecord>;
    fn get_tasks(&self, ids: &[String]) -> Result<Vec<TaskRecord>>;
    fn get_tasks_by_status(&self, completed: bool) -> Result<Vec<TaskRecord>>;
    fn get_tasks_by_parent_id(&self, parent_id: &str) -> Result<Vec<TaskRecord>>;
    fn get_tasks_by_date_range(&self, start_date: i64, end_date: i64) -> Result<Vec<TaskRecord>>;
    fn get_uncompleted_tasks_by_date_range(
        &self,
        start_date: i64,
        end_date: i64,
    ) -> Result<Vec<TaskRecord>>;
    fn get_all_tasks(&self) -> Result<Vec<TaskRecord>>;
}

pub trait PeriodicTaskManager {
    fn create_periodic_task(&self, task: &PeriodicTaskData) -> Result<PeriodicTaskRecord>;
    fn update_periodic_task(&self, id: &str, task: &PeriodicTaskData) -> Result<PeriodicTaskRecord>;
    fn update_periodic_task_last_period(&self, id: &str, next_period: Option<i64>) -> Result<()>;
    fn update_periodic_tasks_last_run(&self, ids: &[String]) -> Result<()>;
    fn delete_periodic_task(&self, id: &str) -> Result<()>;
    fn get_enabled_periodic_tasks(&self) -> Result<Vec<PeriodicTaskRecord>>;
    fn get_startup_periodic_tasks(&self) -> Result<Vec<PeriodicTaskRecord>>;
    fn get_periodic_task(&self, id: &str) -> Result<PeriodicTaskRecord>;
}