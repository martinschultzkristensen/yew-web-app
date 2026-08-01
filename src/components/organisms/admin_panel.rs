use crate::components::atoms::shared_props::AppConfigProps;
use crate::components::atoms::use_focus_div::use_focus_div;
use crate::components::data::image_imports::ImageImport;
use crate::components::data::video_imports::VideoImport;
use crate::Route;

use js_sys::Promise;
use serde::Deserialize;
use serde_json::json;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::{spawn_local, JsFuture};
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::prelude::use_navigator;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        js_namespace = ["window", "__TAURI__", "core"],
        js_name = invoke
    )]
    fn tauri_invoke(command: &str, args: JsValue) -> Promise;
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
struct MachineConnectionResult {
    machine_id: String,
    display_name: String,
    location: Option<String>,
    session_expires_at: u64,
    choreography_count: usize,
    file_count: usize,
}

fn empty_args() -> JsValue {
    serde_wasm_bindgen::to_value(&json!({})).expect("Failed to create empty Tauri arguments")
}

fn error_message(error: JsValue) -> String {
    error.as_string().unwrap_or_else(|| format!("{error:?}"))
}

#[function_component(AdminPanel)]
pub fn admin_panel(props: &AppConfigProps) -> Html {
    let div_ref = use_focus_div();
    let navigator = use_navigator().unwrap();

    let email_ref = use_node_ref();
    let password_ref = use_node_ref();

    let busy = use_state(|| false);
    let status_message = use_state(|| "Checking machine connection...".to_string());
    let connection = use_state(|| None::<MachineConnectionResult>);

    // Check whether this installation already has a valid machine session.
    {
        let connection = connection.clone();
        let status_message = status_message.clone();

        use_effect_with((), move |_| {
            spawn_local(async move {
                match JsFuture::from(tauri_invoke("check_machine_connection", empty_args())).await {
                    Ok(value) => {
                        match serde_wasm_bindgen::from_value::<MachineConnectionResult>(value) {
                            Ok(result) => {
                                status_message
                                    .set(format!("Connected to {}.", result.display_name));
                                connection.set(Some(result));
                            }
                            Err(error) => {
                                status_message
                                    .set(format!("Could not read machine information: {error}"));
                            }
                        }
                    }
                    Err(_) => {
                        status_message
                            .set("Machine is not activated on this computer.".to_string());
                    }
                }
            });

            || ()
        });
    }

    let activate_machine = {
        let email_ref = email_ref.clone();
        let password_ref = password_ref.clone();
        let busy = busy.clone();
        let status_message = status_message.clone();
        let connection = connection.clone();

        Callback::from(move |_| {
            if *busy {
                return;
            }

            let Some(email_input) = email_ref.cast::<HtmlInputElement>() else {
                status_message.set("Could not read the email field.".to_string());
                return;
            };

            let Some(password_input) = password_ref.cast::<HtmlInputElement>() else {
                status_message.set("Could not read the password field.".to_string());
                return;
            };

            let email = email_input.value().trim().to_string();
            let password = password_input.value();

            if email.is_empty() {
                status_message.set("Enter the machine email.".to_string());
                return;
            }

            if password.is_empty() {
                status_message.set("Enter the machine password.".to_string());
                return;
            }

            busy.set(true);
            status_message.set("Connecting to Supabase...".to_string());

            let busy = busy.clone();
            let status_message = status_message.clone();
            let connection = connection.clone();
            let password_ref = password_ref.clone();

            spawn_local(async move {
                let args = match serde_wasm_bindgen::to_value(&json!({
                    "email": email,
                    "password": password
                })) {
                    Ok(args) => args,
                    Err(error) => {
                        status_message.set(format!("Could not prepare login request: {error}"));
                        busy.set(false);
                        return;
                    }
                };

                match JsFuture::from(tauri_invoke("activate_machine", args)).await {
                    Ok(value) => {
                        match serde_wasm_bindgen::from_value::<MachineConnectionResult>(value) {
                            Ok(result) => {
                                status_message.set(format!(
                                    "Machine activated successfully: {}.",
                                    result.display_name
                                ));
                                connection.set(Some(result));
                            }
                            Err(error) => {
                                status_message.set(format!(
                                    "Login succeeded, but the response was invalid: {error}"
                                ));
                            }
                        }
                    }
                    Err(error) => {
                        status_message.set(format!(
                            "Machine activation failed: {}",
                            error_message(error)
                        ));
                        connection.set(None);
                    }
                }

                // Never leave the password visible after an activation attempt.
                if let Some(password_input) = password_ref.cast::<HtmlInputElement>() {
                    password_input.set_value("");
                }

                busy.set(false);
            });
        })
    };

    let clear_session = {
        let busy = busy.clone();
        let status_message = status_message.clone();
        let connection = connection.clone();

        Callback::from(move |_| {
            if *busy {
                return;
            }

            busy.set(true);
            status_message.set("Removing machine session...".to_string());

            let busy = busy.clone();
            let status_message = status_message.clone();
            let connection = connection.clone();

            spawn_local(async move {
                match JsFuture::from(tauri_invoke("clear_machine_session", empty_args())).await {
                    Ok(_) => {
                        connection.set(None);
                        status_message
                            .set("Machine session removed from this computer.".to_string());
                    }
                    Err(error) => {
                        status_message.set(format!(
                            "Could not remove session: {}",
                            error_message(error)
                        ));
                    }
                }

                busy.set(false);
            });
        })
    };

    let handle_video_import = Callback::from(move |new_path: String| {
        log::info!("Imported video path: {}", new_path);
    });

    let handle_image_import = Callback::from(move |new_path: String| {
        log::info!("Imported image path: {}", new_path);
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
        <div
            class="about-choreo-container"
            ref={div_ref}
            tabindex="0"
            onkeydown={restart_app}
        >
            <h1>{ "Admin Panel" }</h1>

            <section class="machine-activation">
                <h2>{ "Machine connection" }</h2>

                <p>
                    {
                        "Activate this installation with one of the dedicated \
                         DanceOmatic machine accounts."
                    }
                </p>

                <div>
                    <label for="machine-email">
                        { "Machine email" }
                    </label>

                    <input
                        id="machine-email"
                        ref={email_ref}
                        type="email"
                        autocomplete="username"
                        placeholder="machine1@artfarm.dk"
                        disabled={*busy}
                    />
                </div>

                <div>
                    <label for="machine-password">
                        { "Machine password" }
                    </label>

                    <input
                        id="machine-password"
                        ref={password_ref}
                        type="password"
                        autocomplete="current-password"
                        disabled={*busy}
                    />
                </div>

                <button
                    type="button"
                    onclick={activate_machine}
                    disabled={*busy}
                >
                    {
                        if *busy {
                            "Working..."
                        } else {
                            "Activate machine"
                        }
                    }
                </button>

                <p>{ (*status_message).clone() }</p>

                {
                    if let Some(machine) = &*connection {
                        html! {
                            <div class="machine-connection-info">
                                <p>
                                    <strong>{ "Machine: " }</strong>
                                    { &machine.display_name }
                                </p>

                                <p>
                                    <strong>{ "Machine ID: " }</strong>
                                    { &machine.machine_id }
                                </p>

                                {
                                    machine.location.as_ref().map(|location| {
                                        html! {
                                            <p>
                                                <strong>{ "Location: " }</strong>
                                                { location }
                                            </p>
                                        }
                                    }).unwrap_or_default()
                                }

                                <p>
                                    <strong>{ "Assigned choreographies: " }</strong>
                                    { machine.choreography_count }
                                </p>

                                <p>
                                    <strong>{ "Required files: " }</strong>
                                    { machine.file_count }
                                </p>

                                <button
                                    type="button"
                                    onclick={clear_session}
                                    disabled={*busy}
                                >
                                    { "Remove machine connection" }
                                </button>
                            </div>
                        }
                    } else {
                        Html::default()
                    }
                }
            </section>

            <hr />

            <section>
                <h2>{ "Local media tools" }</h2>

                <VideoImport on_import={handle_video_import} />
                <ImageImport on_import={handle_image_import} />
            </section>

            <h2>{ "Current Config:" }</h2>
            <pre>{ format!("{:#?}", props.config) }</pre>
        </div>
    }
}
