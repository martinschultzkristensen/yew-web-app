//src/components/organisms/about_choreo.rs
// use crate::components::atoms::use_focus_div::use_focus_div;
use crate::components::atoms::dancer::DancerCard;
use crate::components::atoms::arrow_respnd_ui::*;
use crate::components::data::choreography_data::get_choreography_data;
use crate::components::molecules::scollable_div::ScrollableDiv;
use crate::components::molecules::music_context::*;
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
    let ctx = use_context::<MusicContext>().expect("No music context provider");    
    let stop_music = ctx.stop_music.clone();
      
    // Get all data for this choreography
    let choreo_data = get_choreography_data(props.choreo_number);

    let event_key = Callback::from(move |event: KeyboardEvent|{ 
        match event.key().as_str() {
        "q" => {
            stop_music.emit(());
            navigator.push(&Route::IntroScreen1)},
        "r" => {let soundeffect =
            web_sys::HtmlAudioElement::new_with_src("/static/uiToAboutChoreo.mp3").unwrap();
        let _ = soundeffect.play();
            navigator.push_with_state(&Route::MainMenu, video_index)},

        "e" => {
            stop_music.emit(());
            navigator.push_with_state(&Route::ChoreoVideo, video_index);
        }
        _ => (),
        }
    });

    html! {
        <ScrollableDiv onkeydown={event_key} tabindex="1" class="about-choreo-container">
            // <ArrowIcon class={classes!("svg-arrow-in-about-top")} is_up={true}/>
            <ArrowContainer class={classes!("svg-arrow-in-about-top")} show_top_arrow=true show_bottom_arrow=false is_interactive=false />
            // Title section
            <div class="arcadefont">
                <h2>{ &choreo_data.title }</h2> 
                // Main choreography image
                <div class="info-section-container">
                    <img src={choreo_data.choreo_image} alt={format!("Choreography {}", props.choreo_number)} />
                    // Description section
                    <p class="description">{ &choreo_data.description }</p>
                </div>
            // Dancers section
                <h2>{"Dancers"}</h2>
                {
                    choreo_data.dancers.iter().map(|dancer| {
                        html! {
                            <DancerCard
                            image={dancer.image.clone()}
                            name={dancer.name.clone()}
                            strength={dancer.strength}
                            flexibility={dancer.flexibility}
                        />
                        }
                    }).collect::<Html>()
                    }
                </div>
                    // <ArrowIcon class={classes!("svg-arrow-in-about-bottom")} is_up={false}/>
                    <ArrowContainer class={classes!("svg-arrow-in-about-bottom")} show_top_arrow=false show_bottom_arrow=true is_interactive=false />
        </ScrollableDiv>
    }
}


