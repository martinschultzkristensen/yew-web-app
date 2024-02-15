use crate::get_demo_videos;
use crate::get_toggle_key;
use gloo::console::log;
use crate::VideosList;
use yew::prelude::*;
use crate::Route;
use yew_router::prelude::{use_navigator, Navigator};



#[function_component(MainMenu)]
pub fn main_menu() -> Html {
    let demo_videos = get_demo_videos();
    let current_video_index = use_state(|| 0);  // State to track the index of the currently displayed demo video
    let navigator = use_navigator();
    
    pub fn navigate_to_about(index: usize, navigator: Option<Navigator>, current_video_index: &UseStateHandle<usize>) {
        if let Some(navigator) = navigator {
            let current_video_index = current_video_index.clone(); // Clone the reference for the closure
            let new_index = match index {
                0 => {
                    navigator.push_with_state(&Route::AboutChoreo1, 0); // Pass the index to AboutChoreo1
                    1 // Set new index to 1
                },
                1 => {
                    navigator.push_with_state(&Route::AboutChoreo2, 1); // Pass the index to AboutChoreo2
                    1 // Set new index to 2
                },
                2 => {
                    navigator.push_with_state(&Route::AboutChoreo3, 3); // Pass the index to AboutChoreo3
                    3 // Set new index to 3
                },
                3 => {
                    navigator.push_with_state(&Route::AboutChoreo4, 4); // Pass the index to AboutChoreo4
                    4 // Set new index to 4
                },
                _ => return, // Return early if index is invalid
            };
            
            current_video_index.set(new_index)
        } else {
            log!("Navigator is None");
        }
    }
    
    
    let handle_keydown_toggle =
        get_toggle_key(&demo_videos, current_video_index.clone());

        let current_video_index_clone = current_video_index.clone();
        let press_r_for_about = Callback::from(move |event: KeyboardEvent| {
            if event.key() == "r" {
                navigate_to_about(*current_video_index_clone, navigator.clone(), &current_video_index_clone);
                
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


