//src/components/data/choreography_data.rs
use crate::components::molecules::video_list::VideoType;
use crate::components::data::video_data::get_demo_videos;
use crate::components::atoms::dancer::*;

#[derive(Clone)]
pub struct ChoreographyData {
    pub title: String,
    pub choreo_image: String,
    pub dancers: Vec<Dancer>,
    pub description: String,  // Optional: if you want to add descriptions
}



pub fn get_choreography_data(choreo_number: usize) -> ChoreographyData {
    let mut dancer_manager = DancerManager::new();
    
    // Add dancers with unique IDs
    let martin_id = dancer_manager.add_dancer(
        "/static/img/Martinus.png".to_string(),
        "Martin".to_string(),
        10,
        5
    ).expect("Failed to add Martin");
    
    let alex_id = dancer_manager.add_dancer(
        "/static/img/Jon.png".to_string(),
        "Alex".to_string(),
        8,
        7
    ).expect("Failed to add Alex");
    
    let mengting_id = dancer_manager.add_dancer(
        "/static/img/Mengting.png".to_string(),
        "Meng-Ting".to_string(),
        5,
        10
    ).expect("Failed to add Meng-Ting");
    
    let mengyuan_id = dancer_manager.add_dancer(
        "/static/img/Mengyuin.png".to_string(),
        "Meng-Yuan".to_string(),
        8,
        7
    ).expect("Failed to add Meng-Yuan");
    
    // Get references to the dancers by their IDs
    let dancers = dancer_manager.get_dancers();
    let martin = &dancers[martin_id - 1];
    let alex = &dancers[alex_id - 1];
    let mengting = &dancers[mengting_id - 1];
    let mengyuan = &dancers[mengyuan_id - 1];
    
    match choreo_number {
        1 => ChoreographyData {
            title: match &get_demo_videos()[0] {
                VideoType::Demo(demo) => demo.title.clone(),
                _ => "Choreo 1".to_string(),
            },
            choreo_image: "/static/img/AI&Boy.png".to_string(),
            dancers: vec![alex.clone(), martin.clone()],
            description: "Description for Choreography 1... This is a very good choreography".to_string(),
        },
        2 => ChoreographyData {
            title: match &get_demo_videos()[1] {
                VideoType::Demo(demo) => demo.title.clone(),
                _ => "Choreo 2".to_string(),
            },
            choreo_image: "static/choreo2_image.jpg".to_string(),
            dancers: vec![alex.clone(), mengting.clone()],
            description: "This is a very good choreography this Choreography 2".to_string(),
        },
        3 => ChoreographyData {
            title: match &get_demo_videos()[2] {
                VideoType::Demo(demo) => demo.title.clone(),
                _ => "Choreo 3".to_string(),
            },
            choreo_image: "static/choreo3_image.jpg".to_string(),
            dancers: vec![martin.clone(), mengting.clone(), mengyuan.clone()],
            description: "Description for 3rd dancepiece. Very long just enjoy".to_string(),
        },
        4 => ChoreographyData {
            title: match &get_demo_videos()[3] {
                VideoType::Demo(demo) => demo.title.clone(),
                _ => "Choreo 4".to_string(),
            },
            choreo_image: "static/choreo4_image.jpg".to_string(),
            dancers: vec![martin.clone(), mengting.clone(), mengyuan.clone()],
            description: "Very long Description for Choreography 4".to_string(),
        },
        // Add more cases as needed
        _ => ChoreographyData {
            title: format!("Choreo{}", choreo_number),
            choreo_image: "static/default_choreo.jpg".to_string(),
            dancers: vec![],
            description: "".to_string(),
        }
    }
}