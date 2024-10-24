use yew::prelude::*;
use gloo_timers::callback::Timeout;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;


#[function_component(GreenButton)]
fn green_button() -> Html {
    let svg_content = use_node_ref();
    
    use_effect(move || {
        if let Some(document) = web_sys::window().and_then(|w| w.document()) {
            if let Ok(Some(element)) = document.query_selector("#smCircel") {
                element.set_attribute("class", "green-btn-animate")
                    .expect("Failed to add animation class");
            }
        }
        || ()
    });

    html! {
        <div class="green-btn">
            {svg_content}
        </div>
    }
}


#[function_component(BtnExplainerGraphics)]
pub fn btn_explainer_graphics() -> Html {
    const FINGER_DELAY: u32 = 5000;  // 3 seconds
    const START_ADDITIONAL_DELAY: u32 = 2200; // 2.2 seconds after finger
    let is_start_visible = use_state(|| false);
    let green_btn_ref = use_node_ref();

    // Make these timings available to CSS via CSS custom properties
    let style = format!(
        ":root {{ 
            --finger-delay: {}ms; 
            --finger-animation-duration: 2000ms;
            --finger-bounce-duration: 200ms;
        }}",
        FINGER_DELAY
    );


    
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

    let start_class = if *is_start_visible {
        "txt_start txt_start-visible"
    } else {
        "txt_start"
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

      // Hook to trigger the animation effect on mount
    use_effect({
        let green_btn_ref = green_btn_ref.clone();
        move || {
            let green_btn_ref_clone = green_btn_ref.clone();
            if let Some(green_btn_element) = green_btn_ref_clone.cast::<web_sys::HtmlBaseElement>() {
                // Add an event listener for when the SVG content is fully loaded
                let green_btn_element_clone = green_btn_element.clone();
                let closure = Closure::wrap(Box::new(move || {
                    if let Some(content_document) = green_btn_element_clone.owner_document() {
                        if let Ok(Some(sm_circle)) = content_document.query_selector("#smCircel") {
                            // Add the class or style to animate
                            sm_circle
                                .set_attribute("class", "green-btn-animate")
                                .expect("Failed to add class to #smCircel");
                        }
                    }
                }) as Box<dyn Fn()>);

                green_btn_element.set_onload(Some(closure.as_ref().unchecked_ref()));
                closure.forget(); // Prevent it from being dropped
            }

            || () // Cleanup if needed
        }
    });

    html! {
    <>
    <style>{style}</style>
       <div class="btn-explainer-container">
            <object type="image/svg+xml" data="static/greenBtn.svg" class="green-btn" ref={green_btn_ref}></object>
            <object type="image/svg+xml" data="static/yellow_btn.svg" class="yellow-btn"></object>
            <object type="image/svg+xml" data="static/start.svg" class={start_class}></object>
            <object type="image/svg+xml" data="static/pointFinger.svg" class={finger_class}></object>
        </div>
    </>
    }
}
