<<<<<<< HEAD
use crate::choreo_videos;
=======
//src/components/organisms/choreo_videos.rs
>>>>>>> feature/config_v2
use crate::use_location;
use crate::Route;
use crate::VideosList;
use yew::prelude::*;
use yew_router::prelude::use_navigator;
use crate::components::atoms::use_focus_div::use_focus_div;
use std::rc::Rc;
use crate::Config;



#[derive(Properties, PartialEq)]
pub struct ChoreoVideoProps {
    pub config: Rc<Config>,
}

#[function_component(ChoreoVideo)]
pub fn choreographic_videos(props: &ChoreoVideoProps) -> Html {
    // State to hold the choreography data and loading status
    let choreo_data_state = use_state(|| props.config.load_choreo_videos());

    let navigator = use_navigator().unwrap();
    let div_ref = use_focus_div();


    let choreo_video_index: usize = use_location()
        .and_then(|l| l.state::<usize>().map(|i| *i))
        .unwrap_or(0);
    let choreo_video_index: UseStateHandle<usize> = use_state(|| choreo_video_index);

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
            if !choreo_data_state.is_empty() {
                <VideosList
                    videos={(*choreo_data_state).clone()}
                    current_index={*choreo_video_index}
                    on_ended={Some(handle_video_ended)}
                    video_class="fullscreenvideo"
                />
            } else {
                <div class="about-choreo-container">
                    <p>{"Failed to load choreography data."}</p>
                </div>
            }
        </div>
    }
}
    

