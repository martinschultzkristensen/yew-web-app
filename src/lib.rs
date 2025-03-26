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

#[function_component(DanceOmatic)]
pub fn dance_o_matic() -> Html {
    let config = use_state(|| None);
    let config_clone = config.clone();

    use_effect(move || {
        spawn_local(async move {
            match Config::from_file("/static/config.toml").await {
                Ok(loaded_config) => {
                log::info!("Config loaded successfully");
                config_clone.set(Some(Rc::new(loaded_config)));
            }
                Err(err) => log::error!("Failed to load config: {:?}", err),
            }
        });
    
        || () // No cleanup needed
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
            Route::LoadScreenVideo => html! { <LoadScreenVideo/> },
        }
    }
}
