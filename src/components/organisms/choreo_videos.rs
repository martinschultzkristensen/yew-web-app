use crate::choreo_videos;
use yew_router::prelude::{use_navigator, Navigator};
use crate::use_location;
use crate::Route;
use gloo::console::log;


#[function_component(ChoreoVideo)]
pub fn choreographic_videos() -> Html { //<-- change to performance video
    let showdown_videos = choreo_videos(); 
    let showdown_video_index: usize = use_location()
        .and_then(|l| l.state::<usize>().map(|i| *i))
        .unwrap_or(0);

    let current_video_index: UseStateHandle<usize> = use_state(|| showdown_video_index);
    let navigator = use_navigator();

    pub fn navigate_to_about(index: usize, navigator: Option<Navigator>) -> usize {
        if let Some(navigator) = navigator {
            match index {
                0 => navigator.push(&Route::AboutChoreo1),
                1 => navigator.push(&Route::AboutChoreo2),
                2 => navigator.push(&Route::AboutChoreo3),
                3 => navigator.push(&Route::AboutChoreo4),
                _ => {}
            }
        } else {
            log!("Navigator is None");
        }
        index
    }

    let navigator = use_navigator().unwrap();
    let restart_app = Callback::from(move |event: KeyboardEvent| {
        if event.key() == "q" {
            navigator.push(&Route::IntroScreen1);
        }
    });

    html! {
        <div onkeydown={restart_app} tabindex="0">
            <VideosList videos={showdown_videos} current_index={showdown_video_index}/>
        </div>
    }
}
