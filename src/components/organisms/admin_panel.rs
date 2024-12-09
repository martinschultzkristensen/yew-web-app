use yew::prelude::*;

#[function_component(AdminPanel)]
pub fn admin_panel() -> Html {
    html! {
        <div>
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