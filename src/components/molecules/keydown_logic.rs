use crate::components::molecules::video_list::VideoType;
use yew::prelude::*;

// Handle keydown events to switch demo videos in the main menu
pub fn get_toggle_key(
    v: &Vec<VideoType>,
    video_index: UseStateHandle<usize>,
) -> Callback<KeyboardEvent> {
    
    let videos = v.clone();
    let current_video_index = video_index;

    Callback::from(move |event: KeyboardEvent| {
        if event.key() == "w" || event.key() == "s" {
            let current_index = current_video_index.clone();
            let new_index = match event.key().as_str() {
                "w" => {
                    let next_index = *current_index + 1;
                    if next_index >= videos.len() {
                        0
                    } else {
                        next_index
                    }
                }
                "s" => {
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