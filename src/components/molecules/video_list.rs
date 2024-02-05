use yew::prelude::*;
use std::cell::RefCell;

#[derive(Clone, PartialEq)]
pub struct Video {
    pub id: usize,
    pub title: String,
    pub url: String,
}

#[derive(Properties, PartialEq)]
pub struct VideosListProps {
    pub videos: Vec<Video>,
    pub current_index: yew::UseStateHandle<RefCell<usize>>, // New property for the current index
}

#[function_component(VideosList)]
pub fn videos_list(props: &VideosListProps,) -> Html {
    // Use the current_index to display the corresponding video
    let current_index = *props.current_index.borrow();
    let current_video = &props.videos[current_index];

    html! {
        <div>
            <p>{format!("{}", current_video.title)}</p>
            <video src={format!("{}", current_video.url)} autoplay=true loop=true />
        </div>
    }
} 