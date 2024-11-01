use crate::components::molecules::video_list::VideoType;
use crate::components::data::video_data::get_demo_videos;

#[derive(Clone)]
pub struct ChoreographyData {
    pub title: String,
    pub choreo_image: String,
    pub dancer_images: Vec<String>,
    pub description: String,  // Optional: if you want to add descriptions
}

// Function to get choreography data
pub fn get_choreography_data(choreo_number: usize) -> ChoreographyData {
    match choreo_number {
        1 => ChoreographyData {
            title: match &get_demo_videos()[0] {
                VideoType::Demo(demo) => demo.title.clone(), //is borrow possible? -research
                _ => "Choreo 1".to_string(),
            },
            choreo_image: "static/img/AI&Boy.png".to_string(),
            dancer_images: vec![
                "static/img/Martinus.png".to_string(),
                "static/img/Jon.png".to_string(),
            ],
            description: "Description for Choreography 1... This is a very good choreography".to_string(),
        },
        2 => ChoreographyData {
            title: match &get_demo_videos()[1] {
                VideoType::Demo(demo) => demo.title.clone(),
                _ => "Choreo 2".to_string(),
            },
            choreo_image: "static/choreo2_image.jpg".to_string(),
            dancer_images: vec![
                "static/choreo2_dancer1.jpg".to_string(),
                "static/choreo2_dancer2.jpg".to_string(),
            ],
            description: "Description for Choreography 2".to_string(),
        },
        // Add cases 3-5 following the same pattern...
        _ => ChoreographyData {
            title: format!("Choreo{}", choreo_number),
            choreo_image: "static/default_choreo.jpg".to_string(),
            dancer_images: vec![],
            description: "".to_string(),
        }
    }
}