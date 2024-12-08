use web_sys::HtmlElement;
use yew::prelude::*;


#[derive(Properties, PartialEq)]
pub struct ScrollableDivProps {
    pub onkeydown: yew::Callback<web_sys::KeyboardEvent>,
    pub tabindex: &'static str,
    pub class: &'static str,
    pub children: yew::Children,
}

#[function_component(ScrollableDiv)]
pub fn scrollable_div(props: &ScrollableDivProps) -> Html {
    let div_ref = use_node_ref();
    let div_ref_clone = div_ref.clone();
    
    // Clone props.onkeydown to avoid lifetime issues in the Callback closure
    let parent_onkeydown = props.onkeydown.clone();

    // Audio element for playback
    let audio = use_state(|| {
        web_sys::HtmlAudioElement::new_with_src("/static/button-124476.mp3").unwrap()
    });

    // Focus the div when the component mounts
    use_effect(move || {
        if let Some(div) = div_ref_clone.cast::<HtmlElement>() {
            let _ = div.focus();
        }
        || ()
    });

    
    // Keydown handler for "W" and "S" keys to scroll
    let onkeydown = {
        let div_ref = div_ref.clone();
        let audio = audio.clone();
        Callback::from(move |event: KeyboardEvent| {
            if let Some(div) = div_ref.cast::<HtmlElement>() {
                match event.key().as_str() {
                    "w" => {
                        audio.set_current_time(0.0);
                        let _ = audio.play();
                        // Scroll up by 50px when "W" is pressed
                        div.scroll_by_with_x_and_y(0.0, -50.0);
                    }
                    "s" => {
                        audio.set_current_time(0.0);
                        let _ = audio.play();
                        // Scroll down by 50px when "S" is pressed
                        div.scroll_by_with_x_and_y(0.0, 50.0);
                    }
                    _ => {}
                }
            }
            // Emit the event to the parent handler
            parent_onkeydown.emit(event);
        })
    };

    html! {
        <div ref={div_ref} onkeydown={onkeydown} tabindex={props.tabindex} class={props.class}>
            { for props.children.iter() }
        </div>
    }
}
