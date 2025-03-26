//src/components/data/choreography_data.rs
// use crate::components::molecules::video_list::VideoType;
// use crate::components::data::video_data::get_demo_videos;
use crate::components::atoms::dancer::DancerData;
use crate::components::data::config::Config;
use crate::components::molecules::video_list::VideoType;

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
            title: "Choreo 1".to_string(),
            choreo_image: "/static/img/AI&Boy.png".to_string(),
            dancers: dancers.clone().into_iter().take(2).collect(), // Use directly
            description: "Description for Choreography 1".to_string(),
        },
        2 => ChoreographyData {
            title: "Choreo 2".to_string(),
            choreo_image: "/static/img/siblings.png".to_string(),
            dancers: dancers.clone().into_iter().skip(1).take(2).collect(), // Example: dancers 2-3
            description: "Description for Choreography 2".to_string(),
        },
        3 => ChoreographyData {
            title: "Choreo 3".to_string(),
            choreo_image: "/static/img/culture4fun.png".to_string(),
            dancers: dancers.clone().into_iter().skip(1).take(2).collect(), // Example: dancers 2-3
            description: "Description for Choreography 2".to_string(),
        },
        _ => ChoreographyData {
            title: format!("Choreo {}", choreo_number),
            choreo_image: "static/default_choreo.jpg".to_string(),
            dancers: vec![], // Default to no dancers
            description: "Default description".to_string(),
        },
    }
}
// Old function to get choreography data
// pub fn get_choreography_data(choreo_number: usize) -> ChoreographyData {
//     let martin = Dancer {
//         image: "/static/img/Martinus.png".to_string(),
//         name: "Martin".to_string(),
//         strength: 10,
//         flexibility: 5,
//     };
//     let alex = Dancer {
//         image: "/static/img/Jon.png".to_string(),
//         name: "Alex".to_string(),
//         strength: 8,
//         flexibility: 7,
//     };
//     let mengting = Dancer {
//         image: "/static/img/Mengting.png".to_string(),
//         name: "Meng-Ting".to_string(),
//         strength: 5,
//         flexibility: 10,
//     };
//     let mengyuan = Dancer {
//         image: "/static/img/Mengyuin.png".to_string(),
//         name: "Meng-Yuan".to_string(),
//         strength: 8,
//         flexibility: 7,
//     };
    
//     match choreo_number {
//         1 => ChoreographyData {
//             title: match &get_demo_videos()[0] {
//                 VideoType::Demo(demo) => demo.title.clone(), //is borrow possible? -research
//                 _ => "Choreo 1".to_string(),
//             },
//             choreo_image: "/static/img/AI&Boy.png".to_string(),
//             dancers: vec![alex.clone(), martin.clone()],
//             description: "Description for Choreography 1... This is a very good choreography".to_string(),
//         },
//         2 => ChoreographyData {
//             title: match &get_demo_videos()[1] {
//                 VideoType::Demo(demo) => demo.title.clone(),
//                 _ => "Choreo 2".to_string(),
//             },
//             choreo_image: "static/choreo2_image.jpg".to_string(),
//             dancers: vec![alex.clone(), mengting.clone()],
//             description: "This is a very good choreography this Choreography 2".to_string(),
//         },
//         3 => ChoreographyData {
//             title: match &get_demo_videos()[2] {
//                 VideoType::Demo(demo) => demo.title.clone(),
//                 _ => "Choreo 2".to_string(),
//             },
//             choreo_image: "static/choreo2_image.jpg".to_string(),
//             dancers: vec![martin.clone(), mengting.clone(), mengyuan.clone()],
//             description: "Description for 3rd dancepiece. Very long just enjoy".to_string(),
//         },
//         4 => ChoreographyData {
//             title: match &get_demo_videos()[3] {
//                 VideoType::Demo(demo) => demo.title.clone(),
//                 _ => "Choreo 2".to_string(),
//             },
//             choreo_image: "static/choreo2_image.jpg".to_string(),
//             dancers: vec![martin.clone(), mengting.clone(), mengyuan.clone()],
//             description: "Very long Description for Choreography 4".to_string(),
//         },
//         // Add cases 3-5 following the same pattern...
//         _ => ChoreographyData {
//             title: format!("Choreo{}", choreo_number),
//             choreo_image: "static/default_choreo.jpg".to_string(),
//             dancers: vec![],
//             description: "".to_string(),
//         }
//     }
// }