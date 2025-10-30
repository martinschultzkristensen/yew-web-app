//src/components/organisms/about_choreo.rs
use crate::components::atoms::arrow_respnd_ui::*;
use crate::components::atoms::dancer::DancerCard;
use crate::components::atoms::shared_props::AppConfigProps;
use crate::components::data::choreography_data::get_choreography_data;
use crate::components::molecules::btn_explainer_graphics::BtnExplainerGraphics;
use crate::components::molecules::music_context::*;
use crate::components::molecules::scollable_div::ScrollableDiv;
use crate::components::molecules::sound_effects::SoundEffectsContext;
use crate::Route;
use gloo::console::log;
use yew::prelude::*;
use yew_router::prelude::use_navigator;

// For Tauri backend invoke
use serde_json::json;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"], js_name = invoke)]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

// #[derive(Properties, PartialEq)]
// pub struct AboutChoreoProps {
//     pub choreo_number: usize,
//     pub config: Rc<Config>,
// }

#[function_component(AboutChoreo)]
pub fn about_choreo(props: &AppConfigProps) -> Html {
    let ctx = use_context::<MusicContext>().expect("No music context provider");
    let sound_context =
        use_context::<SoundEffectsContext>().expect("SoundEffectsContext not found");
    let play_sound = sound_context.play_sound.clone();
    let config = &props.config;
    let choreography_data = get_choreography_data(config, props.choreo_number);
    log!("About Choreo Props:", props.choreo_number);
    let navigator = use_navigator().unwrap();
    let video_index = props.choreo_number - 1;
    let stop_music = ctx.stop_music.clone();

    // --- IMAGE RESOLVE LOGIC --- To resolve media/ paths to actual URLs for images
    let img_src = {
        let initial = choreography_data.choreo_image.clone();
        use_state(|| initial)
    };
    {
        let img_src = img_src.clone();
        let img_path = choreography_data.choreo_image.clone();
        use_effect_with(img_path.clone(), move |img_path| {
            if img_path.starts_with("media/") {
                wasm_bindgen_futures::spawn_local({
                    let img_src = img_src.clone();
                    let img_path = img_path.clone();
                    async move {
                        let js_args =
                            serde_wasm_bindgen::to_value(&serde_json::json!({ "path": img_path }))
                                .unwrap();
                        let result = invoke("resolve_media_path", js_args).await;
                        match serde_wasm_bindgen::from_value::<String>(result) {
                            Ok(resolved) => img_src.set(resolved),
                            Err(_) => img_src.set(img_path),
                        }
                    }
                });
            } else {
                img_src.set(img_path.clone());
            }
            || ()
        });
    }

    let event_key = Callback::from(move |event: KeyboardEvent| match event.key().as_str() {
        "q" => {
            stop_music.emit(());
            navigator.push(&Route::IntroScreen1)
        }
        "r" => {
            play_sound.emit("uiToAboutChoreo.mp3".to_string());
            navigator.push_with_state(&Route::MainMenu, video_index)
        }
        "e" => {
            stop_music.emit(());
            play_sound.emit("BtnStart.mp3".to_string());
            navigator.push_with_state(&Route::ChoreoVideo, video_index);
        }
        _ => (),
    });

    html! {
        <ScrollableDiv onkeydown={event_key} tabindex="1" class="about-choreo-container">
            <div class="svg-arrow-in-about-top">
                <ArrowUpIcon/>
            </div>
            <div class="arcadefont">
                <h2>{ choreography_data.title.clone() }</h2>
                // <p>{ format!("Config has {} demo videos", props.config.load_dancers().len())}</p>
                <div class="info-section-container">
                    <img src={(*img_src).clone()}
                         alt={format!("Choreography {}", props.choreo_number)} />
                    <p class="description">{ choreography_data.description.clone() }</p>
                </div>
                <h2>{"Dancers"}</h2>
                {
                    choreography_data.dancers.iter().map(|dancer| {
                        html! {
                            <DancerCard dancer={dancer.clone()}/>
                        }
                    }).collect::<Html>()
                }
            </div>
            <div class="svg-arrow-in-about-bottom">
                <ArrowDownIcon/>
            </div>
            <BtnExplainerGraphics class="btn-container-about-choreo" data="/static/goBack.svg"/>
        </ScrollableDiv>
    }
}
