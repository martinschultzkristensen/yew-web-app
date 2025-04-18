//src/components/molecules/video_list.rs
use crate::components::atoms::arrow_respnd_ui::*;
use yew::prelude::*;
use serde_json::json;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;
use serde_wasm_bindgen::{to_value, from_value};

#[derive(Clone, PartialEq)]
pub struct Video {
    pub id: usize,
    pub url: String,
    pub loop_video: bool, // boolean field indicates if the video should loop
}

#[derive(Clone, PartialEq)]
pub struct DemoVideo {
    pub video: Video,
    pub title: String,
    pub duration: String,
    pub description: String,
    pub choreo_img: String,
}

#[derive(Clone, PartialEq)]
pub enum VideoType {
    Regular(Video),
    Demo(DemoVideo),
}

impl VideoType {
    pub fn get_displayed_id(&self) -> Option<String> {
        match self {
            VideoType::Demo(demo) => Some(format!("NR. {}", demo.video.id)), // Generate based on video.id
            _ => None, // Regular videos don't have a displayed_id
        }
    }
    pub fn get_video(&self) -> &Video {
        match self {
            VideoType::Regular(v) => v,
            VideoType::Demo(d) => &d.video,
        }
    }
    // Method to check if the video should loop
    pub fn should_loop(&self) -> bool {
        self.get_video().loop_video
    }
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"], js_name = invoke)]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Properties, PartialEq)]
pub struct VideosListProps {
    pub videos: Vec<VideoType>,
    pub current_index: usize, // New property for the current index
    pub on_ended: Option<Callback<()>>,
    #[prop_or_default]
    pub video_class: String, // currently used to determine which fond-styling is used in index.scss
}

#[function_component(VideosList)]
pub fn videos_list(props: &VideosListProps) -> Html {
    
    use_effect(|| {
        web_sys::console::log_1(&"VideosList mounted".into());
        || ()
    });

    let VideosListProps {
        videos,
        current_index,
        on_ended,
        video_class,
    } = props;

    let current_video = &videos[*current_index]; // <- get current_index to display the corresponding video. Access the inner Video with .video
    let video = current_video.get_video();
    let should_loop = current_video.should_loop();
    let onended_attr = if !should_loop {
        on_ended.clone().map(|callback| {
            Callback::from(move |_| {
                callback.emit(());
            })
        })
    } else {
        None
    };
   
    // 🧠 Start: Resolve path using Tauri backend
    let video_src = use_state(|| None);
    {
        let video_src = video_src.clone();
        let video_name = current_video.get_video().url.clone(); // e.g. "static/devVideo/DEMO_LetsDuet.mp4"

        use_effect_with(video_name.clone(), move |video_name| {
            wasm_bindgen_futures::spawn_local({
                let video_src = video_src.clone();
                let video_name = video_name.clone();
                async move {
                    let js_args = serde_wasm_bindgen::to_value(&json!({ "videoName": video_name })).unwrap();
                    let result = invoke("get_video_path", js_args).await;
                    match serde_wasm_bindgen::from_value::<String>(result) {
                        Ok(path) => {
                            log::info!("🎥 Video path resolved: {}", path);
                            video_src.set(Some(path));
                        }
                        Err(err) => log::error!("❌ Failed to parse video path: {:?}", err),
                    }
                }
            });
            || ()
        });

}

    match current_video {

        VideoType::Demo(demo) => html! {
        <div class="main_menu-container">
                <div class="video-wrapper">
                    <div class="svg-arrow-in-main">
                    <ArrowUpIcon/>
                    </div>

                <p class="title-center arcadefont">{current_video.get_displayed_id().unwrap_or_default()}</p>
                    <div class="video-placeholder">
                    <video
                        src={format!("/{url}", url = video.url)}
                        autoplay=true
                        loop={should_loop}
                        onended={onended_attr}
                        class={classes!(video_class.clone(), "smallscreenvideo")}
                        preload="auto"
                    />
                    </div>
                    <div class="svg-arrow-in-main">
                    <ArrowDownIcon/>
                    </div>

                    
                </div>
                <div class="right-column">
                <div class="video-info arcadefont">
                    <h4>{format!("{}", &demo.title)}</h4>
                    <h4>{"Duration: "}{&demo.duration}{" seconds"}</h4>
                </div>
            </div>
            </div>
        },
        VideoType::Regular(_) => html! {
            <video
                src={format!("/{url}", url = video.url)}
                autoplay=true
                loop={should_loop}
                onended={onended_attr}
                class={classes!(video_class.clone(), "fullscreenvideo")}
                preload="auto"
            />
        },
    }
}
