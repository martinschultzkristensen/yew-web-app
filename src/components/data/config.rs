
use serde::Deserialize;
use std::{fs, path::Path};
use std::time::Duration;
use tokio::time;

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