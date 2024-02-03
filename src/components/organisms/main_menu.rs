use yew::prelude::*;
use crate::VideosList;
use crate::get_demo_videos;
use crate::get_toggle_key;



#[function_component(MainMenu)]
pub fn main_menu() -> Html {
   

    let demo_videos = get_demo_videos();
    // State to track the index of the currently displayed demo video
    let current_video_index = use_state(|| 0);

    //Handle keydown events to switch videos
    let handle_keydown_toggle = get_toggle_key(&demo_videos, current_video_index.clone());

    html! { 
        <>
            <div>
                <audio src={format!("static/8bit-menusong-short-ed.aif")} autoplay=true loop=true />
            </div>
            <div onkeydown={handle_keydown_toggle} tabindex="0">
                <VideosList videos={demo_videos} current_index={*current_video_index} />
                <img src="static/danceOmatic_logo.png" alt="logo of danceomatic"/>
            </div>
        </>  
    }
}