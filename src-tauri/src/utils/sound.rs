use std::{fs::{File,read_dir}, io::BufReader};
use anyhow::{Result,anyhow};
use super::dirs::app_resources_dir;


fn get_sound_file_path(name: &str) -> Result<File> {
    let sound_dir = app_resources_dir()?.join("sound");
    for entry in read_dir(sound_dir)? {
        let entry = entry?;
        if entry.file_type()?.is_dir() {
            continue;
        }
        let path = entry.path();
        let file_name = match path.file_stem(){
            Some(tn) => tn.to_str().unwrap(),
            None => continue
        };
        if file_name == name {
            return Ok(File::open(path)?);
        }
    }
    return Err(anyhow!("not found the sound file: {}", name));
}


pub async fn play_sound(name: &str) -> Result<()> {
    let file = get_sound_file_path(name)?;
    let buf = BufReader::new(file);
    let stream_handle = rodio::OutputStreamBuilder::open_default_stream().expect("open default audio stream");
    let sink = rodio::play(&stream_handle.mixer(), buf).unwrap();
    sink.set_volume(0.5);
    sink.sleep_until_end();
    Ok(())
}

