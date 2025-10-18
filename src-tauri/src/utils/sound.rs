use std::{fs::File, io::BufReader};

use super::dirs::app_resources_dir;

pub async fn play_sound(name: &str) -> Result<(), String> {
    if let Ok(app_resources_dir) = app_resources_dir() {
        let file: File = File::open(app_resources_dir.join(format!("{}.mp3", name))).map_err(|e| format!("打开文件失败: {}", e))?; 
        let buf = BufReader::new(file);
        let stream_handle = rodio::OutputStreamBuilder::open_default_stream().expect("open default audio stream");
        let sink = rodio::play(&stream_handle.mixer(), buf).unwrap();
        sink.set_volume(0.5);
        sink.sleep_until_end();
    } else {
        return Err(format!("获取应用资源目录失败"));
    }
    Ok(())
}

