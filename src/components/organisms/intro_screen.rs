use crate::components::atoms::shared_props::AppConfigProps;
use crate::components::atoms::use_focus_div::use_focus_div;
use crate::components::molecules::music_context::MusicContext;
use crate::Route;
use crate::VideosList;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_router::prelude::use_navigator;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"], js_name = invoke)]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

fn request_restart_if_update_pending() {
    spawn_local(async {
        let _ = invoke("restart_if_machine_update_pending", JsValue::NULL).await;
    });
}

#[function_component(IntroScreen)]
pub fn intro_screen(props: &AppConfigProps) -> Html {
    let audio_coin = web_sys::HtmlAudioElement::new_with_src("/static/coinSound.mp3").unwrap();
    let navigator = use_navigator().unwrap();
    let intro_video = props.config.get_intro_video();
    let current_video_index = use_state(|| 0);
    let div_ref = use_focus_div(); // Hook sets focus on the div when the component mounts.
    let music_ctx = use_context::<MusicContext>().expect("No music context provider");
    let start_music = music_ctx.start_music.clone();

    use_effect_with((), move |_| {
        request_restart_if_update_pending();
        || ()
    });

    //there's no need for handle_video_ended. Changes must be made in src/components/molecules/video_list.rs before delete!
    let handle_video_ended = {
        let navigator = navigator.clone();
        Callback::from(move |_| {
            request_restart_if_update_pending();
            navigator.push(&Route::IntroScreen1);
        })
    };

    let press_x_for_main = Callback::from(move |event: KeyboardEvent| {
        if event.key() == "x" {
            let _ = audio_coin.play();
            // Called synchronously within this keydown handler (not from a
            // use_effect on MainMenu mount) so it stays inside the user
            // gesture's call stack — WebKitGTK on Linux blocks audio.play()
            // once that stack has unwound, even though other engines allow it.
            start_music.emit(());
            navigator.push(&Route::MainMenu);
        }
    });

    html! {
        <div ref={div_ref} onkeydown={press_x_for_main} tabindex="0">
            <VideosList videos={intro_video} current_index={*current_video_index} on_ended={Some(handle_video_ended)} video_class="fullscreenvideo"/>
        </div>
    }
}
