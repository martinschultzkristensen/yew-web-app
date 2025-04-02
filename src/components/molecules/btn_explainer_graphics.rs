//src/components/molecules/btn_explainer_graphics.rs
use yew::prelude::*;
use gloo_timers::callback::Timeout;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;


#[derive(Properties, PartialEq)]
pub struct BtnExplainerProps {
  #[prop_or_default]
  pub class: String,
  #[prop_or("/static/2info.svg".to_string())] // Default SVG file
  pub data: String,
}


#[function_component(BtnExplainerGraphics)]
pub fn btn_explainer_graphics(props: &BtnExplainerProps) -> Html {
    const FINGER_DELAY: u32 = 3000;  // 3 seconds
    const START_ADDITIONAL_DELAY: u32 = 2200; // 2.2 seconds after finger
    const INFO_ADDITIONAL_DELAY: u32 = 3200; // 3.2 seconds after finger
    let is_start_visible = use_state(|| false);
    let is_info_visible = use_state(|| false);
    

    
    {
        let is_start_visible = is_start_visible.clone();
        use_effect(
            move || {
                let timeout = Timeout::new(START_ADDITIONAL_DELAY + FINGER_DELAY, move || {
                    is_start_visible.set(true);
                });
                timeout.forget();
                || ()
            }
        );
    }
    {
        let is_info_visible = is_info_visible.clone();
        use_effect(
            move || {
                let timeout = Timeout::new(START_ADDITIONAL_DELAY + FINGER_DELAY + INFO_ADDITIONAL_DELAY, move || {
                    is_info_visible.set(true);
                });
                timeout.forget();
                || ()
            }
        );
    }

    let start_class = if *is_start_visible {
        "txt_start_position txt_animation txt_animation-visible"
    } else {
        "txt_start_position txt_animation"
    };

    let info_class = if *is_info_visible {
        "txt_info_position txt_animation-visible"
    } else {
        "txt_info_position txt_animation"
    };

    let point_finger_visible = use_state(|| false);

    {
        let point_finger_visible = point_finger_visible.clone();
        use_effect(
            move || {
                let timeout = Timeout::new(FINGER_DELAY, move || {
                    point_finger_visible.set(true);
                });
                timeout.forget();
                || ()
            }
        );
    }

    let finger_class = if *point_finger_visible {
        "point-finger point-finger-animate"
    } else {
        "point-finger"
    };

    let green_btn_ref = use_node_ref();
    // Calculate animation delay in seconds for SVG
    let svg_animation_delay = (FINGER_DELAY / 1000 + 2).to_string();

    // Effect to modify SVG animation timing once loaded
    {
        let green_btn_ref = green_btn_ref.clone();
        let svg_animation_delay = svg_animation_delay.clone();
        
        use_effect(move || {
            if let Some(object_element) = green_btn_ref.cast::<web_sys::HtmlElement>() {
                // Clone object_element before moving it into closure
                let object_element_clone = object_element.clone();
                
                // Wait for the object to load
                let onload = Closure::wrap(Box::new(move || {
                    // Try to get the contentDocument through JS
                    if let Some(content_document) = js_sys::Reflect::get(&object_element_clone, &"contentDocument".into())
                        .ok()
                        .and_then(|doc| doc.dyn_into::<web_sys::Document>().ok()) {
                        if let Ok(Some(animate_element)) = content_document
                            .query_selector("#smCircel animateTransform") {
                            let _ = animate_element.set_attribute("begin", &format!("{}s", svg_animation_delay));
                        }
                    }
                }) as Box<dyn FnMut()>);

                let _ = object_element.set_attribute("onload", "");
                object_element.set_onload(Some(onload.as_ref().unchecked_ref()));
                onload.forget(); // Prevent closure from being dropped
            }
            
            || ()
        });
    }


    let style = format!(
        ":root {{
            --finger-delay: {}ms;
            --finger-animation-duration: 2000ms;
            --finger-bounce-duration: 200ms;
        }}",
        FINGER_DELAY
    );
   

    html! {
    <>
    <style>{style}</style>
        <div class={classes!(props.class.clone(), "btn-container")}>
        <object ref={green_btn_ref} type="image/svg+xml" data="/static/greenBtn.svg" class="green-btn"></object>
            <object type="image/svg+xml" data="/static/yellow_btn.svg" class="yellow-btn"></object>
            <object type="image/svg+xml" data="/static/start.svg" class={start_class}></object>
            <object type="image/svg+xml" data={props.data.clone()} class={info_class}></object>
            <object type="image/svg+xml" data="/static/pointFinger.svg" class={finger_class}></object>
        </div>
    </>
    }
}