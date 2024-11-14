use yew::prelude::*;
use crate::components::atoms::use_focus_div::*;
use std::rc::Rc;

#[derive(Properties, PartialEq)]
pub struct ArrowIconProps {
    #[prop_or_default]
    pub class: Classes,
    #[prop_or(false)]
    pub is_up: bool,
    #[prop_or(24)]
    pub size: u32,
}

#[function_component(ArrowIcon)]
pub fn arrow_icon(props: &ArrowIconProps) -> Html {
    let rotation = if props.is_up { "transform: rotate(180deg)" } else { "" };
    
    html! {
        <svg 
            class={props.class.clone()}
            style={rotation}
            xmlns="http://www.w3.org/2000/svg" 
            width={props.size.to_string()}
            height={props.size.to_string()}
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


#[derive(Properties, PartialEq)]
pub struct ArrowContainerProps {
    #[prop_or_default]
    pub class: Classes,
    #[prop_or(true)]
    pub show_top_arrow: bool,
    #[prop_or(true)]
    pub show_bottom_arrow: bool,
    #[prop_or(24)]
    pub default_size: u32, // Default size for the arrows
    #[prop_or(false)]
    pub is_interactive: bool, // Whether arrow size should respond to keyboard input
}

#[function_component(ArrowContainer)]
pub fn arrow_container(props: &ArrowContainerProps) -> Html {
    
    let arrow_size = use_state(|| props.default_size);
    let arrow_ref = use_focus_div();


    // Only handle keyboard events if `is_interactive` is true
    let handle_keydown = {
        let arrow_size = arrow_size.clone();
        Callback::from(move |event: KeyboardEvent| {
            match event.key().as_str() {
                "w" => {
                    arrow_size.set((*arrow_size + 5).min(100));
                }
                "s" => {
                    arrow_size.set((*arrow_size - 5).max(10));
                }
                _ => {}
            }
        })
    };

    html! {
        <div class={props.class.clone()}
            tabindex="1"
            onkeydown={if props.is_interactive { handle_keydown } else { None }}
        >

            // Conditionally render top arrow if `show_top_arrow` is true
            { if props.show_top_arrow {
                html! { <ArrowIcon class={classes!(props.class.clone())} is_up={true} size={*arrow_size} /> }
            } else {
                html! {}
            }}

            // Conditionally render bottom arrow if `show_bottom_arrow` is true
            { if props.show_bottom_arrow {
                html! { <ArrowIcon class={classes!(props.class.clone())} is_up={false} size={*arrow_size} /> }
            } else {
                html! {}
            }}

        </div>
    }
}

