use components::organisms::about_choreo::*;
//lib.rs
//gloo writes stuff to the web console
//use gloo::console::log; use serde::{Serialize, Deserialize}; //<-- Uncomment to write to the webconsole
use crate::components::organisms::choreo_videos::ChoreoVideo;
use crate::components::organisms::intro_screen::IntroScreen;
use crate::components::organisms::load_screen::LoadScreenVideo;
use crate::components::organisms::main_menu::MainMenu;
use crate::components::organisms::music::Music;
use crate::components::molecules::music_context::MusicContextProvider;
use components::data::video_data::*;
use components::molecules::video_list::VideosList;
use components::molecules::keydown_logic::get_toggle_key;
use yew::functional::*;
use yew::prelude::*;
use yew_router::prelude::*;

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
pub fn app() -> Html {
    html! {
    <div>
        <MusicContextProvider>
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
        </MusicContextProvider>
    </div>
    }
}

fn switch(routes: Route) -> Html {
    let vnode = match routes {
        Route::AboutChoreo { number } => html! {
            <AboutChoreo choreo_number={number} />},
        Route::MainMenu => html! { <MainMenu /> },
        Route::IntroScreen1 => html! { <IntroScreen/> },
        Route::ChoreoVideo => html! { < ChoreoVideo/> },
        Route::LoadScreenVideo => html! { < LoadScreenVideo/> },
    };
    vnode
}
