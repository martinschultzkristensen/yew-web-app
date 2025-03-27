//src-tauri/src/lib.rs
use serde::Deserialize;

// Copy the exact structs from your frontend config.rs
#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct ConfigDancer {
    pub name: String,
    pub image: String,
    pub strength: u8,
    pub flexibility: u8,
    pub in_chroeography_nr: Vec<usize>,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct Dancers {
    pub list: Vec<ConfigDancer>,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct DemoVideoConfig {
    pub id: usize,
    pub url: String,
    pub loop_video: bool,
    pub title: String,
    pub duration: String,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct DemoVideos {
    pub list: Vec<DemoVideoConfig>,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct ChoreoVideoConfig {
    pub id: usize,
    pub url: String,
    pub loop_video: bool,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct ChoreoVideos {
    pub list: Vec<ChoreoVideoConfig>,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct Config {
    pub dancers: Dancers,
    pub demo_videos: DemoVideos,
    pub choreo_videos: ChoreoVideos,
}

use std::fs;
use std::path::PathBuf;
use tauri::{AppHandle, Manager};

pub fn load_config(app_handle: &AppHandle) -> Result<Config, String> {
    // Get the resource directory
    let resource_path = app_handle
        .path()
        .resource_dir()
        .map_or_else(
            |_| Err("Failed to get resource directory".to_string()),
            |path| Ok(path.join("static/config.toml"))
        )?;

    let config_text = fs::read_to_string(&resource_path)
        .map_err(|e| format!("Failed to read config file at {:?}: {}", resource_path, e))?;

    toml::from_str(&config_text)
        .map_err(|e| format!("Failed to parse config: {}", e))
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let app_handle = app.handle().clone();
            match load_config(&app_handle) {
                Ok(config) => {
                    println!("Config loaded successfully");
                    // Optional: store config in app state if needed
                    // app.handle().manage(config);
                }
                Err(e) => {
                    eprintln!("Failed to load config: {}", e);
                }
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}