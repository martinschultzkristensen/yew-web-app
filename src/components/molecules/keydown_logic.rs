//src/components/molecules/keydown_logic.rs
use crate::components::molecules::video_list::VideoType;
use crate::SoundEffectsContext;
use yew::prelude::*;

// Handle keydown events to switch demo videos in the main menu and play sound effects
pub fn get_toggle_key(
    v: &Vec<VideoType>,
    video_index: UseStateHandle<usize>,
    sound_context: SoundEffectsContext,
) -> Callback<KeyboardEvent> {
    let videos = v.clone();
    let play_sound = sound_context.play_sound;

    Callback::from(move |event: KeyboardEvent| {
        if event.key() == "w" || event.key() == "s" {
            // OS keyboard auto-repeat re-fires keydown continuously (often 20-30Hz)
            // while a key is held, with no debounce here that would otherwise stop
            // it - each repeat created a brand new overlapping AudioBufferSourceNode
            // for the same sound. Holding W/S while browsing the menu could stack
            // dozens of simultaneous copies of button-124476.mp3, summing well past
            // 0dBFS and clipping hard regardless of any single-voice gain reduction -
            // a strong candidate for the distortion seen throughout the main menu.
            if !event.repeat() {
                play_sound.emit("button-124476.mp3".to_string());
            }

            let new_index = match event.key().as_str() {
                "w" => (*video_index + 1) % videos.len(), // Loops back to 0 if at last item
                "s" => {
                    if *video_index == 0 {
                        videos.len() - 1
                    } else {
                        *video_index - 1
                    }
                }
                _ => *video_index,
            };

            video_index.set(new_index);
        }
    })
}
