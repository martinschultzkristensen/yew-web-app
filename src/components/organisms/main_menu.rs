//src/components/organisms/main_menu.rs
use crate::components::atoms::dance_o_matic_logo::DanceOMaticLogo;
use crate::components::atoms::use_focus_div::use_focus_div;
use crate::components::data::config::get_config_path;
use crate::components::data::config::Config;
use crate::components::molecules::btn_explainer_graphics::BtnExplainerGraphics;
use crate::components::molecules::music_context::*;
use crate::components::molecules::sound_effects::*;
use crate::get_toggle_key;
use crate::use_location;
use crate::Route;
use crate::VideosList;
use std::rc::Rc;
use gloo::console::log;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_router::prelude::{use_navigator, Navigator};

#[function_component(MainMenu)]
pub fn main_menu() -> Html {
    let ctx = use_context::<MusicContext>().expect("No music context provider");
   
    let sound_context = use_context::<SoundEffectsContext>().expect("SoundEffectsContext not found");
    let play_sound = sound_context.play_sound.clone();
    


    use_effect_with((), {
        let start_music = ctx.start_music.clone();
        move |_| {
            // Use the start_music callback from the context directly
            start_music.emit(());
            || ()
        }
    });

    
    let div_ref = use_focus_div(); // Hook sets focus on the div (in this case demo video) when the component mounts.
    let demo_videos = use_state(|| Vec::new());
    let loading = use_state(|| true);

    // Get initial index from location if available
    let initial_index: usize = use_location()
        .and_then(|l| l.state::<usize>().map(|i| *i))
        .unwrap_or(0);

    let current_video_index = use_state(|| initial_index);


    // Load videos from config when component mounts
    use_effect_with((), {
        let demo_videos_clone = demo_videos.clone();
        let loading_clone = loading.clone();

        move |_| {
            spawn_local(async move {
                let config_path = get_config_path();

                match Config::from_file(&config_path).await {
                    Ok(config) => {
                        let videos = config.get_demo_videos();
                        demo_videos_clone.set(videos);
                        loading_clone.set(false);
                        log!("Loading videos...");
                    }
                    Err(e) => {
                        log::error!("Failed to load config: {:?}", e);
                        loading_clone.set(false);
                    }
                }
            });

            || ()
        }
    });


    let navigator = use_navigator();
let navigator = Rc::new(navigator); // Wrap navigator in Rc for shared ownership


pub fn navigate_to_about(index: usize, navigator: &Option<Navigator>) -> usize {
    if let Some(navigator) = navigator {
        let choreo_number = index + 1;
        navigator.push(&Route::AboutChoreo { number: choreo_number });
    } else {
        log!("Navigator is None");
    }
    index
}

let stop_music = ctx.stop_music.clone();

pub fn execute_choreo_video(
    index: usize,
    navigator: &Option<Navigator>,
    stop_music: &Callback<()>,

) -> usize {
    if let Some(navigator) = navigator {
        stop_music.emit(());
        match index {
            0 => navigator.push_with_state(&Route::ChoreoVideo, 0usize),
            1 => navigator.push_with_state(&Route::ChoreoVideo, 1usize),
            2 => navigator.push_with_state(&Route::ChoreoVideo, 2usize),
            3 => navigator.push_with_state(&Route::ChoreoVideo, 3usize),
            _ => {}
        }
    } else {
        log!("Navigator is None");
    }
    index
}

let handle_keydown_toggle = get_toggle_key(&demo_videos, current_video_index.clone());
let current_video_index_clone = current_video_index.clone();

let press_r_for_about = {
    let navigator = Rc::clone(&navigator);
    Callback::from(move |event: KeyboardEvent| {
        if event.key() == "r" {
            play_sound.emit("uiToAboutChoreo".to_string());
            navigate_to_about(*current_video_index_clone, &navigator);
        } else if event.key() == "e" {
            play_sound.emit("buttonSelect".to_string());
            execute_choreo_video(*current_video_index_clone, &navigator, &ctx.stop_music);
            // let soundeffect =
            //     web_sys::HtmlAudioElement::new_with_src("/static/buttonSelect.mp3").unwrap();
            // let _ = soundeffect.play();
        }
    })
};

let restart_app = {
    let navigator = Rc::clone(&navigator);
    let stop_music = stop_music.clone();
    
    Callback::from(move |event: KeyboardEvent| {
        if event.key() == "q" {
            if let Some(nav) = navigator.as_ref() {
                nav.push(&Route::IntroScreen1);
                stop_music.emit(());
            }
        }
    })
};

let handle_video_ended = {
    let navigator = Rc::clone(&navigator);
    Callback::from(move |_| {
        if let Some(nav) = navigator.as_ref() {
            nav.push(&Route::IntroScreen1);
        }
    })
};

    html! {
        //styling of page mainly found in molecules::video_list
        <div onkeydown={restart_app} onkeydown={press_r_for_about} tabindex="0">
            <div ref={div_ref} onkeydown={handle_keydown_toggle} tabindex="0">
            if *loading {
                <div></div>
            } else if demo_videos.len() > 0 {
                    <VideosList 
                    videos={(*demo_videos).clone()} 
                    current_index={*current_video_index} 
                    on_ended={Some(handle_video_ended)} 
                    video_class="smallscreenvideo"
                    /> 
                    <DanceOMaticLogo class="top-right-logo"/>
                    <BtnExplainerGraphics class="btn-container-main-menu"/>
            } else {
                <div>{"No videos available"}</div>
            }
        </div>
        </div>
    }
}