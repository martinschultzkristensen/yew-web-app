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
    let outline_class = if *respond { "arrow-outline" } else { "" };

    html! {
        <div class="arrow-stack">
                <object type="image/svg+xml" data="/static/arrowUpPixel.svg" class={classes!(props.class.clone(), bounce_class, "base-arrow")}></object>
                <object type="image/svg+xml" data="/static/pinkArrowUp.svg" class={classes!(props.class.clone(), outline_class)}></object>
        </div>
        
    }
}


#[function_component(ArrowDownIcon)]
pub fn arrow_down_icon(props: &ArrowIconProps) -> Html {
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

    let bounce_class = if *respond { "arrow-small" } else { "" };
    let outline_class = if *respond { "arrow-outline" } else { "" };

    html! {
        <div class="arrow-stack">
                <object type="image/svg+xml" data="/static/arrowDownPixel.svg" class={classes!(props.class.clone(), bounce_class, "base-arrow")}></object>
                <object type="image/svg+xml" data="/static/pinkArrowDown.svg" class={classes!(props.class.clone(), outline_class)}></object>
        </div>
    }
}



