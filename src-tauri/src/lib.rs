//src-tauri/src/lib.rs
#[cfg_attr(mobile, tauri::mobile_entry_point)]
use toml;
use serde::Deserialize;
use serde::Serialize;
use std::fmt;

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

impl From<String> for ConfigError {
    fn from(error: String) -> Self {
        ConfigError(error)
    }
}

impl std::error::Error for ConfigError {}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
struct Config {
    pub dancers: Dancers,
    pub demo_videos: DemoVideos,
    pub choreo_videos: ChoreoVideos,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
struct Dancers {
    pub list: Vec<ConfigDancer>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
struct ConfigDancer {
    pub name: String,
    pub image: String,
    pub strength: u8,
    pub flexibility: u8,
    pub in_choreography_nr: Vec<usize>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
struct DemoVideos {
    pub list: Vec<DemoVideoConfig>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
struct DemoVideoConfig {
    pub id: usize,
    pub url: String,
    pub loop_video: bool,
    pub title: String,
    pub duration: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
struct ChoreoVideos {
    pub list: Vec<ChoreoVideoConfig>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
struct ChoreoVideoConfig {
    pub id: usize,
    pub url: String,
    pub loop_video: bool,
}

impl Config {
    pub fn from_file(path: &str) -> Result<Self, ConfigError> {
        let config_content = std::fs::read_to_string(path).map_err(|err| err.to_string())?;
        let config: Config = toml::from_str(&config_content).map_err(|err| err.to_string())?;
        Ok(config)
    }
}


#[tauri::command]
fn get_config() -> Result<Config, String> {
    // Log the current working directory
    match std::env::current_dir() {
        Ok(path) => println!("Current directory: {:?}", path),
        Err(e) => println!("Failed to get current directory: {}", e),
    }
    Config::from_file(CONFIG_PATH).map_err(|err| err.to_string())
}

pub fn run() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![get_config])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
