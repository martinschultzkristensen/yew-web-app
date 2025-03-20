//src/components/organisms/choreo_videos.rs
use crate::choreo_videos;
use crate::components::molecules::video_list::VideosListProps;
use crate::loadscreen_video;
use crate::use_location;
use crate::Route;
use crate::VideosList;
use yew::prelude::*;
use yew_router::prelude::{use_navigator, Navigator};
use crate::components::atoms::use_focus_div::use_focus_div;
use crate::components::data::config::get_config_path;
use crate::components::data::config::Config;


#[function_component(ChoreoVideo)]
pub fn choreographic_videos() -> Html {
    // State to hold the videos and loading status
    let choreo_videos_state = use_state(|| vec![]);
    let loading = use_state(|| true);
    let navigator = use_navigator().unwrap();
    let div_ref = use_focus_div();
    
    // Load videos from config when component mounts
    use_effect_with((), {
        let choreo_videos_state_clone = choreo_videos_state.clone();
        let loading_clone = loading.clone();

        move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                let config_path = get_config_path();

                match Config::from_file(&config_path).await {
                    Ok(config) => {
                        let videos = choreo_videos(&config).await;
                        choreo_videos_state_clone.set(videos);
                        loading_clone.set(false);
                        log::info!("Loading videos...");
                    }
                    Err(e) => {
                        log::error!("Failed to load config: {:?}", e);
                        loading_clone.set(false);
                    }
                }
            });

            || ()
        }
    });

    let choreo_video_index = use_state(|| 0);


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
        } else {
            <VideosList
                videos={(*choreo_videos_state).clone()}
                current_index={*choreo_video_index}
                on_ended={Some(handle_video_ended)}
                video_class="fullscreenvideo"
            />
        }
    </div>
    }
}

    

