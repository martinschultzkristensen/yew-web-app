//lib.rs
use components::organisms::about_choreo::*;
use crate::components::organisms::choreo_videos::ChoreoVideo;
use crate::components::organisms::intro_screen::IntroScreen;
use crate::components::organisms::load_screen::LoadScreenVideo;
use crate::components::organisms::main_menu::MainMenu;
use crate::components::data::config::Config;
use crate::components::molecules::music_context::MusicContextProvider;
use crate::components::molecules::sound_effects::*;
use components::data::video_data::*;
use components::molecules::video_list::VideosList;
use components::molecules::keydown_logic::get_toggle_key;
use yew::functional::*;
use yew::prelude::*;
use yew_router::prelude::*;
use wasm_bindgen_futures::spawn_local;
use std::rc::Rc;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;
use serde_wasm_bindgen::to_value;

mod components;
#[derive(Clone, Routable, Debug, PartialEq)]
pub enum Route {
    #[at("/about-choreo/:number")]
    AboutChoreo { number: usize },
    #[at("/main-menu")]
    MainMenu,
    #[at("/")]
    IntroScreen1,
    #[at("/choreo-video")]
    ChoreoVideo,
    #[at("/loadscreen_video")]
    LoadScreenVideo,
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"], js_name = invoke)]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}



fn is_tauri() -> bool {
    web_sys::window()
        .unwrap()
        .get("__TAURI__")
        .is_some()
}

#[function_component(DanceOmatic)]
pub fn dance_o_matic() -> Html {
    
    let config = use_state(|| None::<Rc<Config>>);
        let config_fetched = use_state(|| false);
        let config_clone = config.clone();
        let config_fetched_clone = config_fetched.clone();


        use_effect(move || {
            // Only fetch if we haven't yet
            if is_tauri() && !*config_fetched_clone {
                spawn_local(async move {
                    let args = serde_json::json!({});
                    let js_args = to_value(&args).unwrap();
                    let result = invoke("get_config", js_args).await;
                    log::info!("Raw result from invoke: {:?}", result);
    
                    match serde_wasm_bindgen::from_value::<Config>(result) {
                        Ok(loaded_config) => {
                            let new_config = Rc::new(loaded_config);
                            if Some(new_config.clone()) != *config_clone {
                                log::info!("🔄 Config changed, updating state.");
                                config_clone.set(Some(new_config));
                            } else {
                                log::info!("✅ Config unchanged, skipping update.");
                            }
                            config_fetched_clone.set(true);
                        }
                        Err(err) => log::error!("Failed to deserialize config: {:?}", err),
                    }
                    
                });
            }
    
            || ()
        });


    html! {
        <div>
            <MusicContextProvider>
            <SoundEffectsProvider>
            <BrowserRouter>
                { if let Some(config) = &*config {
                    html! { <Switch<Route> render={switch(config.clone())} /> }
                } else {
                    html! { <p>{ "Loading config..." }</p> }
                }}
            </BrowserRouter>
            </SoundEffectsProvider>
            </MusicContextProvider>
        </div>
    }
}

fn switch(config: Rc<Config>) -> impl Fn(Route) -> Html {
    move |routes: Route| {
        match routes {
            Route::AboutChoreo { number } => html! {<AboutChoreo choreo_number={number} config={config.clone()} />},
            Route::MainMenu => html! { <MainMenu config={config.clone()} /> },
            Route::IntroScreen1 => html! { <IntroScreen config={config.clone()} /> },
            Route::ChoreoVideo => html! { <ChoreoVideo config={config.clone()}/> },
            Route::LoadScreenVideo => html! { <LoadScreenVideo config={config.clone()}/> },
        }
    }
}


// fn fetch_config() {
//     spawn_local(async {
//         let args = json!({}); // Create a serde_json::Value
//         let args_js: JsValue = to_value(&args).unwrap(); // Convert directly to JsValue

//         let result = invoke("get_config", args_js).await;
//         log!(format!("Config data: {:?}", result));
//     });
// }