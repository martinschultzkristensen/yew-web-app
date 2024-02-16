use crate::components::molecules::video_list::Video;
use yew::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;


pub fn get_toggle_key(v: &Vec<Video>, video_index: &usize) -> Callback<KeyboardEvent> {
    let videos = v.clone();
    let current_video_index = video_index.clone();

    Callback::from(move |event: KeyboardEvent| {
        if event.key() == "w" || event.key() == "s" {
            let mut current_index = current_video_index;
            let new_index = match event.key().as_str() {
                "w" => {
                    let next_index = current_index + 1;
                    if next_index >= videos.len() {
                        0
                    } else {
                        next_index
                    }
                }
                "s" => {
                    let prev_index = current_index as i32 - 1;
                    if prev_index < 0 {
                        (videos.len() - 1) as usize
                    } else {
                        prev_index as usize
                    }
                }
                _ => current_index,
            };
            current_index = new_index;
            let audio = web_sys::HtmlAudioElement::new_with_src("static/button-124476.mp3").unwrap();
            let _ = audio.play();
        }
    })
}
