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
use crate::components::data::config::ChoreoVideoConfig;
use crate::components::data::config::get_config_path;
use crate::components::data::config::Config;
use wasm_bindgen_futures::spawn_local;
use gloo::console::log;


#[function_component(ChoreoVideo)]
pub fn choreographic_videos() -> Html {
    
    let div_ref = use_focus_div();
    
    //let choreo_videos = choreo_videos();

    // let choreo_video_index: usize = use_location()
    //     .and_then(|l| l.state::<usize>().map(|i| *i))
    //     .unwrap_or(0);

    // let choreo_video_index: UseStateHandle<usize> = use_state(|| choreo_video_index);
        let choreo_videos = use_state(|| Vec::new());
        let loading = use_state(|| true);
    
        // Get initial index from location if available
        let initial_index: usize = use_location()
            .and_then(|l| l.state::<usize>().map(|i| *i))
            .unwrap_or(0);
    
        let current_video_index = use_state(|| initial_index);
    
    
        // Load videos from config when component mounts
        use_effect_with((), {
            let choreo_videos_clone = choreo_videos.clone();
            let loading_clone = loading.clone();
    
            move |_| {
                spawn_local(async move {
                    let config_path = get_config_path();
    
                    match Config::from_file(&config_path).await {
                        Ok(config) => {
                            let videos = config.get_demo_videos();
                            choreo_videos_clone.set(videos);
                            loading_clone.set(false);
                            log!("Loading videos...");
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
            < VideosList videos={choreo_videos} current_index={*choreo_video_index} on_ended={Some(handle_video_ended)} video_class="fullscreenvideo"/>
        </div>
    }
}
