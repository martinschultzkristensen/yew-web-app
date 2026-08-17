// src/components/music_context.rs
use crate::components::molecules::sound_effects::get_audio_effect;
use log;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::{spawn_local, JsFuture};
use web_sys::{AudioBuffer, AudioBufferSourceNode, AudioContext, BaseAudioContext};
use yew::prelude::*;

const MUSIC_TRACK: &str = "low_8bit-menusong-short-ed.mp3";

#[derive(Clone, Debug, PartialEq)]
pub struct MusicContext {
    pub start_music: Callback<()>,
    pub stop_music: Callback<()>,
}

pub enum MusicContextAction {
    TrackLoaded(AudioBuffer),
    StartMusic,
    StopMusic,
}

#[derive(Properties, PartialEq)]
pub struct MusicContextProviderProps {
    #[prop_or_default]
    pub children: Children,
}

#[derive(Clone, PartialEq)]
pub struct MusicContextProvider {
    music_context: MusicContext,
    audio_context: AudioContext,
    buffer: Option<AudioBuffer>,
    current_source: Option<AudioBufferSourceNode>,
}

impl Component for MusicContextProvider {
    type Message = MusicContextAction;
    type Properties = MusicContextProviderProps;

    fn create(ctx: &Context<Self>) -> Self {
        let link = ctx.link().clone();
        let start_music =
            Callback::from(move |_| link.send_message(MusicContextAction::StartMusic));
        let link = ctx.link().clone();
        let stop_music = Callback::from(move |_| link.send_message(MusicContextAction::StopMusic));

        let music_context = MusicContext {
            start_music,
            stop_music,
        };

        let audio_context = match AudioContext::new() {
            Ok(ctx) => ctx,
            Err(e) => {
                log::error!("Failed to create AudioContext for music: {:?}", e);
                panic!("Failed to initialize music playback");
            }
        };

        // Loaded via the same backend byte-fetch + Web Audio decode path as
        // sound_effects.rs, instead of an <audio src>, because WebKitGTK on
        // Linux rejects the plain frontendDist static route with
        // NotSupportedError (it doesn't respond to Range requests with 206).
        {
            let link = ctx.link().clone();
            let audio_context = audio_context.clone();
            spawn_local(async move {
                match get_audio_effect(MUSIC_TRACK).await {
                    Ok(data) => {
                        let array_buffer = data.buffer();
                        match audio_context.decode_audio_data(&array_buffer) {
                            Ok(promise) => match JsFuture::from(promise).await {
                                Ok(buffer) => match buffer.dyn_into::<AudioBuffer>() {
                                    Ok(audio_buffer) => {
                                        link.send_message(MusicContextAction::TrackLoaded(
                                            audio_buffer,
                                        ));
                                    }
                                    Err(e) => {
                                        log::error!("Failed to convert music to AudioBuffer: {:?}", e)
                                    }
                                },
                                Err(e) => log::error!("Failed to decode music track: {:?}", e),
                            },
                            Err(e) => log::error!("Failed to start music decode: {:?}", e),
                        }
                    }
                    Err(e) => log::error!("Failed to load music track {}: {:?}", MUSIC_TRACK, e),
                }
            });
        }

        Self {
            music_context,
            audio_context,
            buffer: None,
            current_source: None,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            MusicContextAction::TrackLoaded(buffer) => {
                self.buffer = Some(buffer);
                false
            }
            MusicContextAction::StartMusic => {
                if let Some(source) = self.current_source.take() {
                    let _ = source.stop();
                }
                let Some(buffer) = &self.buffer else {
                    log::warn!("Music track not loaded yet; ignoring start request");
                    return false;
                };
                let source = match self.audio_context.create_buffer_source() {
                    Ok(s) => s,
                    Err(e) => {
                        log::error!("Failed to create music buffer source: {:?}", e);
                        return false;
                    }
                };
                source.set_buffer(Some(buffer));
                source.set_loop(true);
                if let Err(e) =
                    source.connect_with_audio_node(&self.audio_context.destination())
                {
                    log::error!("Failed to connect music source: {:?}", e);
                    return false;
                }
                if let Err(e) = source.start() {
                    log::error!("Failed to start music playback: {:?}", e);
                    return false;
                }
                self.current_source = Some(source);
                false
            }
            MusicContextAction::StopMusic => {
                if let Some(source) = self.current_source.take() {
                    let _ = source.stop();
                }
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <ContextProvider<MusicContext> context={self.music_context.clone()}>
                { for ctx.props().children.iter() }
            </ContextProvider<MusicContext>>
        }
    }
}
