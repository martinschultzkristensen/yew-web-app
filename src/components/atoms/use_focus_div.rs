//src/components/atoms/use_focus_div.rs
// This code sets focus on the div when the component mounts. This way the selected component is active and bind key can be pressed immediatly.

use yew::prelude::*;
use web_sys::HtmlElement;


#[hook]
pub fn use_focus_div() -> NodeRef {
    let div_ref = use_node_ref();

    use_effect_with(
        div_ref.clone(),
        move |div_ref| {
            if let Some(div) = div_ref.cast::<HtmlElement>() {
                let _ = div.focus();
            }
            || ()
        }
    );

    div_ref
}
