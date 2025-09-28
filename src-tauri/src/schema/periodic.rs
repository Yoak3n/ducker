use serde::{Deserialize, Serialize};

use crate::{schema::TaskData, store::module::TaskManager};
use super::{TaskView,AppState};

#[derive(Deserialize, Serialize, Debug)]
pub struct  PeriodicTask {
    pub id: Option<String>,
    pub name: String,
    pub interval: Period,
    pub task: TaskView,
    pub last_run: Option<u64>,
}

impl TryFrom<(&PeriodicTaskRecord,&AppState)> for PeriodicTask {
    type Error = anyhow::Error;
    
    fn try_from((record, state): (&PeriodicTaskRecord, &AppState)) -> Result<Self, Self::Error> {
        // 获取关联的 task
        let task_record = {
            let db = state.db.lock().unwrap();
            db.get_task(&record.id)?
        };
        let task = TaskView::try_from((&task_record, state))?;
        let interval = match record.interval {
            0 => Period::OnStart,
            1 => Period::Daily,
            7 => Period::Weekly,
            30 => Period::Monthly,
            100 => Period::OnceStarted,
            _ => return Err(anyhow::anyhow!("Invalid interval")),
        };

        Ok(Self {
            id: Some(record.id.clone()),
            name: record.name.clone(),
            interval,
            task,
            last_run: record.last_run,
        })
    }
}


#[derive(Deserialize, Serialize, Clone,Debug)]
#[repr(u8)]
pub enum Period {
    OnStart = 0,
    Daily = 1,
    Weekly = 7,
    Monthly = 30,
    OnceStarted = 100,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct PeriodicTaskData {
    pub name: String,
    pub interval: Period,
    pub task: TaskData,
}


pub struct PeriodicTaskRecord {
    pub id: String,
    pub name: String,
    pub interval: u8,
    pub last_run: Option<u64>,
}