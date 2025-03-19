//src/components/data/config.rs
use crate::components::atoms::dancer::DancerData as Dancer;
use crate::components::molecules::video_list::DemoVideo;
use crate::components::molecules::video_list::Video;
use crate::components::molecules::video_list::VideoType;
use serde::Deserialize;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, Response};

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

#[derive(Debug, Deserialize, Clone)]
pub struct ConfigDancer {
    pub name: String,
    pub image: String,
    pub strength: u8,
    pub flexibility: u8,
    pub inChoreographyNr: Vec<usize>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Dancers {
    pub list: Vec<ConfigDancer>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct DemoVideoConfig {
    pub id: usize,
    pub url: String,
    pub loop_video: bool,
    pub title: String,
    pub duration: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct DemoVideos {
    pub list: Vec<DemoVideoConfig>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub dancers: Dancers,
    pub demo_videos: DemoVideos,
}

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

            for &choreo_number in &dancer_config.inChoreographyNr {
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

    pub async fn from_file(path: &str) -> Result<Self, ConfigError> {
        log::debug!("Attempting to load config from path: {}", path);

        // Create request options
        let opts = RequestInit::new();
        opts.set_method("GET");

        log::debug!("Created request options");

        // Create the request
        let request = match Request::new_with_str_and_init(path, &opts) {
            Ok(req) => {
                log::debug!("Successfully created request");
                req
            }
            Err(e) => {
                log::error!("Failed to create request: {:?}", e);
                return Err(ConfigError(format!("Failed to create request: {:?}", e)));
            }
        };

        // Get window object
        let window = match web_sys::window() {
            Some(w) => {
                log::debug!("Got window object");
                w
            }
            None => {
                log::error!("No window object found");
                return Err(ConfigError("No window object found".to_string()));
            }
        };

        // Fetch the file
        log::debug!("Initiating fetch request");
        let resp_value = match JsFuture::from(window.fetch_with_request(&request)).await {
            Ok(rv) => {
                log::debug!("Fetch successful");
                rv
            }
            Err(e) => {
                log::error!("Fetch failed: {:?}", e);
                return Err(ConfigError(format!("Fetch failed: {:?}", e)));
            }
        };

        let resp: Response = resp_value
            .dyn_into()
            .map_err(|e| ConfigError(format!("Response conversion failed: {:?}", e)))?;

        // Get the response text
        let text = JsFuture::from(
            resp.text()
                .map_err(|e| ConfigError(format!("Text promise failed: {:?}", e)))?,
        )
        .await
        .map_err(|e| ConfigError(format!("Text extraction failed: {:?}", e)))?;

        let config_text = text
            .as_string()
            .ok_or_else(|| ConfigError("Failed to convert response to string".to_string()))?;

        // Parse the TOML
        toml::from_str(&config_text)
            .map_err(|e| ConfigError(format!("Failed to parse config: {}", e)))
    }
}

pub fn get_config_path() -> String {
    "/static/config.toml".to_string()
}
