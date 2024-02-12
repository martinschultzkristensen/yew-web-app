use yew::prelude::*;
use crate::Route;
use yew_router::prelude::use_navigator;


#[function_component(AboutChoreo1)]
pub fn main_menu() -> Html {
    let navigator = use_navigator().unwrap();
    let restart_app = Callback::from(move |event: KeyboardEvent| {
        if event.key() == "q"{
        navigator.push(&Route::IntroScreen1);
        }});
   


    html! { 
        <div onkeydown={restart_app} tabindex="0">
            <p>{ "Choreo1" }</p>
        </div>
    }
}

#[function_component(AboutChoreo2)]
pub fn about_choreo2() -> Html {
    let navigator = use_navigator().unwrap();
    let restart_app = Callback::from(move |event: KeyboardEvent| {
        if event.key() == "q"{
        navigator.push(&Route::IntroScreen1);
        }});
   


    html! { 
        <div onkeydown={restart_app} tabindex="0">
            <p>{ "Choreo2" }</p>
        </div>
    }
}

#[function_component(AboutChoreo3)]
pub fn about_choreo3() -> Html {
    let navigator = use_navigator().unwrap();
    let restart_app = Callback::from(move |event: KeyboardEvent| {
        if event.key() == "q"{
        navigator.push(&Route::IntroScreen1);
        }});
   


    html! { 
        <div onkeydown={restart_app} tabindex="0">
            <p>{ "Choreo3" }</p>
        </div>
    }
}

#[function_component(AboutChoreo4)]
pub fn about_choreo4() -> Html {
    
    html! {
        <div>
            <p>{ "Choreo4" }</p>
        </div>
    }
}