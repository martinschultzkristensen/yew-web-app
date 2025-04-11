use std::fs::File;
use std::io::BufReader;
use rodio::{Decoder, OutputStream, Sink};

#[tauri::command]
fn play_sound_backend(sound_file: String) {
    // Resolve absolute path (you can adjust based on how/where you store files)
    let base_path = tauri::api::path::resource_dir().expect("No resource dir");
    let full_path = base_path.join(sound_file);

    let (_stream, handle) = OutputStream::try_default().expect("No audio output stream");
    let sink = Sink::try_new(&handle).expect("Failed to create audio sink");

    let file = File::open(full_path).expect("Failed to open sound file");
    let source = Decoder::new(BufReader::new(file)).expect("Failed to decode audio");

    sink.append(source);
    sink.detach(); // play-and-forget
}

#[tauri::command]
fn init_app_data(app_handle: tauri::AppHandle) -> Result<String, String> {
    let app_dir = app_handle.path_resolver().app_data_dir().expect("Failed to get app data dir");
    let custom_dir = app_dir.join("customizable");
    
    // Create directory if it doesn't exist
    if !custom_dir.exists() {
        std::fs::create_dir_all(&custom_dir).map_err(|e| e.to_string())?;
    }
    
    Ok(custom_dir.to_string_lossy().to_string())
}