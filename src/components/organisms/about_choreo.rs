use crate::components::atoms::arrow_respnd_ui::*;
use crate::components::atoms::dancer::DancerCard;
use crate::components::data::choreography_data::{get_choreography_data, ChoreographyData};
use crate::components::molecules::music_context::*;
use crate::components::molecules::scollable_div::ScrollableDiv;
use crate::Route;
use yew::prelude::*;
use yew_router::prelude::use_navigator;
use wasm_bindgen_futures::spawn_local;
use gloo::console::log;

#[derive(Properties, PartialEq)]
pub struct AboutChoreoProps {
    pub choreo_number: usize,
}

#[function_component(AboutChoreo)]
pub fn about_choreo(props: &AboutChoreoProps) -> Html {
    log!("About Choreo Props:", props.choreo_number);
    let navigator = use_navigator().unwrap();
    let video_index = props.choreo_number - 1;
    let ctx = use_context::<MusicContext>().expect("No music context provider");
    let stop_music = ctx.stop_music.clone();

    // State to hold choreography data
    let choreo_data = use_state(|| None::<ChoreographyData>);
    
    // Load choreography data
    {
        let choreo_data = choreo_data.clone();
        let choreo_number = props.choreo_number;
        use_effect(
            move || {
                spawn_local(async move {
                    match get_choreography_data(choreo_number).await {
                        data => {
                            log!("Data loaded successfully");
                            choreo_data.set(Some(data));
                        }
                    }
                });
                || ()
            },
        );
    }

    let event_key = Callback::from(move |event: KeyboardEvent| match event.key().as_str() {
        "q" => {
            stop_music.emit(());
            navigator.push(&Route::IntroScreen1)
        }
        "r" => {
            let soundeffect =
                web_sys::HtmlAudioElement::new_with_src("/static/uiToAboutChoreo.mp3").unwrap();
            let _ = soundeffect.play();
            navigator.push_with_state(&Route::MainMenu, video_index)
        }
        "e" => {
            stop_music.emit(());
            navigator.push_with_state(&Route::ChoreoVideo, video_index);
        }
        _ => (),
    });

    match (*choreo_data).as_ref() {
        None => html! {
            <div class="about-choreo-container">
                <p>{"Loading..."}</p>
            </div>
        },
        Some(data) => html! {
            <ScrollableDiv onkeydown={event_key} tabindex="1" class="about-choreo-container">
                <div class="svg-arrow-in-about-top">
                    <ArrowUpIcon/>
                </div>
                <div class="arcadefont">
                    <h2>{ &data.title }</h2>
                    <div class="info-section-container">
                        <img src={data.choreo_image.clone()} alt={format!("Choreography {}", props.choreo_number)} />
                        <p class="description">{ &data.description }</p>
                    </div>
                    <h2>{"Dancers"}</h2>
                    {
                        data.dancers.iter().map(|dancer| {
                            html! {
                                <DancerCard dancer={dancer.clone()}/>
                            }
                        }).collect::<Html>()
                    }
                </div>
                <div class="svg-arrow-in-about-bottom">
                    <ArrowDownIcon/>
                </div>
            </ScrollableDiv>
        }
    }
}