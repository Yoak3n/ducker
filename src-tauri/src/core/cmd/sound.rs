use crate::utils::sound;


#[tauri::command]
pub async fn play_sound(name: &str) -> Result<(), String> {
    sound::play_sound(name).await.map_err(|e| e.to_string())
}