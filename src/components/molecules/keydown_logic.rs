//src/components/molecules/keydown_logic.rs
use crate::components::molecules::video_list::VideoType;
use crate::SoundEffectsContext;
use yew::prelude::*;


// Handle keydown events to switch demo videos in the main menu
pub fn get_toggle_key(
    v: &Vec<VideoType>,
    video_index: UseStateHandle<usize>,
    sound_context: SoundEffectsContext,
) -> Callback<KeyboardEvent> {
    

    let videos = v.clone();
    let play_sound = sound_context.play_sound;

    Callback::from(move |event: KeyboardEvent| {
        if event.key() == "w" || event.key() == "s" {
            play_sound.emit("toggleUpDown".to_string());

            let new_index = match event.key().as_str() {
                "w" => (*video_index + 1) % videos.len(), // Loops back to 0 if at last item
                "s" => if *video_index == 0 {
                            videos.len() - 1
                        } else {
                            *video_index - 1
                        },
                _ => *video_index,
            };

            video_index.set(new_index);
        }
    })
}