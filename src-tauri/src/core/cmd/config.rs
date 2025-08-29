use crate::config::Config;

#[tauri::command]
pub async fn save_config() {
    let config = Config::global().lock().unwrap().clone();
    config.save().ok();
}

#[tauri::command]
pub async fn get_config() -> Config {
    let c = Config::global().lock().unwrap().data().clone();
    c
}

#[tauri::command]
pub async fn update_config(config: Config) -> Config {
    let original = Config::global();
    let mut config_guard = original.lock().unwrap();
    config_guard.patch_config(config.clone());
    config_guard.save().ok();
    println!("update config: {:?}", config_guard);
    config
}
