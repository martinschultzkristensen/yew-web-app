use crate::choreo_videos;
use crate::components::molecules::video_list::VideosListProps;
use crate::loadscreen_video;
use crate::use_location;
use crate::Route;
use crate::VideosList;
use yew::prelude::*;
use yew_router::prelude::{use_navigator, Navigator};
use crate::components::atoms::use_focus_div::use_focus_div;

#[function_component(ChoreoVideo)]
pub fn choreographic_videos() -> Html {
    //<-- change to performance video
    let choreo_videos = choreo_videos();
    let div_ref = use_focus_div();
    let choreo_video_index: usize = use_location()
        .and_then(|l| l.state::<usize>().map(|i| *i))
        .unwrap_or(0);

    let choreo_video_index: UseStateHandle<usize> = use_state(|| choreo_video_index);
    let navigator = use_navigator().unwrap();

    // Callback for handling the ended event of the video element
    let handle_video_ended = {
        let navigator = navigator.clone();
        Callback::from(move |_| {
            navigator.push(&Route::LoadScreenVideo);
        })
    };

    let restart_app = Callback::from(move |event: KeyboardEvent| {
        if event.key() == "q" {
            navigator.push(&Route::IntroScreen1);
        }
    });

    html! {
        <div ref={div_ref} onkeydown={restart_app} tabindex="0">
            < VideosList videos={choreo_videos} current_index={*choreo_video_index} on_ended={Some(handle_video_ended)} /> // Ensure you pass the callback here
        </div>
    }
}
