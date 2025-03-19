//src/components/molecules/async_await_choreo_video.rs
use yew::prelude::*;
use crate::components::data::video_data::choreo_videos;
use crate::components::data::config::Config;
use crate::components::data::config::get_config_path;
use crate::VideosList;

#[function_component(ChoreoVideosComponent)]
pub fn choreo_videos_component() -> Html {
    let choreo_videos_state = use_state(|| vec![]);
    let loading = use_state(|| true);

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
    let handle_video_ended = {
        let choreo_video_index = choreo_video_index.clone();
        Callback::from(move |_| {
            let new_index = *choreo_video_index + 1;
            choreo_video_index.set(new_index);
        })
    };

    html! {
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
    }
}
