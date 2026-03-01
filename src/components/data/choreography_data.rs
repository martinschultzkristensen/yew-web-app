//src/components/data/choreography_data.rs
use crate::components::atoms::dancer::DancerData;
use crate::components::data::config::Config;
use crate::components::molecules::video_list::VideoType;

const DEFAULT_CHOREO_IMG: &str = "/static/img/default_choreo_img.png";

#[derive(Clone)]
pub struct ChoreographyData {
    pub title: String,
    pub choreo_image: String,
    pub dancers: Vec<DancerData>,
    pub description: String,  // Optional: if you want to add descriptions
    pub videos: Vec<VideoType>,
}


    
pub fn get_choreography_data(config: &Config, choreo_number: usize) -> ChoreographyData {
    let videos = config.load_choreo_videos();
    let dancers_map = config.load_dancers();

    let default_description = "There is not yet a description for this choreography.".to_string();

    let demo_video = config
        .demo_videos
        .list
        .iter()
        .find(|video| video.id == choreo_number);

    let (title, description, choreo_img) = match demo_video {
        Some(video) => (
            video.title.clone(),
            video.description.clone().unwrap_or_else(|| default_description.clone()),
            video.choreo_img.clone().unwrap_or_else(|| DEFAULT_CHOREO_IMG.to_string()),
        ),
        None => (
            "Default Title".to_string(),
            default_description.clone(),
            DEFAULT_CHOREO_IMG.to_string(),
        ),
    };

    ChoreographyData {
        title,
        description,
        choreo_image: choreo_img,
        dancers: dancers_map.get(&choreo_number).cloned().unwrap_or_default(),
        videos,
    }
}

