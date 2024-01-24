//lib.rs
//gloo writes stuff to the web console
//use gloo::console::log; use serde::{Serialize, Deserialize}; //<-- Uncomment to write to the webconsole
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
    #[at("/home")]
    Home,
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
        Route::Home => html! { <h1>{ "Home" }</h1> },
        Route::MainMenu => html! { <h1>{ "Main" }</h1> },
        Route::IntroScreen1 => html! {
            <IntroScreen />
        },
        Route::IntroScreen2 => html! {
            <IntroScreen />
        },
    }
}



