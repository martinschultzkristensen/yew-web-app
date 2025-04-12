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
    //let sink = Sink::try_new(&stream_handle).unwrap();

    // The path will be relative to where the binary is run, which is typically the root dir
    let file = BufReader::new(File::open("resources/button-124476.mp3").unwrap());

    let source = Decoder::new(file).unwrap();
    // Play the sound directly on the device
    match stream_handle.play_raw(source.convert_samples()) {
        Ok(_) => println!("Sound played successfully"),
        Err(e) => eprintln!("Failed to play sound: {}", e),
    }

    // The sound plays in a separate audio thread,
    // so we need to keep the main thread alive while it's playing.
    std::thread::sleep(std::time::Duration::from_secs(5));
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