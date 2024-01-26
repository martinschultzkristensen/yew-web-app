use yew::prelude::*;
use crate::VideosList;
use crate::get_intro_video;
use yew_router::prelude::use_navigator;
use crate::Route;


#[function_component(IntroScreen)]
pub fn intro_screen() -> Html {
    let navigator = use_navigator().unwrap();
    let intro_video = get_intro_video();
    let current_video_index = use_state(|| 0);

    let press_x_for_main = Callback::from(move |event: KeyboardEvent| {
        if event.key() == "x"{
        navigator.push(&Route::MainMenu);
        }});


    html! {
        <div onkeydown={press_x_for_main} tabindex="0">
            <VideosList videos={intro_video} current_index={*current_video_index} />
        </div>
    }
}
