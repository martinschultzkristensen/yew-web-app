use crate::Route;
use yew::prelude::*;
use yew_router::prelude::use_navigator;

#[function_component(AboutChoreo1)]
pub fn about_choreo1() -> Html {
    let navigator = use_navigator().unwrap();
    let event_key = Callback::from(move |event: KeyboardEvent| match event.key().as_str() {
        "q" => navigator.push(&Route::IntroScreen1),
        "r" => navigator.push_with_state(&Route::MainMenu, 0usize),
        "e" => navigator.push_with_state(&Route::ChoreoVideo, 0usize),
        _ => (),
    });

    html! {
        <div onkeydown={event_key} tabindex="0">
            <p>{ "Choreo1" }</p>
        </div>
    }
}

#[function_component(AboutChoreo2)]
pub fn about_choreo2() -> Html {
    let navigator = use_navigator().unwrap();
    let event_key = Callback::from(move |event: KeyboardEvent| match event.key().as_str() {
        "q" => navigator.push(&Route::IntroScreen1),
        "r" => navigator.push_with_state(&Route::MainMenu, 1usize),
        "e" => navigator.push_with_state(&Route::ChoreoVideo, 1usize),
        _ => (),
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
    let event_key = Callback::from(move |event: KeyboardEvent| match event.key().as_str() {
        "q" => navigator.push(&Route::IntroScreen1),
        "r" => navigator.push_with_state(&Route::MainMenu, 2usize),
        "e" => navigator.push_with_state(&Route::ChoreoVideo, 2usize),
        _ => (),
    });

    html! {
        <div onkeydown={event_key} tabindex="0">
            <p>{ "Choreo3" }</p>
        </div>
    }
}

#[function_component(AboutChoreo4)]
pub fn about_choreo4() -> Html {
    let navigator = use_navigator().unwrap();
    let restart_app = Callback::from(move |event: KeyboardEvent| match event.key().as_str() {
        "q" => navigator.push(&Route::IntroScreen1),
        "r" => navigator.push_with_state(&Route::MainMenu, 3usize),
        "e" => navigator.push_with_state(&Route::ChoreoVideo, 3usize),
        _ => (),
    });

    html! {
        <div onkeydown={restart_app} tabindex="0">
            <p>{ "Choreo4" }</p>
        </div>
    }
}
