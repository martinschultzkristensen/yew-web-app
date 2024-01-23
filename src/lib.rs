//lib.rs
//gloo writes stuff to the web console
//use gloo::console::log; use serde::{Serialize, Deserialize}; <-- Uncomment to write to the webconsole
use yew::prelude::*;
use yew_router::prelude;
use components::molecules::video_list::VideosList;
use components::data::video_data::*;
use components::organisms::keydown_logic::get_toggle_key;


mod components;

#[derive(Clone, Switch, Debug)]
pub enum AppRoute {
    #[to = "/main-menu"]
    MainMenu,
    #[to = "/intro-screen"]
    IntroScreen,
}


#[function_component(DanceOmatic)]
pub fn app() -> Html {
    html! {
        <Router<AppRoute, ()>
            render = Router::render(|switch: AppRoute| {
                match switch {
                    AppRoute::MainMenu => html! { <main_menu::MainMenu /> },
                    AppRoute::VideoList => html! { <video_list::VideoList /> },
                }
            })
        />
    }
}




