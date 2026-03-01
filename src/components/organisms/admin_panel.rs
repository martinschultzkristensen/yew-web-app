
//src/components/organisms/admin_panel.rs
use yew::prelude::*;
use yew_router::prelude::use_navigator;
use crate::components::atoms::shared_props::AppConfigProps;

use crate::Route;
use crate::components::atoms::use_focus_div::use_focus_div;
use crate::components::data::video_imports::VideoImport; // Note: singular 'video_import'
use crate::components::data::image_imports::ImageImport; 



#[function_component(AdminPanel)]
pub fn admin_panel(props: &AppConfigProps) -> Html {
    let div_ref = use_focus_div();
    let navigator = use_navigator().unwrap();

    let target = use_state(|| "intro_video".to_string()); // or an enum

    let on_import = {
        let target = target.clone();
        Callback::from(move |new_path: String| {
            log::info!("imported {} into {}", new_path, *target);
            // update state/config/etc.
        })
    };

    let handle_video_import = Callback::from(move |new_path: String| {
        log::info!("Imported video path: {}", new_path);
        // Handle the path: save it, update state, route, etc.
    });

    let handle_image_import = Callback::from(move |new_path: String| {
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
        <select
                onchange={{
                    let target = target.clone();
                    Callback::from(move |e: Event| {
                        let sel = e.target_unchecked_into::<web_sys::HtmlSelectElement>();
                        target.set(sel.value());
                    })
                }}
            >
                <option value="intro_video">{"Intro video"}</option>
                <option value="loadscreen_video">{"Load‑screen video"}</option>
                <option value="demo_videos">{"Demo video #… (set index below)"}</option>
            </select>

            <VideoImport
                target_section={(*target).clone()}
                target_index={None /* or derived from another UI control */}
                on_import={on_import}
            />
            <h1>{ "Admin Panel" }</h1>
            <p>{ "Welcome to the lightweight admin panel." }</p>

            <VideoImport 
                target_section={"intro_video".to_string()} 
                /* target_index is optional, so omit for single-table sections */
                on_import={handle_video_import} 
            />
            <ImageImport on_import={handle_image_import} />

            <h2>{ "Current Config:" }</h2>
                <pre>{ format!("{:#?}", props.config) }</pre>

        </div>
    }
}

