//src/components/data/video_import.rs

use wasm_bindgen::JsValue;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use serde_wasm_bindgen::to_value;
use gloo::console::log;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"], js_name = invoke)]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Properties, PartialEq)]
pub struct VideoImportProps {
    pub on_import: Callback<String>, // Called with the new path after successful import
}



#[function_component(VideoImport)]
pub fn video_import(props: &VideoImportProps) -> Html {
    let importing = use_state(|| false);
    let on_import = props.on_import.clone(); // clone callback here

    let on_import_click = {
        let importing = importing.clone();

        Callback::from(move |_| {
            importing.set(true);
            let importing_clone = importing.clone();
            let on_import = on_import.clone(); // move the clone inside async

            spawn_local(async move {
                // Open file dialog
                let select_result = invoke("select_video_file", JsValue::NULL).await;

                match serde_wasm_bindgen::from_value::<String>(select_result) {
                    Ok(selected_path) => {
                        log!("Selected file: ", &selected_path);

                        // Import the selected file
                        let args = serde_json::json!({
                            "sourcePath": selected_path
                        });
                        let js_args = to_value(&args).unwrap();

                        let import_result = invoke("import_video", js_args).await;

                        match serde_wasm_bindgen::from_value::<String>(import_result) {
                            Ok(new_path) => {
                                log!("Imported video to: ", &new_path);
                                on_import.emit(new_path); // now this is safe
                            },
                            Err(e) => {
                                log!("Failed to deserialize import result: ", e);
                            }
                        }
                    },
                    Err(e) => {
                        log!("Failed to select file: ", e);
                    }
                }

                importing_clone.set(false);
            });
        })
    };

    html! {
        <button onclick={on_import_click} disabled={*importing}>
            { if *importing { "Importing..." } else { "Import video" } }
        </button>
    }
}