use crate::Route;
use crate::VideosList;
use yew::prelude::*;
use yew_router::prelude::use_navigator;
use crate::components::atoms::use_focus_div::use_focus_div;
use crate::Config;
use std::rc::Rc;


#[derive(Properties, PartialEq)]
pub struct ScreenProps {
    pub config: Rc<Config>,
}


#[function_component(LoadScreenVideo)]
pub fn load_screen_video(props: &ScreenProps) -> Html {
    let load_screen = props.config.get_loadscreen_video();

    let current_video_index = use_state(|| 0);
    let div_ref = use_focus_div(); // Hook sets focus on the div when the component mounts.


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
            <VideosList videos={load_screen} current_index={*current_video_index} on_ended={Some(handle_video_ended)} video_class="fullscreenvideo"/>
        </div>
    }
}
