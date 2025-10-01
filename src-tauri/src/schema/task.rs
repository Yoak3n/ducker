use crate::{
    schema::AppState,
    store::module::{ActionManager, TaskManager},
    utils::{
        date::{str_to_datetime, to_datetime_str},
        help::random_string,
    },
};
use super::Action;
use chrono::{Duration, Local};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct TaskRecord {
    pub id: String,
    pub value: f64,
    pub completed: bool,
    pub auto: bool,
    pub parent_id: Option<String>,
    pub periodic: Option<String>,
    pub name: String,
    pub actions: Vec<String>,
    pub created_at: i64,
    pub due_to: i64,
    pub reminder: Option<i64>,
}

impl TryFrom<(&TaskRecord, &AppState)> for TaskView {
    type Error = anyhow::Error;

    fn try_from((record, state): (&TaskRecord, &AppState)) -> Result<Self, Self::Error> {
        let db = state.db.lock().unwrap();

        // 获取关联的 actions
        let action_records = db.get_actions(&record.actions)?;
        let actions = Some(
            action_records
                .into_iter()
                .map(|record| Action::from(record))
                .collect(),
        );
        // println!("{:?}", actions);

        let children = db
            .get_tasks_by_parent_id(&record.id)?
            .into_iter()
            .map(|child| Self::try_from((&child, state)))
            .collect::<Result<Vec<_>, _>>()?;

        let created_at = to_datetime_str(record.created_at);
        let due_to = Some(to_datetime_str(record.due_to));
        let reminder = record.reminder.map(to_datetime_str);

        Ok(Self {
            id: record.id.clone(),
            value: record.value,
            name: record.name.clone(),
            completed: record.completed,
            auto: record.auto,
            periodic: record.periodic.clone(),
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
        // 随机生成id
        let id = match &data.id {
            Some(id) => id.clone(),
            None => format!("task{}", random_string(6)),
        };
        let now = Local::now();
        let created_at = if let Some(s) = data.created_at {
            str_to_datetime(s.as_str()).timestamp()
        } else {
            now.timestamp()
        };
        // 截止时间默认为三个小时后
        let due_to = if let Some(s) = data.due_to {
            str_to_datetime(s.as_str()).timestamp()
        } else {
            (now + Duration::hours(12)).timestamp()
        };
        Self {
            id,
            // TODO 添加更复杂的默认值逻辑,比如根据任务所有来判定任务价值
            value: data.value.unwrap_or(0.0),
            completed: data.completed,
            auto: data.auto,
            parent_id: data.parent_id,
            periodic: data.periodic,
            name: data.name,
            actions: data.actions,
            created_at,
            due_to,
            reminder: data
                .reminder
                .map(|s| str_to_datetime(s.as_str()).timestamp()),
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct TaskData {
    pub id: Option<String>,
    pub name: String,
    pub value: Option<f64>,
    pub completed: bool,
    pub auto: bool,
    pub parent_id: Option<String>,
    pub periodic: Option<String>,
    pub actions: Vec<String>,
    pub created_at: Option<String>,
    pub due_to: Option<String>,
    pub reminder: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct TaskView {
    pub id: String,
    pub name: String,
    pub value: f64,
    pub completed: bool,
    pub auto: bool,
    pub periodic: Option<String>,
    pub actions: Option<Vec<Action>>,
    pub children: Option<Vec<TaskView>>,
    pub created_at: String,
    pub due_to: Option<String>,
    pub reminder: Option<String>,
}
