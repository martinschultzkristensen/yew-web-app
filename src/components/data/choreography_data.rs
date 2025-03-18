//src/components/data/choreography_data.rs
// use crate::components::molecules::video_list::VideoType;
// use crate::components::data::video_data::get_demo_videos;
use crate::components::atoms::dancer::DancerData;
use crate::components::data::config::{Config, get_config_path};

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

    log::debug!("Number of dancers loaded: {}", config.dancers.list.len());

    let dancers = config.load_dancers();
    
    match choreo_number {
        1 => ChoreographyData {
            title: "Let's Duet".to_string(),
            choreo_image: "/static/img/AI&Boy.png".to_string(),
            dancers: dancers.clone().into_iter().take(2).collect(), // Use directly
            description: "Description for Choreography 1".to_string(),
        },
        2 => ChoreographyData {
            title: "Siblings".to_string(),
            choreo_image: "static/choreo2_image.jpg".to_string(),
            dancers: dancers.clone().into_iter().take(3).collect(), // Example: dancers 1-2-3
            description: "Description for Choreography 2".to_string(),
        },
        3 => ChoreographyData {
            title: "Hej-Nihao".to_string(),
            choreo_image: "static/choreo2_image.jpg".to_string(),
            dancers: dancers.clone().into_iter().skip(1).take(2).collect(), // Example: dancers 2-3
            description: "Description for Choreography 3".to_string(),
        },
        4 => ChoreographyData {
            title: "Cultur4Fun".to_string(),
            choreo_image: "static/choreo2_image.jpg".to_string(),
            dancers: dancers.clone().into_iter().skip(2).take(2).collect(), // Example: dancers 3-4
            description: "Description for Choreography 3".to_string(),
        },
        5 => ChoreographyData {
            title: "3 Kokke".to_string(),
            choreo_image: "static/choreo2_image.jpg".to_string(),
            dancers: dancers.clone().into_iter().skip(2).take(2).collect(), // Example: dancers 3-4
            description: "Description for Choreography 3".to_string(),
        },
        _ => ChoreographyData {
            title: format!("Choreo {}", choreo_number),
            choreo_image: "static/default_choreo.jpg".to_string(),
            dancers: vec![], // Default to no dancers
            description: "Default description".to_string(),
        },
    }
}
