use crate::components::atoms::use_focus_div::use_focus_div;
use crate::components::data::choreography_data::get_choreography_data;
use crate::Route;
use yew::prelude::*;
use yew_router::prelude::use_navigator;

#[derive(Properties, PartialEq)]
pub struct AboutChoreoProps {
    #[prop_or_default]
    pub choreo_number: usize,
}

#[function_component(AboutChoreo)]
pub fn about_choreo(props: &AboutChoreoProps) -> Html {
    let navigator = use_navigator().unwrap();
    let div_ref = use_focus_div(); // Hook sets focus on the div when the component mounts.
      
      // Get all data for this choreography
    let choreo_data = get_choreography_data(props.choreo_number);

    let event_key = Callback::from(move |event: KeyboardEvent| match event.key().as_str() {
        "q" => navigator.push(&Route::IntroScreen1),
        "r" => navigator.push_with_state(&Route::MainMenu, 0usize),
        "e" => navigator.push_with_state(&Route::ChoreoVideo, 0usize),
        _ => (),
    });

    html! {
        <div ref={div_ref} onkeydown={event_key} tabindex="1" class="about-choreo-container">
            // Title section
            <h1 class="title">{ &choreo_data.title }</h1>
            
            // Main choreography image
            <div class="choreo-image-container">
                <img src={choreo_data.choreo_image} alt={format!("Choreography {}", props.choreo_number)} />
            </div>
            
            // Dancers section
            <div class="dancers-container">
                {
                    choreo_data.dancer_images.iter().map(|image_path| {
                        html! {
                            <div class="dancer-image">
                                <img src={image_path.clone()} alt="Dancer" />
                            </div>
                        }
                    }).collect::<Html>()
                }
            </div>
            
            // Description section
            <p class="description">{ &choreo_data.description }</p>
        </div>
    }
}
