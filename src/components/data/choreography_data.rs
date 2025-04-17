//src/components/data/choreography_data.rs
use crate::components::atoms::dancer::DancerData;
use crate::components::data::config::Config;
use crate::components::molecules::video_list::VideoType;

#[derive(Clone)]
pub struct ChoreographyData {
    pub title: String,
    pub choreo_image: String,
    pub dancers: Vec<DancerData>,
    pub description: String,  // Optional: if you want to add descriptions
    pub videos: Vec<VideoType>,
}

// pub async fn get_choreography_data(choreo_number: usize) -> ChoreographyData {
//     let config_path = get_config_path();
//     log::debug!("Config path: {}", config_path);



//     let config = match Config::from_file(&config_path).await {
//         Ok(cfg) => {
//             log::debug!("Config loaded successfully");
//             cfg
//         },
//         Err(e) => {
//             log::error!("Failed to load config: {}", e);
//             return ChoreographyData {
//                 title: "Error".to_string(),
//                 choreo_image: "/static/img/default.png".to_string(),
//                 dancers: vec![],
//                 description: "Failed to load choreography data".to_string(),
//                 videos: vec![],
//             };
//         }
//     };

    
pub fn get_choreography_data(config: &Config, choreo_number: usize) -> ChoreographyData {
    let videos = config.load_choreo_videos();
    let dancers_map = config.load_dancers();
    let demo_videos_map: std::collections::HashMap<usize, (String, String)> = config // HashMap<K, V> K = video.id as usize, V = (video.title, video.description)
        .demo_videos
        .list
        .iter()
        .map(|video| {
            let description = video.description.clone().unwrap_or_else(|| {
                "There is not yet a description for this choreography❗".to_string()
            });
            (video.id as usize, (video.title.clone(), description))
        })
        .collect();

    let demo_video_text = |id: usize| -> (String, String) {
        demo_videos_map.get(&id).cloned().unwrap_or_else(|| ("Default Title".to_string(), "There is not yet a description for this choreography❗".to_string()))
    };



    match choreo_number {
        1 => ChoreographyData {
            title: demo_video_text(1).0.to_string(),
            description: demo_video_text(1).1.to_string(),
            choreo_image: "/static/img/LetsDuet.jpg".to_string(),
            dancers: dancers_map.get(&1).cloned().unwrap_or_default(),
            videos,
        },
        2 => ChoreographyData {
            title: demo_video_text(2).0.to_string(),
            choreo_image: "/static/img/siblings.png".to_string(),
            dancers: dancers_map.get(&2).cloned().unwrap_or_default(),
            description: demo_video_text(2).1.to_string(),
            videos,
        },
        3 => ChoreographyData {
            title: demo_video_text(3).0.to_string(),
            choreo_image: "/static/img/Hej-Nihao.png".to_string(),
            dancers: dancers_map.get(&3).cloned().unwrap_or_default(),
            description: demo_video_text(3).1.to_string(),
            videos,
        },
        4 => ChoreographyData {
            title: demo_video_text(4).0.to_string(),
            choreo_image: "/static/img/culture4fun.png".to_string(),
            dancers: dancers_map.get(&4).cloned().unwrap_or_default(),
            description: demo_video_text(4).1.to_string(),
            videos,
        },
        5 => ChoreographyData {
            title: demo_video_text(5).0.to_string(),
            choreo_image: "/static/img/frikadelle1080x1080.jpg".to_string(),
            dancers: dancers_map.get(&5).cloned().unwrap_or_default(),
            description: demo_video_text(5).1.to_string(),
            videos,
        },
        _ => ChoreographyData {
            title: "Unknown Choreography".to_string(),
            choreo_image: "/static/img/default.png".to_string(),
            dancers: vec![],
            description: "Description for unknown choreography".to_string(),
            videos: vec![],
        },
    }
}

// impl ChoreographyData {
//     pub fn from_config(config: &Config, choreo_number: usize) -> Self {
//         let videos = config.load_choreo_videos();
//         let dancers_map = config.load_dancers();
        
//         ChoreographyData {
//             title: config.demo_videos.list.get(choreo_number - 1)
//                 .map(|v| v.title.clone())
//                 .unwrap_or_else(|| "Unknown Choreography".to_string()),
//             choreo_image: format!("/static/img/Choreo{}.png", choreo_number),
//             dancers: dancers_map.get(&choreo_number).cloned().unwrap_or_default(),
//             description: format!("Description for Choreography {}", choreo_number),
//             videos,
//         }
//     }
// }