use crate::components::molecules::video_list::VideoType;
use web_sys::HtmlElement;
use yew::NodeRef;
use yew::prelude::*;

// Handle keydown events to switch demo videos in the main menu
pub fn get_toggle_key(
    v: &Vec<VideoType>,
    video_index: UseStateHandle<usize>,
        arrow_size: UseStateHandle<u32>,
) -> Callback<KeyboardEvent> {
    
    let videos = v.clone();
    let current_video_index = video_index;
    let audio = web_sys::HtmlAudioElement::new_with_src("/static/button-124476.mp3").unwrap();

    Callback::from(move |event: KeyboardEvent| {
        if event.key() == "w" || event.key() == "s" {
            let _ = audio.play();
            let current_index = current_video_index.clone();
            let new_index = match event.key().as_str() {
                "w" => {
                    arrow_size.set((*arrow_size + 5).min(100));
                    let next_index = *current_index + 1;
                    if next_index >= videos.len() {
                        0
                    } else {
                        next_index
                    }
                }
                "s" => {
                    arrow_size.set((*arrow_size - 5).max(10));
                    let prev_index = *current_index as i32 - 1;
                    if prev_index < 0 {
                        (videos.len() - 1) as usize
                    } else {
                        prev_index as usize
                    }
                }
                _ => *current_index,
            };
            //callback.emit(new_index);
            current_index.set(new_index);
            
        }
    })
}