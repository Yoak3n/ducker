use crate::config::Config;

#[tauri::command]
pub async fn save_config() {
    let config = Config::global().lock().clone();
    config.save().ok();
}

#[tauri::command]
pub async fn get_config() -> Config {
    let c = Config::global().lock().data().clone();
    c
}

#[tauri::command]
pub async fn update_config(config: Config) -> Config {
    let original = Config::global();
    let mut config_guard = original.lock();
    config_guard.patch_config(config.clone());
    config_guard.save().ok();
    config
}
