use yew::prelude::*;
use crate::Route;
use yew_router::prelude::use_navigator;
use crate::components::molecules::video_list::_VideosListProps::current_index;


#[function_component(AboutChoreo1)]
pub fn about_choreo1() -> Html {
    let navigator = use_navigator().unwrap();
    let event_key = Callback::from(move |event: KeyboardEvent| {
        if event.key() == "q"{
        navigator.push(&Route::IntroScreen1);
        } else if event.key() == "r" {
            navigator.push_with_state(&Route::MainMenu, 0);        
        }});

    
   
    html! { 
        <div onkeydown={event_key} tabindex="0">
            <p>{ "Choreo1" }</p>
        </div>
    }
}

#[function_component(AboutChoreo2)]
pub fn about_choreo2() -> Html {
    let navigator = use_navigator().unwrap();
    let event_key = Callback::from(move |event: KeyboardEvent| {
        if event.key() == "q" {
            navigator.push(&Route::IntroScreen1);
        } else if event.key() == "r" {
            navigator.push_with_state(&Route::MainMenu, 1); // Navigate back to MainMenu with index 2
        }
    });

    html! {
        <div onkeydown={event_key} tabindex="1">
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
            } else if event.key() == "r" {
                navigator.push_with_state(&Route::MainMenu, 2);        
            }});
   


    html! { 
        <div onkeydown={restart_app} tabindex="0">
            <p>{ "Choreo3" }</p>
        </div>
    }
}

#[function_component(AboutChoreo4)]
pub fn about_choreo4() -> Html {
    let navigator = use_navigator().unwrap();
    let restart_app = Callback::from(move |event: KeyboardEvent| {
        if event.key() == "q"{
            navigator.push(&Route::IntroScreen1);
            } else if event.key() == "r" {
                navigator.push_with_state(&Route::MainMenu, 2);        
            }});
   


    html! { 
        <div onkeydown={restart_app} tabindex="0">
            <p>{ "Choreo4" }</p>
        </div>
    }
}