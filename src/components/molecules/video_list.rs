use crate::Route;
use gloo::utils::history;
use yew::prelude::*;
use yew_router::history::History;
use yew_router::prelude::*;

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
    //  We removed the displayed_id field because we are now dynamically generating it using the get_displayed_id() method in VideoType
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
                <p class="title-center arcadefont">{current_video.get_displayed_id().unwrap_or_default()}</p>
                    <video
                        src={format!("{}", video.url)}
                        autoplay=true
                        loop={should_loop}
                        onended={onended_attr}
                        class={classes!(video_class.clone(), "smallscreenvideo")}
                    />
                </div>
                    <div class="video-info arcadefont">
                        <p>{format!("{}", &demo.title)}</p>
                        <p>{"Duration: "}{&demo.duration}{" seconds"}</p>
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


//rest of code is not used. Soon check and delete!
#[derive(Properties, PartialEq, Clone)]
pub struct SingleVideoPlayerProps {
    pub video: Video, // Using the Video struct here
    pub on_ended: Callback<()>,
}

#[function_component(SingleVideoPlayer)]
pub fn single_video_player(props: &SingleVideoPlayerProps) -> Html {
    let SingleVideoPlayerProps { video, on_ended } = props.clone();

    let video_ended_callback = Callback::from(move |_| {
        // Call the on_ended callback when the video ends
        on_ended.emit(());
    });

    html! {
        <div>
            <video src={format! ("{}", video.url)} autoplay=true onended={video_ended_callback}/>
        </div>
    }
}
