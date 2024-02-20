use yew::prelude::*;
use crate::choreo_videos;
use yew_router::prelude::{use_navigator, Navigator};
use crate::use_location;
use crate::Route;
use gloo::console::log;
use crate::VideosList;
use crate::loadscreen_video;
use std::borrow::Borrow;



#[function_component(ChoreoVideo)]
pub fn choreographic_videos() -> Html { //<-- change to performance video
    let showdown_videos = choreo_videos(); 
    let showdown_video_index: usize = use_location()
        .and_then(|l| l.state::<usize>().map(|i| *i))
        .unwrap_or(0);

    let showdown_video_index: UseStateHandle<usize> = use_state(|| showdown_video_index);

     // Use state to track whether the video has finished playing
     let has_video_finished = use_state(|| false);

     // Callback for handling the ended event of the video element
     let navigator = use_navigator().unwrap();
     let handle_video_ended = {
         let navigator = navigator.clone();
         let selected_video_index = showdown_video_index; //<-- if not working try type usize
         Callback::from(move |_| {
             // If the video has finished playing, navigate to the load video action
             if **has_video_finished.borrow() {
                 navigator.push(&loadscreen_video());
             }
         })
     };


    let restart_app = Callback::from(move |event: KeyboardEvent| {
        if event.key() == "q" {
            navigator.push(&Route::IntroScreen1);
        }
    });

    html! {
        <div onkeydown={restart_app} tabindex="0">
            <VideosList videos={showdown_videos} current_index={*showdown_video_index}/>
        </div>
    }
}
