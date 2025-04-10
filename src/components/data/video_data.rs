//src/components/data/video_data.rs
use crate::components::molecules::video_list::{Video, VideoType};


pub fn get_intro_video() -> Vec<VideoType> {
    vec![
        VideoType::Regular(Video {
        id: 0,
        url: "static/IntroDemoVid_4sec.mp4".to_string(),
        loop_video: true,
    })]
}


pub fn loadscreen_video() -> Vec<VideoType> {
    vec![
        VideoType::Regular(Video {
        id: 1,
        url: "static/LoadScreenTest4s.mp4".to_string(),
        loop_video: false,
    })]
}
