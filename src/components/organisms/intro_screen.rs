
use crate::get_intro_video;
use crate::Route;
use crate::VideosList;
use yew::prelude::*;
use yew_router::prelude::use_navigator;
use crate::components::atoms::use_focus_div::use_focus_div;
use crate::components::molecules::video_list::VideosListProps;

#[function_component(IntroScreen)]
pub fn intro_screen() -> Html {
    let navigator = use_navigator().unwrap();
    let intro_video = get_intro_video();
    let current_video_index = use_state(|| 0);
    let div_ref = use_focus_div(); // Hook sets focus on the div when the component mounts.


    let handle_video_ended = {
        let navigator = navigator.clone();
        Callback::from(move |_| {
            navigator.push(&Route::IntroScreen1);
        })
    };

    let press_x_for_main = Callback::from(move |event: KeyboardEvent| {
        if event.key() == "x" {
            let audio = web_sys::HtmlAudioElement::new_with_src("static/coinSound.mp3").unwrap();
            let _ = audio.play();
            navigator.push(&Route::MainMenu);
        }
    });

    html! {
        <div ref={div_ref} onkeydown={press_x_for_main} tabindex="0">
            <VideosList videos={intro_video} current_index={*current_video_index} on_ended={Some(handle_video_ended)} video_class="fullscreenvideo"/>
        </div>
    }
}
