use crate::Route;
use crate::components::atoms::arrow_respnd_ui::*;
use yew::prelude::*;
use yew_router::history::History;
use yew_router::prelude::*;
use crate::components::atoms::dance_o_matic_logo::DanceOMaticLogo;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;



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
                        src={format!("{}", video.url)}
                        autoplay=true
                        loop={should_loop}
                        onended={onended_attr}
                        class={classes!(video_class.clone(), "smallscreenvideo")}
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
                src={format!("{}", video.url)}
                autoplay=true
                loop={should_loop}
                onended={onended_attr}
                class={classes!(video_class.clone(), "fullscreenvideo")}
            />
        },
    }
}
