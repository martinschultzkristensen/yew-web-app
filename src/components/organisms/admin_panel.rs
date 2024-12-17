use yew::prelude::*;
use yew_router::prelude::use_navigator;
use web_sys::HtmlInputElement;
use crate::Route;
use crate::components::atoms::use_focus_div::use_focus_div;
use crate::components::atoms::dancer::{Dancer, DancerManager};

#[function_component(AdminPanel)]
pub fn admin_panel() -> Html {
    let div_ref = use_focus_div();
    let navigator = use_navigator().unwrap();
    
    // State for managing dancers
    let dancers_manager = use_state(|| DancerManager::default());
    let new_dancer_name = use_state(|| String::new());
    let new_dancer_strength = use_state(|| 5);
    let new_dancer_flexibility = use_state(|| 5);
    let new_dancer_image = use_state(|| String::new());
    
    let restart_app = Callback::from(move |event: KeyboardEvent| {
        if event.ctrl_key() && event.shift_key() && event.key() == "Q" {
            navigator.push(&Route::IntroScreen1);
        }
    });
    
    // Callback to add a new dancer
    let add_dancer = {
        let dancers_manager = dancers_manager.clone();
        let new_dancer_name = new_dancer_name.clone();
        let new_dancer_strength = new_dancer_strength.clone();
        let new_dancer_flexibility = new_dancer_flexibility.clone();
        let new_dancer_image = new_dancer_image.clone();
        
        Callback::from(move |_: MouseEvent| {
            // Create a mutable copy of the current DancerManager
            let mut updated_manager = (*dancers_manager).clone();
            
            let result = updated_manager.add_dancer(
                (*new_dancer_image).clone(), 
                (*new_dancer_name).clone(), 
                *new_dancer_strength, 
                *new_dancer_flexibility
            );
            
            match result {
                Ok(_) => {
                    // Update the state with the modified manager
                    dancers_manager.set(updated_manager);
                    
                    // Reset input fields after successful addition
                    new_dancer_name.set(String::new());
                    new_dancer_strength.set(50);
                    new_dancer_flexibility.set(50);
                    new_dancer_image.set(String::new());
                },
                Err(e) => {
                    // TODO: Implement error handling (e.g., show error message)
                    web_sys::console::log_1(&e.into());
                }
            }
        })
    };
    
    html! {
        <div class="about-choreo-container" ref={div_ref} tabindex="0" onkeydown={restart_app}>
            <h1>{ "Admin Panel" }</h1>
            <p>{ "Welcome to the lightweight admin panel." }</p>
            
            <div>
                <h2>{ "Dancer Management" }</h2>
                
                // Dancer Creation Form
                <div>
                    <input 
                        type="text" 
                        placeholder="Dancer Name" 
                        value={(*new_dancer_name).clone()}
                        oninput={Callback::from(move |e: InputEvent| {
                            let input: HtmlInputElement = e.target_unchecked_into();
                            new_dancer_name.set(input.value());
                        })}
                    />
                    
                    <input 
                        type="number" 
                        placeholder="Strength (0-10)" 
                        min="0" 
                        max="10" 
                        value={new_dancer_strength.to_string()}
                        oninput={Callback::from(move |e: InputEvent| {
                            let input: HtmlInputElement = e.target_unchecked_into();
                            if let Ok(val) = input.value().parse() {
                                new_dancer_strength.set(val);
                            }
                        })}
                    />
                    
                    <input 
                        type="number" 
                        placeholder="Flexibility (0-10)" 
                        min="0" 
                        max="10" 
                        value={new_dancer_flexibility.to_string()}
                        oninput={Callback::from(move |e: InputEvent| {
                            let input: HtmlInputElement = e.target_unchecked_into();
                            if let Ok(val) = input.value().parse() {
                                new_dancer_flexibility.set(val);
                            }
                        })}
                    />
                    
                    <input 
                        type="text" 
                        placeholder="Image Path" 
                        value={(*new_dancer_image).clone()}
                        oninput={Callback::from(move |e: InputEvent| {
                            let input: HtmlInputElement = e.target_unchecked_into();
                            new_dancer_image.set(input.value());
                        })}
                    />
                    
                    <button onclick={add_dancer}>{ "Add Dancer" }</button>
                </div>
                
                // Dancer List
                <div>
                    <h3>{ "Existing Dancers" }</h3>
                    <ul>
                        { for dancers_manager.get_dancers().iter().map(|dancer| html! {
                            <li>
                                { format!("ID: {}, Name: {}, Strength: {}, Flexibility: {}", 
                                    dancer.id, 
                                    dancer.name, 
                                    dancer.strength, 
                                    dancer.flexibility
                                )}
                            </li>
                        })}
                    </ul>
                </div>
            </div>
            
            <ul>
                <li>{ "Upload Media" }</li>
                <li>{ "Modify Choreographies" }</li>
                <li>{ "Manage Settings" }</li>
            </ul>
        </div>
    }
}