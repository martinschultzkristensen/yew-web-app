//src-tauri/src/commands.rs

//use rodio::{Decoder, OutputStream, Sink}; 
use tauri::Manager;
use tauri::path::BaseDirectory;
use std::path::PathBuf;
use crate::Config;


const CONFIG_FILENAME: &str = "config.toml";
const CONFIG_RESOURCE_PATH: &str = "resources/config.toml"; // Path relative to resources directory


//serve the video files as a blob to the frontend
#[tauri::command]
pub fn get_video_path(handle: tauri::AppHandle, video_name: String) -> Result<String, String> {
    let start = std::time::Instant::now(); //<--logger variable
    let video_path = handle.path()
        .resolve(&format!("static/devVideo/{}", video_name), BaseDirectory::Resource)
        .map_err(|e| e.to_string())?;

        log::info!("my_command from tauri backend took {:?}", start.elapsed()); //<--logger shows in terminal

    video_path.to_str()
        .map(|s| s.to_string())
        .ok_or("Failed to convert path to string".to_string())
        
}


#[tauri::command]
pub fn get_config(handle: tauri::AppHandle) -> Result<Config, String> {
    // Get the external config path
    let external_config_path = get_external_config_path(&handle)?;
    
    // If external config exists, use it
    if external_config_path.exists() {
        return Config::from_file(external_config_path.to_str().ok_or("Invalid path")?)
            .map_err(|err| format!("Error loading external config: {}", err));
    }
    
// Function to ensure the external config exists
fn ensure_external_config(handle: &tauri::AppHandle) -> Result<(), String> {
    let external_config_path = get_external_config_path(handle)?;
    
    // Create parent directories if they don't exist
    if let Some(parent_dir) = external_config_path.parent() {
        if !parent_dir.exists() {
            std::fs::create_dir_all(parent_dir)
                .map_err(|e| format!("Failed to create config directory: {}", e))?;
        }
    }
    
    // If external config doesn't exist, copy from resources
    if !external_config_path.exists() {
        // Get path to bundled config
        let resource_path = handle.path()
            .resolve(CONFIG_RESOURCE_PATH, BaseDirectory::Resource)
            .map_err(|e| format!("Failed to resolve resource path: {}", e))?;
        
        // Copy the default config to the external location
        std::fs::copy(&resource_path, &external_config_path)
            .map_err(|e| format!("Failed to copy default config: {}", e))?;
        
        println!("Created external config at: {}", external_config_path.display());
    }
    
    Ok(())
}

    // External config doesn't exist - initialize it
    ensure_external_config(&handle)?;
    
    // Now try loading the newly created external config
    Config::from_file(external_config_path.to_str().ok_or("Invalid path")?)
        .map_err(|err| format!("Error loading external config: {}", err))
}

// Helper function to get the path to the external config file
fn get_external_config_path(handle: &tauri::AppHandle) -> Result<PathBuf, String> {
    let app_name = handle.package_info().name.clone();
    
    let path = handle.path().config_dir()
        .map_err(|e| format!("Failed to get config dir: {}", e))?
        .join(app_name)
        .join(CONFIG_FILENAME);
    
    Ok(path)
}

//Command to reset config to default
#[tauri::command]
pub fn reset_config_to_default(handle: tauri::AppHandle) -> Result<Config, String> {
    let external_config_path = get_external_config_path(&handle)?;
    
    // Get path to bundled config
    let resource_path = handle.path()
        .resolve(CONFIG_RESOURCE_PATH, BaseDirectory::Resource)
        .map_err(|e| format!("Failed to resolve resource path: {}", e))?;
    
    // Copy the default config to the external location (overwriting existing)
    std::fs::copy(&resource_path, &external_config_path)
        .map_err(|e| format!("Failed to reset config: {}", e))?;
    
    // Load and return the reset config
    Config::from_file(external_config_path.to_str().ok_or("Invalid path")?)
        .map_err(|err| format!("Error loading reset config: {}", err))
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

//this path command works, but makes the sound more laggy.
// #[tauri::command]
// fn play_sound_backend(sound_file: String) {
//     // Resolve absolute path (you can adjust based on how/where you store files)
//     let base_path = tauri::api::path::resource_dir().expect("No resource dir");
//     let full_path = base_path.join(sound_file);

//     let (_stream, handle) = OutputStream::try_default().expect("No audio output stream");
//     let sink = Sink::try_new(&handle).expect("Failed to create audio sink");

//     let file = File::open(full_path).expect("Failed to open sound file");
//     let source = Decoder::new(BufReader::new(file)).expect("Failed to decode audio");

//     sink.append(source);
//     sink.detach(); // play-and-forget
// }