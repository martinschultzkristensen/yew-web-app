//lib.rs
//gloo writes stuff to the web console
//use gloo::console::log; use serde::{Serialize, Deserialize}; <-- Uncomment to write to the webconsole
use yew::prelude::*;
use components::molecules::video_list::{VideosList, Video};


mod components;



#[function_component(DanceOmatic)]
pub fn app() -> Html {
    let vec_of_videos = vec![
        Video {
            id: 1,
            title: "dancevideo nr.1 ".to_string(),
            url: "static/AI&Boy.mp4".to_string(),
        },
        Video {
            id: 2,
            title: "dancevideo nr.2".to_string(),
            url: "static/Siblings.mp4".to_string(),
        },
        Video {
            id: 3,
            title: "dancevideo nr.3".to_string(),
            url: "static/Culture4Fun.mp4".to_string(),
        },
        Video {
            id: 4,
            title: "dancevideo nr.4".to_string(),
            url: "static/Hej-Nihao.mp4".to_string(),
        },
    ];
    let videos = vec_of_videos;

    // State to track the index of the currently displayed video
    let current_video_index = use_state(|| 0);

    // Handle keydown events to switch videos
    let handle_keydown = {
        let current_video_index = current_video_index.clone();
        let videos = videos.clone();
        let current_video_index = current_video_index.clone();
        Callback::from(move |event: KeyboardEvent| {
            if event.key() == "ArrowRight" || event.key() == "ArrowLeft" {
                let new_index = match event.key().as_str() {
                    "ArrowRight" => {
                        let next_index = *current_video_index + 1;
                        if next_index >= videos.len() {
                            0
                        } else {
                            next_index
                        }
                    }
                    "ArrowLeft" => {
                        let prev_index = *current_video_index as i32 - 1;
                        if prev_index < 0 {
                            (videos.len() - 1) as usize
                        } else {
                            prev_index as usize
                        }
                    }
                    _ => *current_video_index,
                };
                current_video_index.set(new_index);
            }
        })
    };

    html! {
        <div onkeydown={handle_keydown} tabindex="0">
            <VideosList videos={videos} current_index={*current_video_index} />
            //<img src="static/danceOmatic_logo.png" alt="logo of danceomatic"/>
        </div>
    }
}




