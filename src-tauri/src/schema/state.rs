use crate::store::db::Database;
use crate::{logging, utils::logging::Type};
use std::collections::HashSet;
use std::sync::{Arc, OnceLock};
use parking_lot::{Mutex,Once};
#[derive(Clone)]
pub struct AppState {
    pub db: Arc<Mutex<Database>>,
    pub lightweight: Arc<Mutex<LightWeightState>>,
}

#[derive(Clone)]
pub struct LightWeightState {
    #[allow(unused)]
    once: Arc<Once>,
    pub is_lightweight: bool,
    pub close_listeners: Vec<u32>,
    pub focus_listeners: Vec<u32>,
    pub listened_windows: HashSet<String>,
}

impl LightWeightState {
    pub fn new() -> Self {
        Self {
            once: Arc::new(Once::new()),
            is_lightweight: false,
            close_listeners: Vec::new(),
            focus_listeners: Vec::new(),
            listened_windows: HashSet::new(),
        }
    }

    #[allow(unused)]
    pub fn run_once_time<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        self.once.call_once(f);
    }

    pub fn set_lightweight_mode(&mut self, value: bool) -> &Self {
        self.is_lightweight = value;
        if value {
            logging!(info, Type::Lightweight, "轻量模式已开启");
        } else {
            logging!(info, Type::Lightweight, "轻量模式已关闭");
        }
        self
    }
}

impl Default for LightWeightState {
    fn default() -> Self {
        static INSTANCE: OnceLock<LightWeightState> = OnceLock::new();
        INSTANCE.get_or_init(LightWeightState::new).clone()
    }
}
