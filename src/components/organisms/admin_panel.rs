
//src/components/organisms/admin_panel.rs
use yew::prelude::*;
use yew_router::prelude::use_navigator;
use crate::components::atoms::shared_props::AppConfigProps;

use crate::Route;
use crate::components::atoms::use_focus_div::use_focus_div;
use crate::components::data::video_imports::VideoImport; // Note: singular 'video_import'



#[function_component(AdminPanel)]
pub fn admin_panel(props: &AppConfigProps) -> Html {
    let div_ref = use_focus_div();
    let navigator = use_navigator().unwrap();

    let handle_video_import = Callback::from(move |new_path: String| {
        log::info!("Imported video path: {}", new_path);
        // Handle the path: save it, update state, route, etc.
    });

    let restart_app = Callback::from({
        let navigator = navigator.clone();
        move |event: KeyboardEvent| {
            if event.ctrl_key() && event.shift_key() && event.key() == "Q" {
                navigator.push(&Route::IntroScreen1);
            }
        }
    });


    html! {
        <div class="about-choreo-container" ref={div_ref} tabindex="0" onkeydown={restart_app}>
            <h1>{ "Admin Panel" }</h1>
            <p>{ "Welcome to the lightweight admin panel." }</p>

            <VideoImport on_import={handle_video_import} />

            <h2>{ "Current Config:" }</h2>
                <pre>{ format!("{:#?}", props.config) }</pre>

        </div>
    }
}
