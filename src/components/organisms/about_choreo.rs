//src/components/organisms/about_choreo.rs
use crate::components::atoms::arrow_respnd_ui::*;
use crate::components::atoms::dancer::DancerCard;
use crate::components::data::choreography_data::get_choreography_data;
use crate::components::molecules::music_context::*;
use crate::components::molecules::sound_effects::SoundEffectsContext;
use crate::components::molecules::scollable_div::ScrollableDiv;
use crate::components::molecules::btn_explainer_graphics::BtnExplainerGraphics;
use crate::Route;
use yew::prelude::*;
use yew_router::prelude::use_navigator;
use gloo::console::log;
use crate::Config;
use std::rc::Rc;

#[derive(Properties, PartialEq)]
pub struct AboutChoreoProps {
    pub choreo_number: usize,
    pub config: Rc<Config>,
}

#[function_component(AboutChoreo)]
pub fn about_choreo(props: &AboutChoreoProps) -> Html {
    let ctx = use_context::<MusicContext>().expect("No music context provider");
    
    let sound_context = use_context::<SoundEffectsContext>().expect("SoundEffectsContext not found");
    let play_sound = sound_context.play_sound.clone();
    
    let config = &props.config;
    let choreography_data = get_choreography_data(config, props.choreo_number);

    log!("About Choreo Props:", props.choreo_number);
    let navigator = use_navigator().unwrap();
    let video_index = props.choreo_number - 1;
    let stop_music = ctx.stop_music.clone();



    // State to hold choreography data
    let choreo_data = use_state(|| props.config.dancers.clone());
    let choreo_data = props.config.dancers.list.get(video_index)
        .expect("Choreography data not found for the given index");


    // Load choreography data
    // {
    //     let choreo_data = choreo_data.clone();
    //     let choreo_number = props.choreo_number;
    //     use_effect(
    //         move || {
    //             spawn_local(async move {
    //                 match get_choreography_data(choreo_number).await {
    //                     data => {
    //                         log!("Data loaded successfully");
    //                         choreo_data.set(Some(data));
    //                     }
    //                 }
    //             });
    //             || ()
    //         },
    //     );
    // }

    let event_key = Callback::from(move |event: KeyboardEvent| match event.key().as_str() {
        "q" => {
            stop_music.emit(());
            navigator.push(&Route::IntroScreen1)
        }
        "r" => {
            play_sound.emit("uiToAboutChoreo".to_string());
            navigator.push_with_state(&Route::MainMenu, video_index)
        }
        "e" => {
            stop_music.emit(());
            play_sound.emit("buttonSelect".to_string());
            navigator.push_with_state(&Route::ChoreoVideo, video_index);
        }
        _ => (),
    });

    html! {
        <ScrollableDiv onkeydown={event_key} tabindex="1" class="about-choreo-container">
            <div class="svg-arrow-in-about-top">
                <ArrowUpIcon/>
            </div>
            <div class="arcadefont">
                <h2>{ choreography_data.title.clone() }</h2>
                <p>{ format!("Config has {} demo videos", props.config.load_dancers().len())}</p>
                <div class="info-section-container">
                    <img src={choreography_data.choreo_image.clone()} 
                         alt={format!("Choreography {}", props.choreo_number)} />
                    <p class="description">{ choreography_data.description.clone() }</p>
                </div>
                <h2>{"Dancers"}</h2>
                {
                    choreography_data.dancers.iter().map(|dancer| {
                        html! {
                            <DancerCard dancer={dancer.clone()}/>
                        }
                    }).collect::<Html>()
                }
            </div>
            <div class="svg-arrow-in-about-bottom">
                <ArrowDownIcon/>
            </div>
            <BtnExplainerGraphics class="btn-container-about-choreo" data="/static/goBack.svg"/>
        </ScrollableDiv>
    }
}