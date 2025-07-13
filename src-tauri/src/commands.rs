//src-tauri/src/commands.rs

//use rodio::{Decoder, OutputStream, Sink}; 
use tauri::Manager;
use tauri::path::BaseDirectory;
use std::path::PathBuf;
use std::path::Path;
use crate::Config;
use tauri::Emitter;

const CONFIG_FILENAME: &str = "config.toml";
const CONFIG_RESOURCE_PATH: &str = "resources/config.toml"; // Path relative to resources directory

// This function creates a user media directory
fn get_user_media_path(handle: &tauri::AppHandle) -> Result<PathBuf, String> {
    let app_name = handle.package_info().name.clone();
    
    let path = handle.path().data_dir()  // Use data_dir instead of config_dir for media
        .map_err(|e| format!("Failed to get data dir: {}", e))?
        .join(app_name)
        .join("media");
    
    // Create directory if it doesn't exist
    if !path.exists() {
        std::fs::create_dir_all(&path)
            .map_err(|e| format!("Failed to create media directory: {}", e))?;
    }
    
    Ok(path)
}

//command to import video. Note in video_imports.rs source_path is changed to sourcePath since Tauri commands use camelCase by default when bridging between JavaScript and Rust
#[tauri::command]
pub fn import_video(handle: tauri::AppHandle, source_path: String) -> Result<String, String> {
    // Get the destination directory
    let media_dir = get_user_media_path(&handle)?;
    
    // Extract the filename from the source path
    let file_name = Path::new(&source_path)
        .file_name()
        .ok_or("Invalid source path")?
        .to_str()
        .ok_or("Invalid filename")?;
    
    // Create the destination path
    let dest_path = media_dir.join(file_name);
    
    // Copy the file
    std::fs::copy(&source_path, &dest_path)
        .map_err(|e| format!("Failed to copy video: {}", e))?;
    
    // Return the path to be used in the config
    let path_string = format!("media/{}", file_name);
    
    Ok(path_string)
}

//command to import images from config.toml after build
#[tauri::command]
pub fn import_images(handle: tauri::AppHandle, source_path: String) -> Result<String, String> {
    // Get the destination directory
    let media_dir = get_user_media_path(&handle)?;
    
    // Extract the filename from the source path
    let file_name = Path::new(&source_path)
        .file_name()
        .ok_or("Invalid source path")?
        .to_str()
        .ok_or("Invalid filename")?;
    
    // Create the destination path
    let dest_path = media_dir.join(file_name);
    
    // Copy the file
    std::fs::copy(&source_path, &dest_path)
        .map_err(|e| format!("Failed to copy image: {}", e))?;
    
    // Return the path to be used in the config
    let path_string = format!("media/{}", file_name);
    
    Ok(path_string)
}

#[tauri::command]
pub fn resolve_media_path(handle: tauri::AppHandle, path: String) -> Result<String, String> {
    // If path starts with "media/", it's in the ~/Library/Application Support/danceOmatic/media
    if path.starts_with("media/") {
        let file_name = path.strip_prefix("media/").unwrap();
        let media_path = get_user_media_path(&handle)?;
        let full_path = media_path.join(file_name);
        
        // Check if file exists
        if !full_path.exists() {
            return Err(format!("Media file not found: {}", file_name));
        }

        // Return media:// URL instead of file path
        return Ok(format!("media://{}", file_name));
    }

    Err("Only media/ paths are supported".to_string())
}


//this code should be unnesesarry for serving video as blob. Test and delete.
// //serve the video files as a blob to the frontend. (I belive only for the videos build in)
// #[tauri::command]
// pub fn get_video_path(handle: tauri::AppHandle, relative_path: String) -> Result<String, String> {
//     let asset_path = handle.path_resolver().resolve_resource(&relative_path)
//         .ok_or("Could not resolve video path")?;

//     let url = format!("file://{}", asset_path.display());
//     Ok(url)
// }
// #[tauri::command]
// pub fn load_video(handle: tauri::AppHandle, path: String) -> Result<Vec<u8>, String> {
//     use std::fs;

//     // Resolve full path using the same method you use elsewhere
//     let full_path = resolve_media_path(handle, path)?;

//     fs::read(&full_path).map_err(|e| format!("Failed to read video file: {}", e))
// }


//serve the image files as a blob to the frontend.
#[tauri::command]
pub fn get_image_path(handle: tauri::AppHandle, relative_path: String) -> Result<String, String> {
    resolve_media_path(handle, relative_path)
}


#[tauri::command]
pub async fn select_video_file(handle: tauri::AppHandle) -> Result<Option<String>, String> { //seem to remember async functions in tuari::command are very experimental, and not reliable.
    use tauri_plugin_dialog::{DialogExt, FileDialogBuilder};

    let dialog = handle.dialog().clone(); // 👈 clone the Dialog to pass ownership

    let (sender, receiver) = tokio::sync::oneshot::channel();

    FileDialogBuilder::new(dialog)
        .add_filter("Video Files", &["mp4", "webm", "mov"])
        .pick_file(move |file_path| {
            let result = match file_path {
                Some(tauri_plugin_dialog::FilePath::Path(path_buf)) => {
                    Some(path_buf.to_string_lossy().to_string())
                }
                Some(tauri_plugin_dialog::FilePath::Url(url)) => Some(url.to_string()),
                None => None,
            };
            let _ = sender.send(result);
        });

    receiver.await.map_err(|err| err.to_string())
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



//this command plays sound from backend. Works, but makes the sound more laggy.
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

// #[tauri::command]
// pub fn get_video_path(handle: tauri::AppHandle, video_name: String) -> Result<String, String> {
//     let start = std::time::Instant::now(); //<--logger variable
//     let video_path = handle.path()
//         .resolve(&format!("static/devVideo/{}", video_name), BaseDirectory::Resource)
//         .map_err(|e| e.to_string())?;

//         log::info!("my_command from tauri backend took {:?}", start.elapsed()); //<--logger shows in terminal

//     video_path.to_str()
//         .map(|s| s.to_string())
//         .ok_or("Failed to convert path to string".to_string())
        
// }