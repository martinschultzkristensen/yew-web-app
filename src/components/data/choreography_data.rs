//src/components/data/choreography_data.rs

use crate::components::atoms::dancer::DancerData;
use crate::components::data::config::{Config, get_config_path};
use crate::components::data::config::DemoVideoConfig;

#[derive(Clone)]
pub struct ChoreographyData {
    pub title: String,
    pub choreo_image: String,
    pub dancers: Vec<DancerData>,
    pub description: String,  // Optional: if you want to add descriptions
}

pub async fn get_choreography_data(choreo_number: usize) -> ChoreographyData {
    let config_path = get_config_path();
    log::debug!("Config path: {}", config_path);

    let config = match Config::from_file(&config_path).await {
        Ok(cfg) => {
            log::debug!("Config loaded successfully");
            cfg
        },
        Err(e) => {
            log::error!("Failed to load config: {}", e);
            return ChoreographyData {
                title: "Error".to_string(),
                choreo_image: "/static/img/default.png".to_string(),
                dancers: vec![],
                description: "Failed to load choreography data".to_string(),
            };
        }
    };

    let dancers_map = config.load_dancers();
    let demo_videos_map: std::collections::HashMap<usize, String> = config
        .demo_videos
        .list
        .iter()
        .map(|video| (video.id as usize, video.title.clone())) // Clone the title
        .collect();

    let demo_video_title = |id: usize| -> String {
        demo_videos_map.get(&id).cloned().unwrap_or_else(|| "Default Title".to_string())
    };

    match choreo_number {
        1 => ChoreographyData {
            title: demo_video_title(1).to_string(),
            choreo_image: "/static/img/AI&Boy.png".to_string(),
            dancers: dancers_map.get(&1).cloned().unwrap_or_default(),
            description: "Description for Choreography 1".to_string(),
        },
        2 => ChoreographyData {
            title: demo_video_title(2).to_string(),
            choreo_image: "/static/img/choreo2.png".to_string(),
            dancers: dancers_map.get(&2).cloned().unwrap_or_default(),
            description: "Description for Choreography 2".to_string(),
        },
        3 => ChoreographyData {
            title: demo_video_title(3).to_string(),
            choreo_image: "/static/img/choreo3.png".to_string(),
            dancers: dancers_map.get(&3).cloned().unwrap_or_default(),
            description: "Description for Choreography 3".to_string(),
        },
        4 => ChoreographyData {
            title: demo_video_title(4).to_string(),
            choreo_image: "/static/img/choreo4.png".to_string(),
            dancers: dancers_map.get(&4).cloned().unwrap_or_default(),
            description: "Description for Choreography 4".to_string(),
        },
        5 => ChoreographyData {
            title: demo_video_title(5).to_string(),
            choreo_image: "/static/img/choreo5.png".to_string(),
            dancers: dancers_map.get(&5).cloned().unwrap_or_default(),
            description: "Description for Choreography 5".to_string(),
        },
        _ => ChoreographyData {
            title: "Unknown Choreography".to_string(),
            choreo_image: "/static/img/default.png".to_string(),
            dancers: vec![],
            description: "Description for unknown choreography".to_string(),
        },
    }
}

