//src/components/atoms/green_btn.rs
//Delete this file

use web_sys::Element;
use yew::prelude::*;

#[function_component(GreenButton)]
pub fn green_button() -> Html {
    let svg_ref = use_node_ref();

    // Add animation effect when component mounts
    use_effect({
        let svg_ref = svg_ref.clone();
        move || {
            if let Some(svg_element) = svg_ref.cast::<Element>() {
                if let Ok(Some(circle)) = svg_element.query_selector("#smCircle") {
                    let _ = circle.set_attribute("class", "green-btn-animate");
                }
            }
            || ()
        }
    });

    html! {
    
}}
          