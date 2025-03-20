//src/components/organisms/choreo_videos.rs
use crate::use_location;
use crate::Route;
use crate::VideosList;
use yew::prelude::*;
use yew_router::prelude::{use_navigator, Navigator};
use crate::components::atoms::use_focus_div::use_focus_div;

use crate::components::data::choreography_data::get_choreography_data;


#[function_component(ChoreoVideo)]
pub fn choreographic_videos() -> Html {
    // State to hold the choreography data and loading status
    let choreo_data_state = use_state(|| None);
    let loading = use_state(|| true);
    let navigator = use_navigator().unwrap();
    let div_ref = use_focus_div();

    // Load choreography data from config when component mounts
    use_effect_with((), {
        let choreo_data_state_clone = choreo_data_state.clone();
        let loading_clone = loading.clone();

        move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                // Assuming you want to load data for choreography number 1
                let choreo_number = 2; //<-- what does this change?c
                let choreo_data = get_choreography_data(choreo_number).await;
                choreo_data_state_clone.set(Some(choreo_data));
                loading_clone.set(false);
                log::info!("Loading choreography data...");
            });

            || ()
        }
    });

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
            if *loading {
                <div class="about-choreo-container">
                    <p>{"Loading..."}</p>
                </div>
            } else if let Some(choreo_data) = &*choreo_data_state {
                <VideosList
                    videos={choreo_data.videos.clone()}
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
    

