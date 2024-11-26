use yew::prelude::*;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;


#[derive(Properties, PartialEq)]
pub struct ArrowIconProps {
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub transform: String,
    #[prop_or_default]
    pub respond: bool,
}

#[function_component(ArrowIcon)]
pub fn arrow(props: &ArrowIconProps) -> Html {
    
    let bounce_class = if props.respond { "bounce" } else { "" };

    html! {
        <svg 
            class={classes!(props.class.clone(), bounce_class)}
            style={format!("transform: {}", props.transform)}
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
     let rotation = if is_up { "rotate(180deg)" } else { "" };
     //let combined_transform = format!("transform: {} {}", rotation, respond_style).trim().to_string();
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
         <ArrowIcon class={classes!("svg-arrow-in-main")} transform={rotation.to_string()} respond={*respond} />
        }
    }

#[function_component(ArrowDownIcon)]
pub fn arrow_down_icon() -> Html {
    let is_up = false;
    let respond = use_state(|| false);

    // Clone `respond` state setter for use in the event handler
    let respond_clone = respond.clone();
    let rotation = if is_up { "rotate(180deg)" } else { "" };
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
     <ArrowIcon transform={rotation.to_string()} respond={*respond} />
    }
}


