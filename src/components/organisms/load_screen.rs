use crate::loadscreen_video;
use crate::Route;
use crate::VideosList;
use yew::prelude::*;
use yew_router::prelude::use_navigator;

#[function_component(LoadScreenVideo)]
pub fn load_video() -> Html {
    let navigator = use_navigator().unwrap();
    let load_screen = loadscreen_video();
    let current_video_index = use_state(|| 0);

    let press_x_for_main = Callback::from(move |event: KeyboardEvent| {
        if event.key() == "x" {
            navigator.push(&Route::MainMenu);
            let audio = web_sys::HtmlAudioElement::new_with_src("static/coinSound.mp3").unwrap();
            let _ = audio.play();
        }
    });

    html! {
        <div onkeydown={press_x_for_main} tabindex="0">
            <VideosList videos={load_screen} current_index={*current_video_index} />
        </div>
    }
}
