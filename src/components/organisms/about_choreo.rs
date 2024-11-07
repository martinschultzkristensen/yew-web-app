use crate::components::atoms::use_focus_div::use_focus_div;
use crate::components::data::choreography_data::get_choreography_data;
use crate::components::molecules::scollable_div::ScrollableDiv;
use crate::Route;
use yew::prelude::*;
use yew_router::prelude::use_navigator;


#[derive(Properties, PartialEq)]
pub struct AboutChoreoProps {
    pub choreo_number: usize,
}

#[function_component(AboutChoreo)]
pub fn about_choreo(props: &AboutChoreoProps) -> Html {
    let navigator = use_navigator().unwrap();
    let video_index = props.choreo_number - 1;
    let video_index = video_index; // Clone for closure

      
      // Get all data for this choreography
    let choreo_data = get_choreography_data(props.choreo_number);

    let event_key = Callback::from(move |event: KeyboardEvent|{ 
        match event.key().as_str() {
        "q" => navigator.push(&Route::IntroScreen1),
        "r" => navigator.push_with_state(&Route::MainMenu, video_index),
        "e" => navigator.push_with_state(&Route::ChoreoVideo, video_index),
        _ => (),
        }
    });

    html! {
        <ScrollableDiv onkeydown={event_key} tabindex="1" class="about-choreo-container">
            // Title section
            <h1 class="arcadefont">{ &choreo_data.title }</h1>
            
            // Main choreography image
            <div class="info-section-container">
                <img src={choreo_data.choreo_image} alt={format!("Choreography {}", props.choreo_number)} />
                // Description section
            <p class="description arcadefont">{ &choreo_data.description }</p>
            </div>
            
            // Dancers section
                <h1 class="arcadefont">{"Dancers"}</h1>
                {
                    choreo_data.dancers.iter().map(|dancer| {
                        html! {
                            <div class="info-section-container">
                                <img src={dancer.image.clone()} alt="Dancer" />
                                <p class="arcadefont">{dancer.name.clone()}</p>
                            </div>
                        }
                    }).collect::<Html>()
                }
        </ScrollableDiv>
    }
}


