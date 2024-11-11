use web_sys::HtmlElement;
use yew::prelude::*;
use js_sys::Array;
use wasm_bindgen::JsCast;
use crate::components::atoms::use_focus_div::use_focus_div;

#[derive(Properties, PartialEq)]
pub struct ScrollableDivProps {
    pub onkeydown: yew::Callback<web_sys::KeyboardEvent>,
    pub tabindex: &'static str,
    pub class: &'static str,
    pub children: yew::Children,
}

#[function_component(ScrollableDiv)]
pub fn scrollable_div(props: &ScrollableDivProps) -> Html {
    let div_ref = use_focus_div(); // Using your custom hook instead
    let current_focus_index = use_state(|| 0);
    let sections_ref = use_state(|| Vec::new());
    
    // Initialize sections on mount
    {
        let div_ref = div_ref.clone();
        let sections_ref = sections_ref.clone();
        use_effect(move || {
            if let Some(div) = div_ref.cast::<HtmlElement>() {
                if let Ok(sections_nodelist) = div.query_selector_all(".info-section-container") {
                    // Convert NodeList to an Array and then collect into a Vec
                    let sections = Array::from(&sections_nodelist)
                        .iter()
                        .collect::<Vec<_>>();
                    
                    // Set focus on the first section initially
                    if let Some(first) = sections.first() {
                        if let Some(element) = first.dyn_ref::<HtmlElement>() {
                            element.class_list().add_1("focused").unwrap();
                        }
                    }

                    sections_ref.set(sections);
                }
            }
            || ()
        });
    }

    // Keydown handler for arrow keys to toggle focus
    let onkeydown = {
        let sections = sections_ref.clone();
        let current_index = current_focus_index.clone();
        let parent_onkeydown = props.onkeydown.clone();
        
        Callback::from(move |event: KeyboardEvent| {
            let sections = (*sections).clone();
            let mut index = *current_index;
            
            match event.key().as_str() {
                "s" => {
                    if index < sections.len() - 1 {
                        // Remove focus from current section
                        if let Some(current) = sections.get(index) {
                            if let Some(element) = current.dyn_ref::<HtmlElement>() {
                            element.class_list().remove_1("focused").unwrap();
                        }
                    }
                        // Add focus to next section
                        index += 1;
                        if let Some(next) = sections.get(index) {
                            if let Some(element) = next.dyn_ref::<HtmlElement>() {
                                element.class_list().add_1("focused").unwrap();
                            }
                        }
                        current_index.set(index);
                    }
                }
                "w" => {
                    if index > 0 {
                        // Remove focus from the current section
                        if let Some(current) = sections.get(index) {
                            if let Some(element) = current.dyn_ref::<HtmlElement>() {
                                element.class_list().remove_1("focused").unwrap();
                            }
                        }
                        // Add focus to the previous section
                        index -= 1;
                        if let Some(previous) = sections.get(index) {
                            if let Some(element) = previous.dyn_ref::<HtmlElement>() {
                                element.class_list().add_1("focused").unwrap();
                            }
                        }
                        current_index.set(index);
                    }
                }
                _ => {}
            }
            // Propagate the event to parent handler
            parent_onkeydown.emit(event);
        })
    };

    html! {
        <div ref={div_ref} onkeydown={onkeydown} tabindex={props.tabindex} class={props.class}>
            { for props.children.iter() }
        </div>
    }
}