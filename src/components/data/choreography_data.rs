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

// Function to get choreography data
pub fn get_choreography_data(choreo_number: usize) -> ChoreographyData {
    let martin = Dancer {
        image: "/static/img/Martinus.png".to_string(),
        name: "Martin".to_string(),
        strength: 10,
        flexibility: 5,
    };
    let alex = Dancer {
        image: "/static/img/Jon.png".to_string(),
        name: "Alex".to_string(),
        strength: 8,
        flexibility: 7,
    };
    let mengting = Dancer {
        image: "/static/img/Mengting.png".to_string(),
        name: "Meng-Ting".to_string(),
        strength: 5,
        flexibility: 10,
    };
    let mengyuan = Dancer {
        image: "/static/img/Mengyuin.png".to_string(),
        name: "Meng-Yuan".to_string(),
        strength: 8,
        flexibility: 7,
    };
    match choreo_number {
        1 => ChoreographyData {
            title: match &get_demo_videos()[0] {
                VideoType::Demo(demo) => demo.title.clone(), //is borrow possible? -research
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
                _ => "Choreo 2".to_string(),
            },
            choreo_image: "static/choreo2_image.jpg".to_string(),
            dancers: vec![martin.clone(), mengting.clone(), mengyuan.clone()],
            description: "Description for 3rd dancepiece. Very long just enjoy".to_string(),
        },
        4 => ChoreographyData {
            title: match &get_demo_videos()[3] {
                VideoType::Demo(demo) => demo.title.clone(),
                _ => "Choreo 2".to_string(),
            },
            choreo_image: "static/choreo2_image.jpg".to_string(),
            dancers: vec![martin.clone(), mengting.clone(), mengyuan.clone()],
            description: "Very long Description for Choreography 4".to_string(),
        },
        // Add cases 3-5 following the same pattern...
        _ => ChoreographyData {
            title: format!("Choreo{}", choreo_number),
            choreo_image: "static/default_choreo.jpg".to_string(),
            dancers: vec![],
            description: "".to_string(),
        }
    }
}