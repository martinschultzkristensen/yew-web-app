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

// pub async fn choreo_videos(config: &Config) -> Vec<VideoType> {
//     config.load_choreo_videos()
// }
// pub fn choreo_videos() -> Vec<VideoType> {
//     vec![
//         VideoType::Regular(Video {
//             id: 1,
//             url: "static/AI&Boy.mp4".to_string(),
//             loop_video: false,
//         }),
//         VideoType::Regular(Video {
//             id: 2,
//             url: "static/Siblings.mp4".to_string(),
//             loop_video: false,
//         }),
//         VideoType::Regular(Video {
//             id: 3,
//             url: "static/Culture4Fun.mp4".to_string(),
//             loop_video: false,
//         }),
//         VideoType::Regular(Video {
//             id: 4,
//             url: "static/fodbold_tiny.mp4".to_string(),
//             loop_video: false,
//         }),
//     ]
// }

pub fn loadscreen_video() -> Vec<VideoType> {
    vec![
        VideoType::Regular(Video {
        id: 1,
        url: "static/LoadScreenTest4s.mp4".to_string(),
        loop_video: false,
    })]
}
