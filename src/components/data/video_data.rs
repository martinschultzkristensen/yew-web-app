use crate::components::molecules::video_list::{DemoVideo, Video, VideoType};

pub fn get_demo_videos() -> Vec<VideoType> {
    vec![
        VideoType::Demo(DemoVideo {
            video: Video {
                id: 1,
                url: "static/Flash_AI&Boy.mp4".to_string(),
                loop_video: true,
            },
            title: "dancevideo nr.1".to_string(),
            duration: "30".to_string(), // Add appropriate duration
            displayed_id: "Demo 1".to_string(),
        }),
        VideoType::Demo(DemoVideo {
            video: Video {
                id: 2,
                url: "static/Flash_Siblings.mp4".to_string(),
                loop_video: true,
            },
            title: "dancevideo nr.2".to_string(),
            duration: "25".to_string(), // Add appropriate duration
            displayed_id: "Demo 2".to_string(),
        }),
        // Video {
        //     id: 3,
        //     title: "dancevideo nr.3".to_string(),
        //     url: "static/Flash_Culture4Fun.mp4".to_string(),
        //     loop_video: true,
        // },
        // Video {
        //     id: 4,
        //     title: "dancevideo nr.4".to_string(),
        //     url: "static/Flash_Hej-Nihao.mp4".to_string(),
        //     loop_video: true,
        // },
    ]
}

pub fn get_intro_video() -> Vec<VideoType> {
    vec![
        VideoType::Regular(Video {
        id: 0,
        url: "static/IntroDemoVid_4sec.mp4".to_string(),
        loop_video: true,
    })]
}

pub fn choreo_videos() -> Vec<VideoType> {
    vec![
        VideoType::Regular(Video {
            id: 1,
            url: "static/fodbold_tiny.mp4".to_string(),
            loop_video: false,
        }),
        VideoType::Regular(Video {
            id: 2,
            url: "static/siblings_中文_countdown.mp4".to_string(),
            loop_video: false,
        }),
        VideoType::Regular(Video {
            id: 3,
            url: "static/Hej-Nihao.mp4".to_string(),
            loop_video: false,
        }),
        VideoType::Regular(Video {
            id: 4,
            url: "static/AI&Boy.mp4".to_string(),
            loop_video: false,
        }),
    ]
}

pub fn loadscreen_video() -> Vec<VideoType> {
    vec![
        VideoType::Regular(Video {
        id: 1,
        url: "static/LoadScreenTest4s.mp4".to_string(),
        loop_video: false,
    })]
}
