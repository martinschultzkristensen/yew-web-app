//src/components/data/config.rs
use crate::components::atoms::dancer::DancerData as Dancer;
use crate::components::molecules::video_list::DemoVideo;
use crate::components::molecules::video_list::Video;
use crate::components::molecules::video_list::VideoType;
use serde::Deserialize;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, Response};
use tauri::api::file;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use log;




pub struct ConfigError(String);

impl std::fmt::Debug for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ConfigError: {}", self.0)
    }
}

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for ConfigError {}

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


//impl PartialEq for Config and comprare relevant fields

impl Config {
    pub fn load_dancers(&self) -> std::collections::HashMap<usize, Vec<Dancer>> {
        let mut choreography_map = std::collections::HashMap::new();

        for dancer_config in &self.dancers.list {
            let dancer = Dancer {
                image: dancer_config.image.clone(),
                name: dancer_config.name.clone(),
                strength: dancer_config.strength,
                flexibility: dancer_config.flexibility,
            };

            for &choreo_number in &dancer_config.in_chroeography_nr {
                choreography_map
                    .entry(choreo_number)
                    .or_insert_with(Vec::new)
                    .push(dancer.clone());
            }
        }

        choreography_map
    }

    pub fn get_demo_videos(&self) -> Vec<VideoType> {
        self.demo_videos.list.iter().map(|video_config| {
            VideoType::Demo(DemoVideo {
                video: Video {
                    id: video_config.id,
                    url: video_config.url.clone(),
                    loop_video: video_config.loop_video,
                },
                title: video_config.title.clone(),
                duration: video_config.duration.clone(),
            })
        }).collect()
    }

    pub fn load_choreo_videos(&self) -> Vec<VideoType> {
        self.choreo_videos.list.iter().map(|video_config| {
            VideoType::Regular(Video {
                id: video_config.id,
                url: video_config.url.clone(),
                loop_video: video_config.loop_video,
            })
        }).collect()
    }

    pub async fn from_file(path: &str) -> Result<Self, ConfigError> {
        log::debug!("Attempting to load config from path: {}", path);
        
        // Get the config file path
        let mut config_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        config_path.push(path);
        
        // Read the file using Tauri's file API
        match file::read_string(config_path).await {
            Ok(contents) => {
                toml::from_str(&contents)
                    .map_err(|e| ConfigError(format!("Failed to parse config: {}", e)))
            }
            Err(e) => {
                log::error!("Failed to read config file: {:?}", e);
                Err(ConfigError(format!("Failed to read config file: {:?}", e)))
            }
        }
    }
}

pub fn get_config_path() -> String {
    "/static/config.toml".to_string()
}
