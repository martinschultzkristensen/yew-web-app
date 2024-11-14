//src/components/atoms/arrow_respond_ui.rs
use yew::prelude::*;


#[derive(Properties, PartialEq)]
pub struct ArrowProps {
    #[prop_or_default]
    pub class: Classes,
    #[prop_or(false)]
    pub is_up: bool,
}

#[function_component(ArrowIcon)]
pub fn arrow(props: &ArrowProps) -> Html {
    let rotation = if props.is_up { "transform: rotate(180deg)" } else { "" };
    
    html! {
        <svg 
            class={props.class.clone()}
            style={rotation}
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
    html! {
         
    }
}
#[function_component(ArrowDownIcon)]
pub fn arrow_down_icon() -> Html {
    html! {
        <object type="image/svg+xml" data="static/arrow-down-circle.svg"></object>
    }
}


#[function_component(ArrowRespondUi)]
pub fn arrow_respond_ui() -> Html {
    


    html! {
   
}}
          