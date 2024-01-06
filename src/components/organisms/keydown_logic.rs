use crate::components::molecules::video_list::{self, Video};
use yew::prelude::*;



// Handle keydown events to switch videos
pub fn get_toggle_key(v: Vec<Video>) -> Callback<KeyboardEvent> {
    let videos = v;
    let videos = videos.clone();
    let current_video_index = use_state(|| 0);
    let current_video_index = current_video_index.clone();
    Callback::from(move |event: KeyboardEvent| {
        if event.key() == "w" || event.key() == "s" {
            let new_index = match event.key().as_str() {
                "w" => {
                    let next_index = *current_video_index + 1;
                    if next_index >= videos.len() {
                        0
                    } else {
                        next_index
                    }
                }
                "s" => {
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
            let audio = web_sys::HtmlAudioElement::new_with_src("static/button-124476.mp3").unwrap();
            let _ = audio.play();
        }
    })
}