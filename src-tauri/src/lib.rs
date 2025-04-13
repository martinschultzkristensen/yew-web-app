//src-tauri/src/lib.rs
mod commands;
use commands::play_sound_backend;
use serde::{Deserialize, Serialize};
use std::fmt;
use toml;
const CONFIG_PATH: &str = "src/config.toml";

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
    pub duration: String,
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
}

impl Config {
    pub fn from_file(path: &str) -> Result<Self, ConfigError> {
        let config_content = std::fs::read_to_string(path).map_err(|err| err.to_string())?;
        let config: Config = toml::from_str(&config_content).map_err(|err| err.to_string())?;
        Ok(config)
    }
}

#[tauri::command]
fn play_sound_backend2() {
    println!("✅ Backend sound command received!");
}

#[tauri::command]
fn get_config() -> Result<Config, String> {
    // Log the current working directory
    // match std::env::current_dir() {
    //     Ok(path) => log::info!("Current directory: {:?}", path),
    //     Err(e) => log::error!("Failed to get current directory: {}", e),
    // }
    Config::from_file(CONFIG_PATH).map_err(|err| err.to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // env_logger::init();
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .invoke_handler(tauri::generate_handler![get_config])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
