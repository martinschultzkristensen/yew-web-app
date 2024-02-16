use gloo::utils::history;
use yew::prelude::*;
use yew_router::prelude::*;
use crate::Route;
use yew_router::history::History;

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

    // Obtain the current location, ensuring it implements the History trait
    let location = use_location();

    // Create a callback function to update the URL with the new current_video_index
    let switch_video = Callback::from(move |id: usize| {
        // Update the URL with the new video ID
        let new_url = format!("/main-menu/{}", id); // Modify the URL as needed
        location.path(&new_url);
    });

    html! {
        <div>
            <p>{format!("{}", current_video.title)}</p>
            <video src={format!("{}", current_video.url)} autoplay=true loop=true />
        </div>
    }
}
