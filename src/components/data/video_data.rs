use crate::components::molecules::video_list::Video;

pub fn get_demo_videos() -> Vec<Video> {
    vec![
        Video {
            id: 1,
            title: "dancevideo nr.1".to_string(),
            url: "static/Flash_AI&Boy.mp4".to_string(),
            loop_video: true,
        },
        Video {
            id: 2,
            title: "dancevideo nr.2".to_string(),
            url: "static/Flash_Siblings.mp4".to_string(),
            loop_video: true,
        },
        Video {
            id: 3,
            title: "dancevideo nr.3".to_string(),
            url: "static/Flash_Culture4Fun.mp4".to_string(),
            loop_video: true,
        },
        Video {
            id: 4,
            title: "dancevideo nr.4".to_string(),
            url: "static/Flash_Hej-Nihao.mp4".to_string(),
            loop_video: true,
        },
    ]
}

pub fn get_intro_video() -> Vec<Video> {
    vec![Video {
        id: 0,
        title: "IntroVideo".to_string(),
        url: "static/IntroDemoVid_4sec.mp4".to_string(),
        loop_video: true,
    }]
}

pub fn choreo_videos() -> Vec<Video> {
    vec![
        Video {
            id: 1,
            title: "Performance video nr.1".to_string(),
            url: "static/fodbold_tiny.mp4".to_string(),
            loop_video: false,
        },
        Video {
            id: 2,
            title: "Performance video nr.2".to_string(),
            url: "static/siblings_中文_countdown.mp4".to_string(),
            loop_video: false,
        },
        Video {
            id: 3,
            title: "TEST VIDEO 3 CHOREO".to_string(),
            url: "static/Hej-Nihao.mp4".to_string(),
            loop_video: false,
        },
        Video {
            id: 4,
            title: "TEST CHOREO VIDEO 4".to_string(), //<-- update string before registering videoUpdate
            url: "static/AI&Boy.mp4".to_string(),
            loop_video: false,
        },
    ]
}

pub fn loadscreen_video() -> Vec<Video> {
    vec![Video {
        id: 1,
        title: "Load Video".to_string(),
        url: "static/LoadScreenTest4s.mp4".to_string(),
        loop_video: false,
    }]
}
