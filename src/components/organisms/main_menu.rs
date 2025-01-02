//src/components/organisms/main_menu.rs
use crate::get_demo_videos;
use crate::get_toggle_key;
use crate::use_location;
use crate::Route;
use crate::VideosList;
use gloo::console::log;
use yew::prelude::*;
use yew_router::prelude::{use_navigator, Navigator};
use crate::components::atoms::use_focus_div::use_focus_div;
use crate::components::atoms::dance_o_matic_logo::DanceOMaticLogo;
use crate::components::molecules::btn_explainer_graphics::BtnExplainerGraphics;
use crate::components::molecules::music_context::*;





#[function_component(MainMenu)]
pub fn main_menu() -> Html {
    let ctx = use_context::<MusicContext>().expect("No music context provider");

        use_effect_with((), {
            let start_music = ctx.start_music.clone();
            move |_| {
                // Use the start_music callback from the context directly
                start_music.emit(());
                || ()
            }
        });
    

    let demo_videos = get_demo_videos();
    // State to track the index of the currently displayed demo video
    let current_video_index: usize = use_location()
        .and_then(|l| l.state::<usize>().map(|i| *i))
        .unwrap_or(0);

    let current_video_index: UseStateHandle<usize> = use_state(|| current_video_index);
    let navigator = use_navigator();
    let div_ref = use_focus_div(); // Hook sets focus on the div (in this case demo video) when the component mounts.

    pub fn navigate_to_about(index: usize, navigator: Option<Navigator>) -> usize {
        if let Some(navigator) = navigator {

        let choreo_number = index + 1;
        navigator.push(&Route::AboutChoreo { number: choreo_number });
    } else {
        log!("Navigator is None");
    }
    index
}

    pub fn execute_showdown_video(index: usize, navigator: Option<Navigator>, stop_music: &Callback<()>) -> usize {
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
    let stop_music = ctx.stop_music.clone();
    let current_video_index_clone = current_video_index.clone();
    let press_r_for_about = Callback::from(move |event: KeyboardEvent| {
        if event.key() == "r" {
            navigate_to_about(*current_video_index_clone, navigator.clone());

            let soundeffect =
                web_sys::HtmlAudioElement::new_with_src("/static/uiToAboutChoreo.mp3").unwrap();
            let _ = soundeffect.play();
        } else if event.key() == "e" {
            execute_showdown_video(*current_video_index_clone, navigator.clone(), &ctx.stop_music);
            let soundeffect =
                web_sys::HtmlAudioElement::new_with_src("/static/buttonSelect.mp3").unwrap();
            let _ = soundeffect.play();
        }
    });
    let navigator = use_navigator().unwrap(); //<-- #2 navigator. Is it enough with one? Test!
    let restart_app = Callback::from(move |event: KeyboardEvent| {
        if event.key() == "q" {
            navigator.push(&Route::IntroScreen1);
            stop_music.emit(());
        }
    });

    ////there's no need for atribute handle_video_ended, since it loops. Changes must be made in src/components/molecules/video_list.rs before delete!
    let navigator = use_navigator().unwrap(); //<-- #1 navigator. Is it enough with one? Test!
    let handle_video_ended = {
        let navigator = navigator.clone();
        Callback::from(move |_| {
            navigator.push(&Route::IntroScreen1);
        })
    };

    html! {
        //styling of page mainly found in molecules::video_list
        <div onkeydown={restart_app} onkeydown={press_r_for_about} tabindex="0">
            <div ref={div_ref} onkeydown={handle_keydown_toggle} tabindex="0"> 
                <VideosList videos={demo_videos} current_index={*current_video_index} on_ended={Some(handle_video_ended)} video_class="smallscreenvideo"/> 
                <DanceOMaticLogo class="top-right-logo"/>
                <BtnExplainerGraphics class="btn-container-main-menu"/>
            </div>
        </div>
    }
}
