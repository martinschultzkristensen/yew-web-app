use crate::loadscreen_video;
use crate::Route;
use crate::VideosList;
use yew::prelude::*;
use yew_router::prelude::use_navigator;
use crate::components::atoms::use_focus_div::use_focus_div;

#[function_component(LoadScreenVideo)]
pub fn load_video() -> Html {
    let navigator = use_navigator().unwrap();
    let load_screen = loadscreen_video();
    let current_video_index = use_state(|| 0);
    let div_ref = use_focus_div(); // Hook sets focus on the div when the component mounts.

    let press_x_for_main = Callback::from(move |event: KeyboardEvent| {
        if event.key() == "x" {
            navigator.push(&Route::MainMenu);
            let audio = web_sys::HtmlAudioElement::new_with_src("static/coinSound.mp3").unwrap();
            let _ = audio.play();
        }
    });
    let navigator = use_navigator().unwrap();
    let handle_video_ended = {
        let navigator = navigator.clone();
        Callback::from(move |_| {
            navigator.push(&Route::IntroScreen1);
        })
    };

    let restart_app = Callback::from(move |event: KeyboardEvent| {
        if event.key() == "q" {
            navigator.push(&Route::IntroScreen1);
        }
    });

    html! {
        <div ref={div_ref} onkeydown={restart_app} tabindex="0">
            <VideosList videos={load_screen} current_index={*current_video_index} on_ended={Some(handle_video_ended)}/>
        </div>
    }
}
