use crate::{schema::AppState, store::module::ActionManager, store::module::TaskManager};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct TaskRecord {
    pub id: String,
    pub completed: bool,
    pub parent_id: Option<String>,
    pub name: String,
    pub desc: String,
    pub actions: Vec<String>,
    pub created_at: String,
    pub start_at: Option<String>,
}
impl TaskRecord {
    pub fn into_view(self, actions: Vec<Action>, children: Vec<TaskView>) -> TaskView {
        TaskView {
            id: self.id,
            name: self.name,
            completed: self.completed,
            desc: self.desc,
            actions: Some(actions),
            children: Some(children),
            created_at: self.created_at,
            start_at: self.start_at,
        }
    }
}
impl From<TaskRecord> for TaskView {
    fn from(record: TaskRecord) -> Self {
        Self {
            id: record.id,
            name: record.name,
            completed: record.completed,
            desc: record.desc,
            actions: None,
            children: None,
            created_at: record.created_at,
            start_at: record.start_at,
        }
    }
}

impl TryFrom<(TaskRecord, &AppState)> for TaskView {
    type Error = anyhow::Error;

    fn try_from((record, state): (TaskRecord, &AppState)) -> Result<Self, Self::Error> {
        // 获取关联的 actions
        let action_records = state.db.get_actions(&record.actions)?;
        let actions = Some(
            action_records
                .into_iter()
                .map(|record| Action::from(record))
                .collect(),
        );

        let children = state
            .db
            .get_tasks_by_parent_id(&record.id)?
            .into_iter()
            .map(|child| Self::try_from((child, state)))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self {
            id: record.id,
            name: record.name,
            completed: record.completed,
            desc: record.desc,
            actions,
            children: Some(children),
            created_at: record.created_at,
            start_at: record.start_at,
        })
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct TaskData {
    pub id: String,
    pub completed: bool,
    pub parent_id: Option<String>,
    pub name: String,
    pub desc: String,
    pub actions: Vec<String>,
    pub start_at: Option<String>,
}
use super::Action;
#[derive(Deserialize, Serialize, Debug)]
pub struct TaskView {
    pub id: String,
    pub name: String,
    pub completed: bool,
    pub desc: String,
    pub actions: Option<Vec<Action>>,
    pub children: Option<Vec<TaskView>>,
    pub created_at: String,
    pub start_at: Option<String>,
}

impl TaskView {
    pub fn from_record(record: TaskRecord, actions: Vec<Action>, children: Vec<TaskView>) -> Self {
        record.into_view(actions, children)
    }
}
