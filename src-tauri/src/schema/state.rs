use std::sync::Arc;

use crate::store::db::Database;
#[derive(Clone)]
pub struct AppState {
    pub db: Arc<Database>,
}
