extern crate rodio;
use std::path::PathBuf;
use std::fs::File;
use std::io::BufReader;
use rodio::{Decoder, OutputStream, Sink, source::Source};
// use symphonia::core::formats::FormatOptions;
// use symphonia::core::io::MediaSourceStream;
// use symphonia::core::meta::MetadataOptions;
// use symphonia::default::get_probe;


#[tauri::command]
pub fn play_sound_backend() {
    println!("✅ Backend sound command received!");
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    // The path will be relative to where the binary is run, which is typically the root dir
    let mut path = PathBuf::from("resources/button-124476.mp3");

    // Open and decode the audio file
    let file = BufReader::new(File::open(&path).unwrap());
    let source = Decoder::new(file).unwrap();
    sink.append(source);

    sink.detach(); // Don't block — let it play asynchronously
}

// #[tauri::command]
// fn init_app_data(app_handle: tauri::AppHandle) -> Result<String, String> {
//     let app_dir = app_handle.path_resolver().app_data_dir().expect("Failed to get app data dir");
//     let custom_dir = app_dir.join("customizable");
    
//     // Create directory if it doesn't exist
//     if !custom_dir.exists() {
//         std::fs::create_dir_all(&custom_dir).map_err(|e| e.to_string())?;
//     }
    
//     Ok(custom_dir.to_string_lossy().to_string())
// }