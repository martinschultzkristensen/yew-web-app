use crate::components::atoms::use_focus_div::use_focus_div;
use crate::Route;
use yew::prelude::*;
use yew_router::prelude::use_navigator;

#[function_component(AboutChoreo1)]
pub fn about_choreo1() -> Html {
    let navigator = use_navigator().unwrap();
    let div_ref = use_focus_div(); // Hook sets focus on the div when the component mounts.
    let event_key = Callback::from(move |event: KeyboardEvent| match event.key().as_str() {
        "q" => navigator.push(&Route::IntroScreen1),
        "r" => navigator.push_with_state(&Route::MainMenu, 0usize),
        "e" => navigator.push_with_state(&Route::ChoreoVideo, 0usize),
        _ => (),
    });

    html! {
            <div ref={div_ref} onkeydown={event_key} tabindex="0">
                <p>{ "Choreo1" }</p>
                <svg>
                <defs>
        <symbol id="star" viewBox="0 0 100 100">
          <polygon points="50,15 61,35 82,35 67,50 73,72 50,60 27,72 33,50 18,35 39,35" />
        </symbol>
      </defs>
      <use href="#star" x="0" y="0" width="50" height="50" />
      <use href="#star" x="50" y="50" width="50" height="50" />
      <use href="#star" x="100" y="0" width="50" height="50" />
    </svg>
            </div>
        }
}

#[function_component(AboutChoreo2)]
pub fn about_choreo2() -> Html {
    let navigator = use_navigator().unwrap();
    let div_ref = use_focus_div();
    let event_key = Callback::from(move |event: KeyboardEvent| match event.key().as_str() {
        "q" => navigator.push(&Route::IntroScreen1),
        "r" => navigator.push_with_state(&Route::MainMenu, 1usize),
        "e" => navigator.push_with_state(&Route::ChoreoVideo, 1usize),
        _ => (),
    });

    html! {
        <div ref={div_ref} onkeydown={event_key} tabindex="1">
            <p>{ "Choreo2" }</p>
        </div>
    }
}

#[function_component(AboutChoreo3)]
pub fn about_choreo3() -> Html {
    let navigator = use_navigator().unwrap();
    let div_ref = use_focus_div();
    let event_key = Callback::from(move |event: KeyboardEvent| match event.key().as_str() {
        "q" => navigator.push(&Route::IntroScreen1),
        "r" => navigator.push_with_state(&Route::MainMenu, 2usize),
        "e" => navigator.push_with_state(&Route::ChoreoVideo, 2usize),
        _ => (),
    });

    html! {
        <div ref={div_ref} onkeydown={event_key} tabindex="0">
            <p>{ "Choreo3" }</p>
        </div>
    }
}

#[function_component(AboutChoreo4)]
pub fn about_choreo4() -> Html {
    let navigator = use_navigator().unwrap();
    let div_ref = use_focus_div();
    let restart_app = Callback::from(move |event: KeyboardEvent| match event.key().as_str() {
        "q" => navigator.push(&Route::IntroScreen1),
        "r" => navigator.push_with_state(&Route::MainMenu, 3usize),
        "e" => navigator.push_with_state(&Route::ChoreoVideo, 3usize),
        _ => (),
    });

    html! {
        <div ref={div_ref} onkeydown={restart_app} tabindex="0">
            <p>{ "Choreo4" }</p>
        </div>
    }
}
