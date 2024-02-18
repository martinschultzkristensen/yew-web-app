use components::organisms::about_choreo1::*;
//lib.rs
//gloo writes stuff to the web console
//use gloo::console::log; use serde::{Serialize, Deserialize}; //<-- Uncomment to write to the webconsole
use crate::components::organisms::intro_screen::IntroScreen;
use crate::components::organisms::main_menu::MainMenu;
// use crate::components::organisms::choreo_videos::ChoreoVideo;
use components::data::video_data::*;
use components::molecules::video_list::VideosList;
use components::organisms::keydown_logic::get_toggle_key;
use yew::functional::*;
use yew::prelude::*;
use yew_router::prelude::*;

mod components;

#[derive(Clone, Routable, Debug, PartialEq)]
pub enum Route {
    #[at("/about-choreo1")]
    AboutChoreo1,
    #[at("/about-choreo2")]
    AboutChoreo2,
    #[at("/about-choreo3")]
    AboutChoreo3,
    #[at("/about-choreo4")]
    AboutChoreo4,
    #[at("/main-menu")]
    MainMenu,
    #[at("/intro-screen")]
    IntroScreen2,
    #[at("/")]
    IntroScreen1,
    // #[at("/choreo-video")]
    // ChoreoVideo,
}

#[function_component(DanceOmatic)]
pub fn app() -> Html {
    html! {
    <div>
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    </div>
    }
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::AboutChoreo1 => html! { <AboutChoreo1 /> },
        Route::AboutChoreo2 => html! { <AboutChoreo2 /> },
        Route::AboutChoreo3 => html! { <AboutChoreo3 /> },
        Route::AboutChoreo4 => html! { <AboutChoreo4 /> },
        Route::MainMenu => html! { <MainMenu /> },
        Route::IntroScreen1 => html! { <IntroScreen/> },
        Route::IntroScreen2 => html! { <IntroScreen /> },
       // Route::ChoreoVideo => html! { < ChoreoVideo/> },
    }
}
