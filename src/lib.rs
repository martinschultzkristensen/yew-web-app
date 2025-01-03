//lib.rs
//gloo writes stuff to the web console
//use gloo::console::log; use serde::{Serialize, Deserialize}; //<-- Uncomment to write to the webconsole
use crate::components::organisms::choreo_videos::ChoreoVideo;
use crate::components::organisms::intro_screen::IntroScreen;
use crate::components::organisms::load_screen::LoadScreenVideo;
use crate::components::organisms::main_menu::MainMenu;
use crate::components::organisms::admin_panel::AdminPanel;
use crate::components::molecules::music_context::MusicContextProvider;
use components::organisms::about_choreo::*;
use components::data::video_data::*;
use components::molecules::video_list::VideosList;
use components::molecules::keydown_logic::get_toggle_key;
use yew::functional::*;
use yew::prelude::*;
use yew_router::prelude::*;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;

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
    #[at("/admin-panel")]
    AdminPanel, // New route for the admin panel
}

#[function_component(DanceOmatic)]
pub fn app() -> Html {

    // Callback for key detection
    {
        use_effect(|| {
            let callback = Closure::wrap(Box::new(move |event: KeyboardEvent| {
                if event.ctrl_key() && event.shift_key() && event.key() == "A" {
                    // Navigate to the Admin Panel
                    web_sys::window()
                        .unwrap()
                        .location()
                        .set_href("/admin-panel")
                        .unwrap();
                }
            }) as Box<dyn FnMut(_)>);

            // Attach the listener
            web_sys::window()
                .unwrap()
                .add_event_listener_with_callback("keydown", callback.as_ref().unchecked_ref())
                .unwrap();

            // Cleanup
            move || {
                web_sys::window()
                    .unwrap()
                    .remove_event_listener_with_callback(
                        "keydown",
                        callback.as_ref().unchecked_ref(),
                    )
                    .unwrap();
            }
        });
    }
    
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
        Route::ChoreoVideo => html! { <ChoreoVideo/> },
        Route::LoadScreenVideo => html! { <LoadScreenVideo/> },
        Route::AdminPanel => html! { <AdminPanel/> }, // Render AdminPanel
    };
    vnode
}
