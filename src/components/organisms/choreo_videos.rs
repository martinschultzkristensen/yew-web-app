use yew::prelude::*;
use yew_router::navigator;
use crate::choreo_videos;
use crate::components::molecules::video_list::SingleVideoPlayer;
use yew_router::prelude::{use_navigator, Navigator};
use crate::use_location;
use crate::Route;
use gloo::console::log;
use crate::VideosList;
use crate::LoadScreenVideo;
use std::borrow::Borrow;



#[function_component(ChoreoVideo)]
pub fn choreographic_videos() -> Html { //<-- change to performance video
    let navigator = use_navigator().unwrap();
    let choreo_videos = choreo_videos(); 
    let choreo_video_index: usize = use_location()
        .and_then(|l| l.state::<usize>().map(|i| *i))
        .unwrap_or(0);

    let _choreo_video_index: UseStateHandle<usize> = use_state(|| choreo_video_index);

     // Use state to track whether the video has finished playing
     let has_video_finished = use_state(|| false);

     // Callback for handling the ended event of the video element
     let handle_video_ended: Callback<()> = {
        let navigator = navigator.clone();
        let has_video_finished = has_video_finished.clone();
         Callback::from(move |_:()| {
             // If the video has finished playing, navigate to the load video action
             if *has_video_finished {
                 navigator.push(&Route::LoadScreenVideo);
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
            <SingleVideoPlayer video={choreo_videos} on_ended={handle_video_ended.clone()}/>
        </div>
    }
}
