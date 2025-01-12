
use serde::Deserialize;
use std::{fs, path::PathBuf};
use std::time::Duration;
use tokio::time;
use crate::components::atoms::dancer::Dancer as DancerCardDancer;
use dirs_next;

/// Name of the configuration file.
pub const CONFIG_FILE_NAME: &str = "config.toml";

/// Function to find the config directory dynamically using the `dirs_next` crate.
/// Appends the `config.toml` filename to the path.
pub fn config_dir() -> PathBuf {
    dirs_next::config_dir()
        .expect("Expected a valid config directory")
        .join(CONFIG_FILE_NAME)
}

/// Function to find the data directory dynamically using the `dirs_next` crate.
pub fn data_dir() -> PathBuf {
    dirs_next::data_dir()
        .expect("Expected a valid data directory")
        .join("halloy")
}

impl Config {
    pub fn load_dancers(&self) -> Vec<DancerCardDancer> {
        self.dancers.list.iter().map(|dancer| {
            DancerCardDancer {
                image: dancer.image.clone(),
                name: dancer.name.clone(),
                strength: dancer.strength,
                flexibility: dancer.flexibility,
            }
        }).collect()
    }
}

pub fn get_config_path() -> String {
    let config_dir = dirs_next::config_dir().expect("Failed to get config directory");
    let config_path = config_dir.join("config.toml");
    config_path
        .to_str()
        .expect("Failed to convert config path to string")
        .to_string()
}

#[derive(Debug, Deserialize, Clone)]
pub struct Dancer {
    pub name: String,
    pub image: String,
    pub strength: u8,
    pub flexibility: u8,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub dancers: Dancers,
    pub songs: Songs,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Dancers {
    pub list: Vec<Dancer>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Songs {
    pub available: Vec<String>,
}

impl Config {
    pub fn from_file(path: &str) -> Self {
        let content = fs::read_to_string(path)
            .unwrap_or_else(|_| panic!("Failed to read config file: {}", path));
        toml::from_str(&content).expect("Failed to parse config file")
    }
}

pub async fn refresh_songs(config_path: &str) {
    let mut interval = time::interval(Duration::from_secs(60)); // Check every 60 seconds
    loop {
        interval.tick().await;
        let config = Config::from_file(config_path);

        // Simulate checking for new songs
        println!("Refreshing songs...");
        for song in &config.songs.available {
            println!("Available song: {}", song);
        }
    }
}