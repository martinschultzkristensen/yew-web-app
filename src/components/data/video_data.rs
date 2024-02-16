use crate::components::molecules::video_list::Video;

pub fn get_demo_videos() -> Vec<Video> {
    vec![
        Video {
            id: 1,
            title: "dancevideo nr.1".to_string(),
            url: "static/Flash_AI&Boy.mp4".to_string(),
        },
        Video {
            id: 2,
            title: "dancevideo nr.2".to_string(),
            url: "static/Flash_Siblings.mp4".to_string(),
        },
        Video {
            id: 3,
            title: "dancevideo nr.3".to_string(),
            url: "static/Flash_Culture4Fun.mp4".to_string(),
        },
        Video {
            id: 4,
            title: "dancevideo nr.4".to_string(),
            url: "static/Flash_Hej-Nihao.mp4".to_string(),
        },
    ]
}

pub fn get_intro_video() -> Vec<Video> {
    vec![
        Video {
            id: 0,
            title: "IntroVideo".to_string(),
            url: "static/Intro-Movie_2015Horsens.mp4".to_string(),
        },
    ]
}

pub fn choreo_videos() -> Vec<Video> {
    vec![
        Video {
            id: 1,
            title: "dancevideo nr.1".to_string(),
            url: "static/LetsDuet_中文_countdown.mp4".to_string(),
        },
        Video {
            id: 2,
            title: "dancevideo nr.2".to_string(),
            url: "static/siblings_中文_countdown.mp4".to_string(),
        },
        Video {
            id: 3,
            title: "dancevideo nr.3".to_string(),
            url: "static/culture4Fun_中文_countdown.mp4".to_string(),
        },
        Video {
            id: 4,
            title: "dancevideo nr.4".to_string(),
            url: "static/HejNihao_中文_countdown.mp4".to_string(),
        },
    ]
}