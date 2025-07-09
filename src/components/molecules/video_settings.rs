// src/components/molecules/video_settings.rs
// Example of how to integrate video import into your main component
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use serde_wasm_bindgen::to_value;
use std::rc::Rc;

use crate::components::video_import::VideoImport;

// Assuming update_config is a function that updates a specific video path in your config
fn update_config(config: &mut Config, video_type: &str, new_path: &str) {
    match video_type {
        "intro" => {
            config.intro_video.url = new_path.to_string();
        },
        "loadscreen" => {
            config.loadscreen_video.url = new_path.to_string();
        },
        // Add more video types as needed
        _ => {}
    }
}

// Example integration in your DanceOmatic component
#[function_component(VideoSettings)]
pub fn video_settings() -> Html {
    let config = use_state(|| None::<Rc<Config>>);
    let selected_video_type = use_state(|| "intro".to_string());
    
    // Load the config when component mounts
    let config_clone = config.clone();
    use_effect(move || {
        if is_tauri() {
            spawn_local(async move {
                let result = invoke("get_config", JsValue::NULL).await;
                if let Ok(loaded_config) = serde_wasm_bindgen::from_value::<Config>(result) {
                    config_clone.set(Some(Rc::new(loaded_config)));
                }
            });
        }
        || ()
    }, []);
    
    // Handle video type selection
    let on_video_type_change = {
        let selected_video_type = selected_video_type.clone();
        Callback::from(move |e: Event| {
            let target = e.target_dyn_into::<HtmlSelectElement>().unwrap();
            selected_video_type.set(target.value());
        })
    };
    
    // Handle video import
    let on_video_import = {
        let config = config.clone();
        let selected_video_type = selected_video_type.clone();
        
        Callback::from(move |new_path: String| {
            let video_type = (*selected_video_type).clone();
            let config_value = config.clone();
            
            if let Some(current_config) = (*config_value).clone() {
                // Create a mutable copy of the config
                let mut updated_config = Config::clone(&current_config);
                
                // Update the config with the new path
                update_config(&mut updated_config, &video_type, &new_path);
                
                // Save the updated config
                spawn_local(async move {
                    let args = serde_json::json!({
                        "config": updated_config
                    });
                    let js_args = to_value(&args).unwrap();
                    
                    match invoke("save_config", js_args).await {
                        Ok(_) => {
                            // Update the local config state
                            config.set(Some(Rc::new(updated_config)));
                        },
                        Err(e) => {
                            log!("Failed to save config: ", e);
                        }
                    }
                });
            }
        })
    };
    
    // Display the current video path
    let current_video_path = {
        if let Some(current_config) = &*config {
            match (*selected_video_type).as_str() {
                "intro" => current_config.intro_video.url.clone(),
                "loadscreen" => current_config.loadscreen_video.url.clone(),
                // Add more video types as needed
                _ => "No video selected".to_string()
            }
        } else {
            "Loading...".to_string()
        }
    };
    
    html! {
        <div class="video-settings">
            <h2>{"Video Settings"}</h2>
            
            <div class="select-container">
                <label for="video-type">{"Select Video Type:"}</label>
                <select 
                    id="video-type" 
                    onchange={on_video_type_change}
                    value={(*selected_video_type).clone()}
                >
                    <option value="intro">{"Intro Video"}</option>
                    <option value="loadscreen">{"Loading Screen Video"}</option>
                    // Add more options as needed
                </select>
            </div>
            
            <div class="current-video">
                <p>{"Current Path: "}{current_video_path}</p>
            </div>
            
            <VideoImport on_import={on_video_import} />
        </div>
    }
}