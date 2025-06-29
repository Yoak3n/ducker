use crate::schema::{Action, ActionRecord, TaskData, TaskRecord};
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
    fn get_tasks_by_date_range(&self, start_date: &str, end_date: &str) -> Result<Vec<TaskRecord>>;
    fn get_all_tasks(&self) -> Result<Vec<TaskRecord>>;
}
