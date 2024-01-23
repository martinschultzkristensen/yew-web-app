//lib.rs
//gloo writes stuff to the web console
//use gloo::console::log; use serde::{Serialize, Deserialize}; <-- Uncomment to write to the webconsole
use yew::prelude::*;
use components::molecules::video_list::VideosList;
use components::data::video_data::*;
use components::organisms::keydown_logic::get_toggle_key;


mod components;



#[function_component(DanceOmatic)]
pub fn app() -> Html {
    let videos = get_demo_videos();
    // State to track the index of the currently displayed demo video
    let current_video_index = use_state(|| 0);
    
    

    // Handle keydown events to switch videos

    let handle_keydown_toggle = get_toggle_key(&videos, current_video_index.clone());

    html! {        
        <div onkeydown={handle_keydown_toggle} tabindex="0">
            <VideosList videos={videos} current_index={*current_video_index} />
            <img src="static/danceOmatic_logo.png" alt="logo of danceomatic"/>
        </div>
    }
}




