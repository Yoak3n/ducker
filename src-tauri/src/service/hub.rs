use super::schedule::create_scheduled_tasks;
use crate::{schema::TaskView, singleton};
use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

pub struct Hub {
    pub schedule: Arc<RwLock<HashMap<i64, Vec<TaskView>>>>,
}
singleton!(Hub, INSTANCE);

impl Hub {
    pub fn new() -> Self {
        let hub = Hub {
            schedule: Arc::new(RwLock::new(create_scheduled_tasks())),
        };
        hub
    }

    pub fn latest_schedule(&self) -> Option<HashMap<i64, Vec<TaskView>>> {
        let schedule = self.schedule.read().unwrap();
        if schedule.is_empty() {
            None
        } else {
            Some(schedule.clone())
        }
    }

    pub fn get_schedule(&self, id: &str, ts: i64) -> Option<Vec<TaskView>> {
        let schedule = self.schedule.read().unwrap();
        schedule.get(&ts).map_or(None, |v| {
            let tasks = v.iter().filter(|t| t.id == id).cloned().collect();
            Some(tasks)
        })
    }

    pub async fn refresh(&self) {
        let schedule = create_scheduled_tasks();
        let mut schedule_map = self.schedule.write().unwrap();
        *schedule_map = schedule;
    }
}
