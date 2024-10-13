// This code sets focus on the div when the component mounts

use yew::prelude::*;
use web_sys::HtmlElement;

pub fn use_focus_div() -> NodeRef {
    
    let div_ref = use_node_ref();
    // Set focus on the div when the component mounts
    {
        let div_ref = div_ref.clone(); //<-- error: method not found in `impl Hook<Output = NodeRef>`
        use_effect(move || {
            if let Some(div) = div_ref.cast::<HtmlElement>() {
                div.focus().unwrap();
            }
            || ()
        });
    }

    div_ref
}
