//src/components/data/config.rs
use crate::components::atoms::dancer::DancerData as Dancer;
use dirs_next;
use futures::stream::StreamExt;
use gloo::timers::future::IntervalStream;
use serde::Deserialize;
use std::fs;
use std::path::PathBuf;
use gloo_file::File;
use web_sys::window;

/// Name of the configuration file.
pub const CONFIG_FILE_NAME: &str = "config.toml";

//static Config = Config::from_file("/Users/martinsk/projects/yew-app/config.toml");

/// Function to find the config directory dynamically using the `dirs_next` crate.
/// Appends the `config.toml` filename to the path.
pub fn config_dir() -> PathBuf {
    dirs_next::config_dir()
        .expect("Expected a valid config directory")
        .join(CONFIG_FILE_NAME)
}

impl Config {
    pub fn load_dancers(&self) -> Vec<Dancer> {
        self.dancers
            .list
            .iter()
            .map(|dancer| Dancer {
                image: dancer.image.clone(),
                name: dancer.name.clone(),
                strength: dancer.strength,
                flexibility: dancer.flexibility,
            })
            .collect()
    }
}

// pub fn get_config_path() -> String {
//     let config_dir = dirs_next::config_dir().expect("Failed to get config directory");
//     let config_path = config_dir.join("config.toml");
//     config_path
//         .to_str()
//         .expect("Failed to convert config path to string")
//         .to_string()
// }
pub fn get_config_path() -> String {
    "/config.toml".to_string()
}

#[derive(Debug, Deserialize, Clone)]
pub struct ConfigDancer {
    pub name: String,
    pub image: String,
    pub strength: u8,
    pub flexibility: u8,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub dancers: Dancers,
    // pub songs: Songs,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Dancers {
    pub list: Vec<ConfigDancer>,
}

// #[derive(Debug, Deserialize, Clone)]
// pub struct Songs {
//     pub available: Vec<String>,
// }

impl Config {
    pub fn from_file(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        // Use fetch to get file content
        let window = window().unwrap();
        let document = window.document().unwrap();
        let response = window
            .fetch_with_str(path)
            .ok()
            .and_then(|r| r.text().ok())
            .ok_or("Failed to fetch config")?;

        toml::from_str(&response)
            .map_err(|e| format!("Failed to parse config: {}", e).into())
    }
}

// pub async fn refresh_songs(config_path: &str) {
//     let mut interval = IntervalStream::new(60_000);

//     while (interval.next().await).is_some() {
//         // Simulate reading config from file (replace with actual async logic if needed)
//         let config = Config::from_file(config_path);

//         // Simulate checking for new songs
//         println!("Refreshing songs...");
//         for song in &config.unwrap().songs.available {
//             println!("Available song: {}", song);
//         }
//     }
// }
