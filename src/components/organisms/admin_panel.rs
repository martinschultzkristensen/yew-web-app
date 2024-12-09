use yew::prelude::*;
use yew_router::prelude::use_navigator;
use crate::Route;
use crate::components::atoms::use_focus_div::use_focus_div;

#[function_component(AdminPanel)]
pub fn admin_panel() -> Html {
    let div_ref = use_focus_div();
    let navigator = use_navigator().unwrap();
    
    let restart_app = Callback::from(move |event: KeyboardEvent| {
        if event.ctrl_key() && event.shift_key() && event.key() == "Q" {
            navigator.push(&Route::IntroScreen1);
        }
    });
    html! {
        <div ref={div_ref} tabindex="0" onkeydown={restart_app}>
            <h1>{ "Admin Panel" }</h1>
            <p>{ "Welcome to the lightweight admin panel." }</p>
            <ul>
                <li>{ "Upload Media" }</li>
                <li>{ "Modify Choreographies" }</li>
                <li>{ "Manage Settings" }</li>
            </ul>
        </div>
    }
}