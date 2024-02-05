use std::string;

use components::organisms::about_choreo1::AboutChoreo1;
//lib.rs
//gloo writes stuff to the web console
//use gloo::console::log; use serde::{Serialize, Deserialize}; //<-- Uncomment to write to the webconsole
use crate::components::organisms::intro_screen::IntroScreen;
use crate::components::organisms::main_menu::MainMenu;
use components::data::video_data::*;
use components::molecules::video_list::VideosList;
use components::organisms::keydown_logic::get_toggle_key;
use yew::functional::*;
use yew::prelude::*;
use yew_router::prelude::*;

mod components;

#[derive(Clone, Routable, Debug, PartialEq)]
pub enum Route {
    #[at("/about-choreo1/:id")]
    AboutChoreo1 { id: String },
    #[at("/main-menu")]
    MainMenu,
    #[at("/intro-screen")]
    IntroScreen2,
    #[at("/")]
    IntroScreen1,
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
        Route::AboutChoreo1 { id } => html! { <AboutChoreo1 /> },
        Route::MainMenu => html! {
           <MainMenu />
        },
        Route::IntroScreen1 => html! {
            <IntroScreen/>
        },
        Route::IntroScreen2 => html! {
            <IntroScreen />
        },
    }
}
