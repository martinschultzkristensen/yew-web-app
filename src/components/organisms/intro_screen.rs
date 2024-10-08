
use crate::get_intro_video;
use crate::Route;
use crate::VideosList;
use yew::prelude::*;
use yew_router::prelude::use_navigator;
use crate::components::molecules::video_list::VideosListProps;

#[function_component(IntroScreen)]
pub fn intro_screen(props: &VideosListProps) -> Html {
    let navigator = use_navigator().unwrap();
    let intro_video = get_intro_video();
    let current_video_index = use_state(|| 0);

    //let should_loop = intro_video[*current_video_index].should_loop();
    let onended_attr = match &props.on_ended {
        Some(callback) => Some(callback.clone()), // Clone the Callback if it's Some
        None => None,
    };

    let press_x_for_main = Callback::from(move |event: KeyboardEvent| {
        if event.key() == "x" {
            navigator.push(&Route::MainMenu);
            let audio = web_sys::HtmlAudioElement::new_with_src("static/coinSound.mp3").unwrap();
            let _ = audio.play();
        }
    });

    html! {
        <div onkeydown={press_x_for_main} tabindex="0">
            <VideosList videos={intro_video} current_index={*current_video_index} on_ended={onended_attr}/>
        </div>
    }
}
