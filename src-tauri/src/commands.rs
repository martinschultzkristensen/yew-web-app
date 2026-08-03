//src-tauri/src/commands.rs
use crate::machine_delivery_store::{load_active_machine_config, resolve_delivery_media_file};
use crate::path_utils;
use crate::path_utils::{external_config_path, media_dir};
use crate::Config;
use bytes::Bytes;
use parking_lot::RwLock;
use std::collections::HashMap;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;
use tauri::path::BaseDirectory;
use tauri::Emitter;
use tauri::Manager;

const CONFIG_FILENAME: &str = "config.toml";
const CONFIG_RESOURCE_PATH: &str = "resources/config.toml"; // Path relative to resources directory

// Audio cache structure
#[derive(Default)]
pub struct AudioCache {
    effects: HashMap<String, Bytes>,
}

impl AudioCache {
    pub fn new() -> Self {
        Self {
            effects: HashMap::new(),
        }
    }

    pub fn load_effects(&mut self, handle: &tauri::AppHandle) -> Result<(), String> {
        // Pre-load your sound effects into memory
        let effect_files = ["uiToAboutChoreo.mp3", "BtnStart.mp3", "button-124476.mp3"];
        for file in effect_files {
            log::info!("Loading audio file: {}", file);
            let path = handle
                .path()
                .resolve(
                    &format!("resources/static/{}", file),
                    tauri::path::BaseDirectory::Resource,
                )
                .map_err(|e| format!("Failed to resolve audio path: {}", e))?;

            log::info!("Resolved path: {:?}", path);

            if !path.exists() {
                return Err(format!("Audio file not found at path: {:?}", path));
            }

            let data = std::fs::read(&path)
                .map_err(|e| format!("Failed to read audio file {}: {}", file, e))?;
            self.effects.insert(file.to_string(), Bytes::from(data));
            log::info!("Successfully loaded: {}", file);
        }
        Ok(())
    }
}

// Add this to your State
pub struct TauriState {
    audio_cache: Arc<RwLock<AudioCache>>,
}

impl TauriState {
    pub fn new(handle: &tauri::AppHandle) -> Result<Self, String> {
        let mut audio_cache = AudioCache::new();
        audio_cache.load_effects(handle)?;

        Ok(Self {
            audio_cache: Arc::new(RwLock::new(audio_cache)),
        })
    }
}

// Add a command to get audio data
#[tauri::command]
pub async fn get_audio_effect(
    state: tauri::State<'_, TauriState>,
    effect_name: String,
) -> Result<Vec<u8>, String> {
    let cache = state.audio_cache.read();
    cache
        .effects
        .get(&effect_name)
        .map(|bytes| bytes.to_vec())
        .ok_or_else(|| "Effect not found".to_string())
}

// This function creates a user media directory
fn get_user_media_path(handle: &tauri::AppHandle) -> Result<PathBuf, String> {
    let path = media_dir(handle)?;
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
    std::fs::copy(&source_path, &dest_path).map_err(|e| format!("Failed to copy video: {}", e))?;

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
    std::fs::copy(&source_path, &dest_path).map_err(|e| format!("Failed to copy image: {}", e))?;

    // Return the path to be used in the config
    let path_string = format!("media/{}", file_name);

    Ok(path_string)
}

fn media_protocol_url(path: &str) -> String {
    #[cfg(any(target_os = "windows", target_os = "android"))]
    {
        format!("https://media.localhost/{path}")
    }

    #[cfg(not(any(target_os = "windows", target_os = "android")))]
    {
        format!("media://{path}")
    }
}

#[tauri::command]
pub fn resolve_media_path(handle: tauri::AppHandle, path: String) -> Result<String, String> {
    if let Some(file_name) = path.strip_prefix("media/") {
        if file_name.is_empty()
            || Path::new(file_name)
                .components()
                .any(|component| !matches!(component, std::path::Component::Normal(_)))
        {
            return Err(format!("Invalid media path: {path}"));
        }

        let media_path = media_dir(&handle)?;
        let full_path = media_path.join(file_name);

        if !full_path.is_file() {
            return Err(format!("Media file not found: {file_name}"));
        }

        Ok(media_protocol_url(file_name))
    } else if path.starts_with("delivery/") {
        resolve_delivery_media_file(&handle, &path)?;
        Ok(media_protocol_url(&path))
    } else {
        Err("Only media/ and delivery/ paths are supported".to_string())
    }
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
pub async fn select_video_file(handle: tauri::AppHandle) -> Result<Option<String>, String> {
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
pub async fn select_img_file(handle: tauri::AppHandle) -> Result<Option<String>, String> {
    use tauri_plugin_dialog::{DialogExt, FileDialogBuilder};

    let dialog = handle.dialog().clone(); // 👈 clone the Dialog to pass ownership

    let (sender, receiver) = tokio::sync::oneshot::channel();

    FileDialogBuilder::new(dialog)
        .add_filter("Image Files", &["png", "jpg", "jpeg", "gif"])
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

fn load_external_or_default_config(handle: tauri::AppHandle) -> Result<Config, String> {
    println!("🔧 get_config called");

    let external_config_path = get_external_config_path(&handle)?;
    println!("🔧 External config path: {:?}", external_config_path);

    if external_config_path.exists() {
        println!("🔧 Loading external config");
        return Config::from_file(external_config_path.to_str().ok_or("Invalid path")?)
            .map_err(|err| format!("Error loading external config: {}", err));
    }

    println!("🔧 External config not found, loading default");

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
            let resource_path = handle
                .path()
                .resolve(CONFIG_RESOURCE_PATH, BaseDirectory::Resource)
                .map_err(|e| format!("Failed to resolve resource path: {}", e))?;

            // Copy the default config to the external location
            std::fs::copy(&resource_path, &external_config_path)
                .map_err(|e| format!("Failed to copy default config: {}", e))?;

            println!(
                "Created external config at: {}",
                external_config_path.display()
            );
        }

        Ok(())
    }

    // External config doesn't exist - initialize it
    ensure_external_config(&handle)?;

    // Now try loading the newly created external config
    Config::from_file(external_config_path.to_str().ok_or("Invalid path")?)
        .map_err(|err| format!("Error loading external config: {}", err))
}

#[tauri::command]
pub async fn get_config(handle: tauri::AppHandle) -> Result<Config, String> {
    match load_active_machine_config(&handle).await {
        Ok(config) => {
            log::info!("Loaded config from active machine delivery");
            Ok(config)
        }
        Err(active_error) => {
            log::warn!(
                "Active machine delivery could not be used; falling back to config.toml: {}",
                active_error
            );

            load_external_or_default_config(handle)
        }
    }
}

// Helper function to get the path to the external config file
fn get_external_config_path(handle: &tauri::AppHandle) -> Result<PathBuf, String> {
    external_config_path(handle)
}

//Command to reset config to default
#[tauri::command]
pub fn reset_config_to_default(handle: tauri::AppHandle) -> Result<Config, String> {
    let external_config_path = get_external_config_path(&handle)?;

    // Get path to bundled config
    let resource_path = handle
        .path()
        .resolve(CONFIG_RESOURCE_PATH, BaseDirectory::Resource)
        .map_err(|e| format!("Failed to resolve resource path: {}", e))?;

    // Copy the default config to the external location (overwriting existing)
    std::fs::copy(&resource_path, &external_config_path)
        .map_err(|e| format!("Failed to reset config: {}", e))?;

    // Load and return the reset config
    Config::from_file(external_config_path.to_str().ok_or("Invalid path")?)
        .map_err(|err| format!("Error loading reset config: {}", err))
}

#[tauri::command]
pub fn debug_paths(handle: tauri::AppHandle) -> Result<String, String> {
    let cfg = path_utils::external_config_path(&handle)?;
    let media = path_utils::media_dir(&handle)?;
    Ok(format!(
        "Config file → {}\nMedia folder → {}",
        cfg.display(),
        media.display()
    ))
}
