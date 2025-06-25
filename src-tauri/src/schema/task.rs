use crate::{
    schema::AppState, 
    store::module::{ActionManager, TaskManager},
    utils::{
        date::{to_datetime_str,str_to_datetime}, 
        help::random_string
    },
};
use serde::{Deserialize, Serialize};


#[derive(Deserialize, Serialize, Debug)]
pub struct TaskRecord {
    pub id: String,
    pub value: f64,
    pub completed: bool,
    pub auto: bool,
    pub parent_id: Option<String>,
    pub name: String,
    pub actions: Vec<String>,
    pub created_at: i64,
    pub due_to: Option<i64>,
    pub reminder: Option<i64>,
}
impl TaskRecord {
    pub fn into_view(self, actions: Vec<Action>, children: Vec<TaskView>) -> TaskView {
        TaskView {
            id: self.id,
            name: self.name,
            value: self.value,
            completed: self.completed,
            auto: self.auto,
            actions: Some(actions),
            children: Some(children),
            created_at: to_datetime_str(self.created_at),
            due_to: self.due_to.map(to_datetime_str),
            reminder: self.reminder.map(to_datetime_str),
        }
    }
}
impl From<TaskRecord> for TaskView {
    fn from(record: TaskRecord) -> Self {
        Self {
            id: record.id,
            value: record.value,
            name: record.name,
            completed: record.completed,
            auto: record.auto,
            actions: None,
            children: None,
            created_at: to_datetime_str(record.created_at),
            due_to: record.due_to.map(to_datetime_str),
            reminder: record.reminder.map(to_datetime_str),
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
        
        let created_at = to_datetime_str(record.created_at);
        let due_to = record.due_to.map(to_datetime_str);
        let reminder = record.reminder.map(to_datetime_str);


        Ok(Self {
            id: record.id,
            value: record.value,
            name: record.name,
            completed: record.completed,
            auto: record.auto,
            actions,
            children: Some(children),
            created_at,
            due_to,
            reminder,
        })
    }
}


impl From<TaskData> for TaskRecord {
    fn from(data: TaskData) -> Self {
        let id = match &data.id {
            Some(id) => id.clone(),
            None => format!("task{}", random_string(6)),
        };
        let created_at = if let Some(s) = data.created_at {
            str_to_datetime(s.as_str()).timestamp()
        } else {
            chrono::Local::now().timestamp()
        };

        Self {
            id,
            value: data.value,
            completed: data.completed,
            auto: data.auto,
            parent_id: data.parent_id,
            name: data.name,
            actions: data.actions,
            created_at,
            due_to:data.due_to.map(|s| str_to_datetime(s.as_str()).timestamp()),
            reminder: data.reminder.map(|s| str_to_datetime(s.as_str()).timestamp()),
        }
    }
}



#[derive(Deserialize, Serialize, Debug,Clone)]
pub struct TaskData {
    pub id: Option<String>,
    pub name: String,
    pub value: f64,
    pub completed: bool,
    pub auto: bool,
    pub parent_id: Option<String>,
    pub actions: Vec<String>,
    pub created_at: Option<String>,
    pub due_to: Option<String>,
    pub reminder: Option<String>,
}
use super::Action;
#[derive(Deserialize, Serialize, Debug)]
pub struct TaskView {
    pub id: String,
    pub name: String,
    pub value: f64,
    pub completed: bool,
    pub auto: bool,
    pub actions: Option<Vec<Action>>,
    pub children: Option<Vec<TaskView>>,
    pub created_at: String,
    pub due_to: Option<String>,
    pub reminder: Option<String>,
}

impl TaskView {
    pub fn from_record(record: TaskRecord, actions: Vec<Action>, children: Vec<TaskView>) -> Self {
        record.into_view(actions, children)
    }
}
