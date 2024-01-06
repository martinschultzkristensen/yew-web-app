//lib.rs
//gloo writes stuff to the web console
//use gloo::console::log; use serde::{Serialize, Deserialize}; <-- Uncomment to write to the webconsole
use yew::prelude::*;
use components::molecules::video_list::VideosList;
use components::data::video_data::*;


mod components;



#[function_component(DanceOmatic)]
pub fn app() -> Html {
    let videos = get_demo_videos();
    // State to track the index of the currently displayed demo video
    let current_video_index = use_state(|| 0);
    
    

    // Handle keydown events to switch videos
    let handle_keydown_toggle = {
        let videos = videos.clone();
        let current_video_index = current_video_index.clone();
        Callback::from(move |event: KeyboardEvent| {
            if event.key() == "w" || event.key() == "s" {
                let new_index = match event.key().as_str() {
                    "w" => {
                        let next_index = *current_video_index + 1;
                        if next_index >= videos.len() {
                            0
                        } else {
                            next_index
                        }
                    }
                    "s" => {
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
                let audio = web_sys::HtmlAudioElement::new_with_src("static/button-124476.mp3").unwrap();
                let _ = audio.play();
            }
        })
    };

    html! {        
        <div onkeydown={handle_keydown_toggle} tabindex="0">
            <VideosList videos={videos} current_index={*current_video_index} />
            //<img src="static/danceOmatic_logo.png" alt="logo of danceomatic"/>
        </div>
    }
}




