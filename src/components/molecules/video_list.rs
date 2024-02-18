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
}

#[derive(Properties, PartialEq)]
pub struct VideosListProps {
    pub videos: Vec<Video>,
    pub current_index: usize, // New property for the current index
}

#[function_component(VideosList)]
pub fn videos_list(
    VideosListProps {
        videos,
        current_index,
    }: &VideosListProps,
) -> Html {
    // Use the current_index to display the corresponding video
    let current_video = &videos[*current_index];

    html! {
        <div>
            <p>{format!("{}", current_video.title)}</p>
            <video src={format!("{}", current_video.url)} autoplay=true loop=true />
        </div>
    }
}
