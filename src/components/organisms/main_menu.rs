use crate::get_demo_videos;
use crate::get_toggle_key;

use crate::VideosList;
use gloo::console::log;
use yew::prelude::*;
use crate::Route;
use yew_router::prelude::{use_navigator, Navigator};


#[derive(Properties, Clone, PartialEq)]
pub struct MainMenuProps {
    pub current_index: usize,
}


#[function_component(MainMenu)]
pub fn main_menu() -> Html {
    let demo_videos = get_demo_videos();
    // State to track the index of the currently displayed demo video
    let current_video_index = use_state(|| 0);
    let navigator = use_navigator();
    
    pub fn navigate_to_about(index: usize, navigator: Option<Navigator>) -> usize {
        if let Some(navigator) = navigator {
            match index {
                0 => navigator.push(&Route::AboutChoreo1),
                1 => navigator.push(&Route::AboutChoreo2),
                2 => navigator.push(&Route::AboutChoreo3),
                3 => navigator.push(&Route::AboutChoreo4),
                _ => {}
            }
        } else {
            log!("Navigator is None");
        }
        index
    }
    
    let handle_keydown_toggle =
        get_toggle_key(&demo_videos, current_video_index.clone());

        let current_video_index_clone = current_video_index.clone();
        let press_r_for_about = Callback::from(move |event: KeyboardEvent| {
            if event.key() == "r" {
                navigate_to_about(*current_video_index_clone, navigator.clone());
                
                let soundeffect = web_sys::HtmlAudioElement::new_with_src("static/buttonClick.mp3").unwrap();
                let _ = soundeffect.play();
            }
        });

    let navigator = use_navigator().unwrap();
    let restart_app = Callback::from(move |event: KeyboardEvent| {
        if event.key() == "q" {
            navigator.push(&Route::IntroScreen1);
        }
    });

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


