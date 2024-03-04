use yew::prelude::*;
use yew::Callback;

#[derive(Clone, PartialEq)]
pub struct Video {
    pub id: usize,
    pub title: String,
    pub url: String,
}

#[derive(Properties, PartialEq)]
pub struct VideosListProps {
    pub videos: Vec<Video>,
    pub current_index: usize,
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
