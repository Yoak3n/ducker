use crate::utils::sound;


#[tauri::command]
pub async fn play_sound(name: &str) -> Result<(), String> {
    sound::play_sound(name).await
}