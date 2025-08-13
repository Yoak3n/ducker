use super::schedule::create_scheduled_tasks;
use crate::{logging, schema::Action, singleton, utils::logging::Type};
use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

pub struct Hub {
    pub schedule: Arc<RwLock<HashMap<i64, Vec<Action>>>>,
}
singleton!(Hub, INSTANCE);

impl Hub {
    pub fn new() -> Self {
        let hub = Hub {
            schedule: Arc::new(RwLock::new(create_scheduled_tasks())),
        };
        hub
    }

    pub fn latest_schedule(&self) -> Option<HashMap<i64, Vec<Action>>> {
        let schedule = self.schedule.read().unwrap();
        if schedule.is_empty() {
            None
        } else {
            Some(schedule.clone())
        }
    }

    pub async fn refresh(&self) {
        let schedule = create_scheduled_tasks();
        let mut schedule_map = self.schedule.write().unwrap();
        *schedule_map = schedule;

        logging!(info, Type::Service, true, "Schedule refreshed");
    }
}
