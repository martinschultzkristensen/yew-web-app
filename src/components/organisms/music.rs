//src/components/organisms/music.rs
use yew::prelude::*;

#[function_component(Music)]
pub fn music() -> Html {
    let music_ref = use_node_ref();
    let is_playing = use_state(|| false);

    let start_music = {
        let music_ref = music_ref.clone();
        let is_playing = is_playing.clone(); // clone `is_playing` to modify it within the closure
        Callback::from(move |_: ()| { // specify type as `()`
            if let Some(audio) = music_ref.cast::<web_sys::HtmlAudioElement>() {
                let _ = audio.play().unwrap();
                is_playing.set(true);
            }
        })
    };

    let stop_music = {
        let music_ref = music_ref.clone();
        let is_playing = is_playing.clone(); // clone `is_playing` to modify it within the closure
        Callback::from(move |_: ()| { 
            if let Some(audio) = music_ref.cast::<web_sys::HtmlAudioElement>() {
                let _ = audio.pause(); 
                is_playing.set(false);
            }
        })
    };


    // let div_ref = use_focus_div();

    // Keyboard handler for specific keys
    let handle_keydown = Callback::from(move |event: KeyboardEvent| {
        match event.key().as_str() {
            "x" => start_music.emit(()),  // Press "x" to start music
            "e" => stop_music.emit(()),   // Press "e" to stop music
            "q" => stop_music.emit(()),
            _ => {}
        }
    });
    

    html! {
        <div tabindex="0" onkeydown={handle_keydown}>
            <audio ref={music_ref} src="/static/8bit-menusong-short-ed.aif" autoplay=false loop=true />
        </div>
    }
}