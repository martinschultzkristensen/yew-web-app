//src/components/data/config.rs
use crate::components::atoms::dancer::DancerData as Dancer;
use crate::components::molecules::video_list::DemoVideo;
use crate::components::molecules::video_list::Video;
use crate::components::molecules::video_list::VideoType;
use serde::Deserialize;
use serde_wasm_bindgen::from_value;
use wasm_bindgen::JsValue;




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
    pub in_choreography_nr: Vec<usize>,
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
    pub description: String, //<-- new
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
    pub fn from_toml(toml_str: &str) -> Result<Self, toml::de::Error> {
        let config: Config = toml::from_str(toml_str)?;
        println!("{:?}", config); // Debugging statement
        Ok(config)
    }
    // Deserialize the `JsValue` into the Rust struct
    pub fn from_jsvalue(js_value: JsValue) -> Result<Config, ConfigError> {
        // Deserialize the JsValue into Config struct using serde_wasm_bindgen
        from_value(js_value).map_err(|e| ConfigError(format!("Failed to deserialize JsValue: {:?}", e)))
    }
    pub fn load_dancers(&self) -> std::collections::HashMap<usize, Vec<Dancer>> {
        let mut choreography_map = std::collections::HashMap::new();

        for dancer_config in &self.dancers.list {
            let dancer = Dancer {
                image: dancer_config.image.clone(),
                name: dancer_config.name.clone(),
                strength: dancer_config.strength,
                flexibility: dancer_config.flexibility,
            };

            for &choreo_number in &dancer_config.in_choreography_nr {
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
                description: video_config.description.clone(),
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

}