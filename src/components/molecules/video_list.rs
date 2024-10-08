use crate::Route;
use gloo::utils::history;
use yew::prelude::*;
use yew_router::history::History;
use yew_router::prelude::*;

#[derive(Clone, PartialEq)]
pub struct Video {
    pub id: usize,
    pub title: String,
    pub url: String,
    pub loop_video: bool, // boolean field indicates if the video should loop
}

impl Video {
    // Method to check if the video should loop
    pub fn should_loop(&self) -> bool {
        self.loop_video
    }
}

#[derive(Properties, PartialEq)]
pub struct VideosListProps {
    pub videos: Vec<Video>,
    pub current_index: usize, // New property for the current index
    pub on_ended: Option<Callback<()>>,
}

#[function_component(VideosList)]
pub fn videos_list(props: &VideosListProps) -> Html {
    let VideosListProps {
        videos,
        current_index,
        on_ended,
    } = props;

    let should_loop = videos[*current_index].should_loop();

    let onended_attr = if !should_loop {
        on_ended.clone().map(|callback| {
            Callback::from(move |_| {
                callback.emit(());
            })
        })
    } else {
        None
    };
    let current_video = &videos[*current_index]; // <- get current_index to display the corresponding video

    html! {
        <div>
            <p>{format!("{}", current_video.title)}</p>
            <video src={format!("{}", current_video.url)} autoplay=true loop=true onended={onended_attr}/>
        </div>
    }
}

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
            <p>{&video.title}</p>
            <video src={format! ("{}", video.url)} autoplay=true onended={video_ended_callback} />
        </div>
    }
}
