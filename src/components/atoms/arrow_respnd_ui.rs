use yew::prelude::*;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;

#[derive(Properties, PartialEq)]
pub struct ArrowIconProps {
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub transform: String,
}



#[function_component(ArrowUpIcon)]
pub fn arrow_up_icon(props: &ArrowIconProps) -> Html {
    let respond = use_state(|| false);

    let respond_clone = respond.clone();

    use_effect(move || {
        let respond = respond_clone.clone();

        let listener = Closure::wrap(Box::new(move |event: web_sys::KeyboardEvent| {
            if event.key() == "w" {
                respond.set(true);
                let respond_reset = respond.clone();
                gloo::timers::callback::Timeout::new(300, move || {
                    respond_reset.set(false); // Reset after bounce animation
                })
                .forget();
            }
        }) as Box<dyn FnMut(_)>);

        web_sys::window()
            .unwrap()
            .add_event_listener_with_callback("keydown", listener.as_ref().unchecked_ref())
            .unwrap();

        move || {
            web_sys::window()
                .unwrap()
                .remove_event_listener_with_callback("keydown", listener.as_ref().unchecked_ref())
                .unwrap();
        }
    });

    let bounce_class = if *respond { "arrow-small" } else { "" };

    html! {
        <svg 
            class={classes!(props.class.clone(), "gradient-icon", bounce_class)}
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
            <polyline points="5 12 12 5 19 12"></polyline> // "Up" SVG points
        </svg>
    }
}


#[function_component(ArrowDownIcon)]
pub fn arrow_down_icon() -> Html {
    let respond = use_state(|| false);

    let respond_clone = respond.clone();

    use_effect(move || {
        let respond = respond_clone.clone();

        let listener = Closure::wrap(Box::new(move |event: web_sys::KeyboardEvent| {
            if event.key() == "s" {
                respond.set(true);
                let respond_reset = respond.clone();
                gloo::timers::callback::Timeout::new(300, move || {
                    respond_reset.set(false); // Reset after bounce animation
                })
                .forget();
            }
        }) as Box<dyn FnMut(_)>);

        web_sys::window()
            .unwrap()
            .add_event_listener_with_callback("keydown", listener.as_ref().unchecked_ref())
            .unwrap();

        move || {
            web_sys::window()
                .unwrap()
                .remove_event_listener_with_callback("keydown", listener.as_ref().unchecked_ref())
                .unwrap();
        }
    });

    let bounce_class = if *respond { "bounce" } else { "" };

    html! {
        <svg 
            class={classes!(bounce_class)}
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
            <polyline points="5 12 12 19 19 12"></polyline> // "Down" SVG points
        </svg>
    }
}



