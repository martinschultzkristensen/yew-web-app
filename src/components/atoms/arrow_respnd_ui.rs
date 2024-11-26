//src/components/atoms/arrow_respond_ui.rs
use yew::prelude::*;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;


#[derive(Properties, PartialEq)]
pub struct ArrowProps {
    #[prop_or_default]
    pub class: Classes,
    #[prop_or(false)]
    pub is_up: bool,
    #[prop_or_default]
    pub respond: bool,
}

#[function_component(ArrowIcon)]
pub fn arrow(props: &ArrowProps) -> Html {
    let rotation = if props.is_up { "rotate(180deg)" } else { "" };
    let respond_style = if props.respond { "scale(1.5)" } else { "" };
    let combined_transform = format!("transform: {} {}", rotation, respond_style).trim().to_string();

    html! {
        <svg 
            class={props.class.clone()}
            style={combined_transform}
            xmlns="http://www.w3.org/2000/svg" 
            width="24" 
            height="24" 
            viewBox="0 0 24 24" 
            fill="white" 
            stroke="white" 
            stroke-width="2" 
            stroke-linecap="round" 
            stroke-linejoin="round"
        >
            <line x1="12" y1="5" x2="12" y2="19"></line>
            <polyline points="19 12 12 19 5 12"></polyline>
        </svg>
    }
}

#[function_component(ArrowUpIcon)]
pub fn arrow_down_icon() -> Html {
     // State to track the `respond` property
     let is_up = true;
     let respond = use_state(|| false);
 
     // Clone `respond` state setter for use in the event handler
     let respond_clone = respond.clone();
     // Add a keydown event listener when the component mounts
     use_effect(
         move || {
             let respond = respond_clone.clone();
             let listener = Closure::wrap(Box::new(move |event: web_sys::KeyboardEvent| {
                if event.key() == "w" {
                    respond.set(!*respond);
                }
             }) as Box<dyn FnMut(_)>);
 
             web_sys::window()
                 .unwrap()
                 .add_event_listener_with_callback("keydown", listener.as_ref().unchecked_ref())
                 .unwrap();
 
             // Cleanup function to remove the event listener
             move || {
                 web_sys::window()
                     .unwrap()
                     .remove_event_listener_with_callback("keydown", listener.as_ref().unchecked_ref())
                     .unwrap();
             }
         }, // Dependencies (empty so this runs once on mount)
     );
    html! {
         <ArrowIcon is_up={is_up} respond={*respond} />
    }
}

#[function_component(ArrowDownIcon)]
pub fn arrow_down_icon() -> Html {
    let is_up = false;
    let respond = use_state(|| false);

    // Clone `respond` state setter for use in the event handler
    let respond_clone = respond.clone();
    // Add a keydown event listener when the component mounts
    use_effect(
        move || {
            let respond = respond_clone.clone();
            let listener = Closure::wrap(Box::new(move |event: web_sys::KeyboardEvent| {
               if event.key() == "s" {
                   respond.set(!*respond);
               }
            }) as Box<dyn FnMut(_)>);

            web_sys::window()
                .unwrap()
                .add_event_listener_with_callback("keydown", listener.as_ref().unchecked_ref())
                .unwrap();

            // Cleanup function to remove the event listener
            move || {
                web_sys::window()
                    .unwrap()
                    .remove_event_listener_with_callback("keydown", listener.as_ref().unchecked_ref())
                    .unwrap();
            }
        }, // Dependencies (empty so this runs once on mount)
    );
    html! {
     <ArrowIcon is_up={is_up} respond={*respond} />
    }
}


#[function_component(ArrowRespondUi)]
pub fn arrow_respond_ui() -> Html {
    


    html! {
   
}}
          