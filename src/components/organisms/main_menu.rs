use yew::prelude::*;
use crate::VideosList;
use crate::get_demo_videos;
use crate::get_toggle_key;

use yew_router::prelude::{use_navigator, Navigator};
use crate::Route;

pub fn navigate_to_about(index: usize, nav: Option<Navigator>) {
    if let Some(navigator) = nav {
        match index {
            1 => navigator.push(&Route::AboutChoreo1),
            2 => navigator.push(&Route::AboutChoreo2),
            3 => navigator.push(&Route::AboutChoreo3),
            4 => navigator.push(&Route::AboutChoreo4),
            _ => {}
        }
    } else {
        // Handle the case where nav is None
        // For example, log an error or provide a default action
    }
}

#[function_component(MainMenu)]
pub fn main_menu() -> Html {
    
    let demo_videos = get_demo_videos();
    // State to track the index of the currently displayed demo video
    let current_video_index = use_state(|| 0);
    let current_video_index_ref = &current_video_index;

    
    //Handle keydown events to switch videos
    let handle_keydown_toggle = get_toggle_key(&demo_videos, current_video_index.clone(), navigate_to_about);
    let navigator = use_navigator();

    let press_r_for_about = Callback::from(move |event: KeyboardEvent| {
        if event.key() == "r"{
            navigate_to_about(*current_video_index, navigator);
            let soundeffect = web_sys::HtmlAudioElement::new_with_src("static/buttonClick.mp3").unwrap();
            let _ = soundeffect.play();
        }});
    

        let navigator = use_navigator().unwrap();
        let restart_app = Callback::from(move |event: KeyboardEvent| {
            if event.key() == "q"{
            navigator.push(&Route::IntroScreen1);
            }});
       

    html! { 
        <div onkeydown={restart_app} onkeydown={press_r_for_about} tabindex="0">
            <audio src={format!("static/8bit-menusong-short-ed.aif")} autoplay=true loop=true />
            <div onkeydown={handle_keydown_toggle} tabindex="0">
                <VideosList videos={demo_videos} current_index={*current_video_index} />
                <img src="static/danceOmatic_logo.png" alt="logo of danceomatic"/>
            </div>
        </div>
    }
}