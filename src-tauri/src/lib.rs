//src-tauri/src/lib.rs
mod commands;
use commands::*;
use serde::{Deserialize, Serialize};
use std::fmt;
use tauri::http::Response;
use tauri::{Manager, Runtime};
use tauri_plugin_log::{Target, TargetKind};
use toml;
pub mod path_utils;

//const CONFIG_PATH: &str = "resources/config.toml";

pub struct ConfigError(String);
impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ConfigError: {}", self.0)
    }
}
impl fmt::Debug for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ConfigError: {}", self.0)
    }
}
impl std::error::Error for ConfigError {}

impl From<String> for ConfigError {
    fn from(err: String) -> Self {
        ConfigError(err)
    }
}

#[derive(Debug, Deserialize, Clone, PartialEq, Serialize)]
pub struct ConfigDancer {
    pub name: String,
    pub image: String,
    pub strength: u8,
    pub flexibility: u8,
    pub in_choreography_nr: Vec<usize>,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Serialize)]
pub struct Dancers {
    pub list: Vec<ConfigDancer>,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Serialize)]
pub struct DemoVideoConfig {
    pub id: usize,
    pub url: String,
    pub loop_video: bool,
    pub title: String,
    pub description: Option<String>,
    pub duration: String,
    pub choreo_img: Option<String>,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Serialize)]
pub struct DemoVideos {
    pub list: Vec<DemoVideoConfig>,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Serialize)]
pub struct ChoreoVideoConfig {
    pub id: usize,
    pub url: String,
    pub loop_video: bool,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Serialize)]
pub struct ChoreoVideos {
    pub list: Vec<ChoreoVideoConfig>,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Serialize)]
pub struct Config {
    pub dancers: Dancers,
    pub demo_videos: DemoVideos,
    pub choreo_videos: ChoreoVideos,
    pub intro_video: ChoreoVideoConfig,
    pub loadscreen_video: ChoreoVideoConfig,
}

impl Config {
    pub fn from_file(path: &str) -> Result<Self, ConfigError> {
        let config_content = std::fs::read_to_string(path).map_err(|err| err.to_string())?;
        let config: Config = toml::from_str(&config_content).map_err(|err| err.to_string())?;
        Ok(config)
    }
}

//now get's config from commands.rs Same code under comment: "Fall back to bundle config"
// #[tauri::command]
// fn get_config(handle: tauri::AppHandle) -> Result<Config, String> {
//     let resource_path = handle.path()
//     .resolve(CONFIG_PATH, BaseDirectory::Resource)
//     .map_err(|err| err.to_string())?;

//     Config::from_file(resource_path.to_str().ok_or("Invalid path")?)
//         .map_err(|err| err.to_string())

// }

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        //new section from here adding error output.
        .plugin(
            tauri_plugin_log::Builder::new()
                .targets([
                    Target::new(TargetKind::Stdout),                     //<-- to terminal
                    Target::new(TargetKind::LogDir { file_name: None }), //<-- to an com.artfarm.danceomatic
                    Target::new(TargetKind::Webview),
                ])
                .build(),
        )
        //detailed logging in build
        .setup(|app| {
            println!("🔧 App setup starting...");
            //more logging of bundel app
            if let Ok(resource_dir) = app.path().resource_dir() {
                println!("📁 Resource dir: {:?}", resource_dir);
                if let Ok(entries) = std::fs::read_dir(&resource_dir) {
                    println!("📁 Resource dir contents:");
                    for entry in entries {
                        if let Ok(entry) = entry {
                            println!("  - {:?}", entry.path());
                        }
                    }
                }
            }
            // Check if media directory exists
            if let Ok(app_data_dir) = app.path().app_data_dir() {
                let media_dir = app_data_dir.join("media");
                println!("📁 Media dir: {:?}", media_dir);
                println!("📁 Media dir exists: {}", media_dir.exists());

                if let Ok(entries) = std::fs::read_dir(&media_dir) {
                    println!("📁 Media dir contents:");
                    for entry in entries {
                        if let Ok(entry) = entry {
                            println!("  - {:?}", entry.path());
                        }
                    }
                }
            }

            println!("🔧 App setup complete");
            Ok(())
        })
        .register_uri_scheme_protocol("media", move |app, request| {
            // Resolve the *same* media folder that the commands use
            let media_root = match path_utils::media_dir(&app.app_handle()) {
                Ok(p) => p,
                Err(e) => {
                    eprintln!("⚠️ Could not resolve media dir: {e}");
                    return Response::builder().status(500).body(Vec::new()).unwrap();
                }
            };

            // Parse file path from URI
            let uri = request.uri().to_string();
            let rel_path = uri.trim_start_matches("media://").trim_end_matches('/');
            let full_path = media_root.join(rel_path);

            // Debug logging
            eprintln!("🔍 media:// request → {full_path:?}");

            // Try to read the file
            match std::fs::read(&full_path) {
                Ok(data) => {
                    let mime = mime_guess::from_path(&full_path).first_or_octet_stream();
                    Response::builder()
                        .header("Content-Type", mime.as_ref())
                        .body(data)
                        .unwrap()
                }
                Err(e) => {
                    eprintln!("❌ Failed to serve {rel_path}: {e}");
                    Response::builder().status(404).body(Vec::new()).unwrap()
                }
            }
        })
        .invoke_handler(tauri::generate_handler![
            get_config,
            debug_paths,
            reset_config_to_default,
            import_video,
            import_images,
            resolve_media_path,
            // get_video_path,
            // load_video,
            get_image_path,
            select_video_file
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
