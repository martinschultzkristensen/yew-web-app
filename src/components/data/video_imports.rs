//src/components/data/video_import.rs

use wasm_bindgen::JsValue;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use serde_wasm_bindgen::to_value;
use gloo::console::log;
use wasm_bindgen::prelude::wasm_bindgen;
use super::config::Config;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"], js_name = invoke)]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Properties, PartialEq)]
pub struct VideoImportProps {
    pub on_import: Callback<String>,      // Called with the new path after successful import
    #[prop_or("intro_video".to_string())]
    pub target_section: String,           // "intro_video", "loadscreen_video", "demo_videos", "choreo_videos"
    #[prop_or_default]
    pub target_index: Option<usize>,      // Required for demo_videos, choreo_videos; None for intro_video, loadscreen_video
}



#[function_component(VideoImport)]
pub fn video_import(props: &VideoImportProps) -> Html {
    let importing = use_state(|| false);
    let on_import = props.on_import.clone(); // clone callback here
    let target_section = props.target_section.clone();
    let target_index = props.target_index;

    let on_import_click = {
        let importing = importing.clone();

        Callback::from(move |_| {
            importing.set(true);
            let importing_clone = importing.clone();

            let on_import = on_import.clone(); // move the clone inside async
            let on_import = on_import.clone(); // clone again for async move
            let target_section = target_section.clone();
            let target_index = target_index;

            spawn_local(async move {
                // Open file dialog
                let select_result = invoke("select_video_file", JsValue::NULL).await;

                match serde_wasm_bindgen::from_value::<Option<String>>(select_result) {
                    Ok(Some(selected_path)) => {
                        log!("Selected file: ", &selected_path);

                        // 2️⃣ Import selected file into media folder
                        let args = serde_json::json!({
                            "sourcePath": selected_path
                        });

                        let js_args = to_value(&args).unwrap();

                        let import_result =
                            invoke("import_video", js_args).await;

                        match serde_wasm_bindgen::from_value::<String>(import_result) {
                            Ok(new_path) => {
                                log!("Imported video to: ", &new_path);

                                // 3️⃣ Update config.toml with target section
                                let update_args = serde_json::json!({
                                    "section": target_section,
                                    "index": target_index,
                                    "newPath": new_path.clone()
                                });

                                let update_js_args =
                                    to_value(&update_args).unwrap();

                                let update_result =
                                    invoke("update_config_video", update_js_args).await;

                                match serde_wasm_bindgen::from_value::<Config>(update_result) {
                                    Ok(_updated_config) => {
                                        log!("Config successfully updated");

                                        // 4️⃣ Emit callback only after config update
                                        on_import.emit(new_path);
                                    }
                                    Err(e) => {
                                        log!("Failed to update config: ", e);
                                    }
                                }
                            }
                            Err(e) => {
                                log!("Failed to deserialize import result: ", e);
                            }
                        }
                    }
                    Ok(None) => {
                        log!("User cancelled file selection");
                    }
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