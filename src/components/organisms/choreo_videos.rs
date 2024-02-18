use yew::prelude::*;
use crate::VideosList;
use crate::choreo_videos;
use yew_router::prelude::use_navigator;
use crate::Route;


#[function_component(ChoreoVideo)]
pub fn choreographic_videos() -> Html { //<-- change to performance video
    let navigator = use_navigator().unwrap();
    let performance_video = choreo_videos();
    let performance_video_index = use_state(|| 0);

    let restart_app = Callback::from(move |event: KeyboardEvent| {
        if event.key() == "q" {
            navigator.push(&Route::IntroScreen1);
        }
    });


    html! {
        <div onkeydown={restart_app} tabindex="0">
            <VideosList videos={performance_video} current_index={*performance_video_index} />
        </div>
    }
}