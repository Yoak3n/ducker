// use std::{fmt, path::PathBuf, sync::Arc};
// use tokio::sync::Mutex;
// use once_cell::sync::OnceCell;

// #[derive(Debug)]
// pub struct CoreManager {
//     // running: Arc<Mutex<RunningMode>>,
//     // child_sidecar: Arc<Mutex<Option<CommandChild>>>,
// }

// impl CoreManager {
//     pub fn global() -> &'static CoreManager {
//         static CORE_MANAGER: OnceCell<CoreManager> = OnceCell::new();
//         CORE_MANAGER.get_or_init(|| CoreManager {})
//     }

// }
