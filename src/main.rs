

use yew::prelude::*;



#[derive(Clone, PartialEq)]
struct Video {
    id: usize,
    title: String,
    url: String,
}

#[derive(Properties, PartialEq)]
struct VideosListProps {
    videos: Vec<Video>,
    current_index: usize, // New property for the current index
}

#[function_component(VideosList)]
fn videos_list(
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

#[function_component]
fn App() -> Html {
    let vec_of_videos = vec![
        Video {
            id: 1,
            title: "dancevideo nr.1 ".to_string(),
            url: "static/AI&Boy.mp4".to_string(),
        },
        Video {
            id: 2,
            title: "dancevideo nr.2".to_string(),
            url: "static/Siblings.mp4".to_string(),
        },
        Video {
            id: 3,
            title: "dancevideo nr.3".to_string(),
            url: "static/Culture4Fun.mp4".to_string(),
        },
        Video {
            id: 4,
            title: "dancevideo nr.4".to_string(),
            url: "static/Hej-Nihao.mp4".to_string(),
        },
    ];
    let videos = vec_of_videos;

    // State to track the index of the currently displayed video
    let current_video_index = use_state(|| 0);

    // Handle keydown events to switch videos
    let handle_keydown = {
        let current_video_index = current_video_index.clone();
        let videos = videos.clone();
        let current_video_index = current_video_index.clone();
        Callback::from(move |event: KeyboardEvent| {
            if event.key() == "ArrowRight" || event.key() == "ArrowLeft" {
                let new_index = match event.key().as_str() {
                    "ArrowRight" => {
                        let next_index = *current_video_index + 1;
                        if next_index >= videos.len() {
                            0
                        } else {
                            next_index
                        }
                    }
                    "ArrowLeft" => {
                        let prev_index = *current_video_index as i32 - 1;
                        if prev_index < 0 {
                            (videos.len() - 1) as usize
                        } else {
                            prev_index as usize
                        }
                    }
                    _ => *current_video_index,
                };
                current_video_index.set(new_index);
            }
        })
    };

    html! {
        <div onkeydown={handle_keydown} tabindex="0">
            <VideosList videos={videos} current_index={*current_video_index} />
            <img src="static/danceOmatic_logo.png" alt="logo of danceomatic"/>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
