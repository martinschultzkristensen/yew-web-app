//lib.rs
//gloo writes stuff to the web console
//use gloo::console::log; use serde::{Serialize, Deserialize}; <-- Uncomment to write to the webconsole
use yew::functional::*;
use yew::prelude::*;
use yew_router::prelude::*;
use components::molecules::video_list::VideosList;
use components::data::video_data::*;
use components::organisms::keydown_logic::get_toggle_key;
use crate::components::organisms::intro_screen::IntroScreen;


mod components;



#[derive(Clone, Routable, Debug, PartialEq)]
pub enum Route {
    #[at("/main-menu")]
    MainMenu,
    #[at("/intro-screen")]
    IntroScreen,
}


#[function_component(DanceOmatic)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::MainMenu => html! { <h1>{ "Home" }</h1> },
        Route::IntroScreen => html! {
            <IntroScreen />
        },
    }
}



