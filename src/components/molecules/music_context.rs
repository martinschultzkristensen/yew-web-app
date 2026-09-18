// src/components/music_context.rs
use crate::components::molecules::sound_effects::{frontend_log, static_asset_url};
use log;
use web_sys::HtmlAudioElement;
use yew::prelude::*;

const MUSIC_TRACK: &str = "low_8bit-menusong-short-ed.mp3";

#[derive(Clone, Debug, PartialEq)]
pub struct MusicContext {
    pub start_music: Callback<()>,
    pub stop_music: Callback<()>,
}

pub enum MusicContextAction {
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
    audio_element: Option<HtmlAudioElement>,
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

        // The <audio> element (and the GStreamer playbin/pipewiresink pipeline behind
        // it) is created lazily on first StartMusic, not eagerly here. Creating it at
        // mount time raced directly against the intro video's own <video> element
        // autoplaying at the same instant - two playbin pipelines both initializing
        // pipewiresink concurrently deadlocked on PipeWire's shared per-process
        // thread-loop lock (confirmed via gdb: WebKit's main thread blocked in
        // libgstplayback.so's g_object_get, while a GStreamer streaming thread was
        // blocked on pw_thread_loop_lock() during autoaudiosink's pipewiresink
        // negotiation for a second, concurrent pipeline). By the time StartMusic
        // fires (the "x" keypress), the video's playbin is already stable.
        Self {
            music_context,
            audio_element: None,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            MusicContextAction::StartMusic => {
                let audio_element = self.audio_element.get_or_insert_with(|| {
                    // Played through a plain <audio> element pointed at the local media
                    // server (WebKitGTK's normal media pipeline) instead of the Web Audio
                    // decode+play path, which has proven unreliable over Bluetooth on this
                    // WebKitGTK/PipeWire setup (first silent, then distorted) despite
                    // several targeted PipeWire/GStreamer fixes. The <video> element's
                    // audio, using this same native pipeline, has stayed clean throughout.
                    let url = static_asset_url(MUSIC_TRACK);
                    match HtmlAudioElement::new_with_src(&url) {
                        Ok(el) => {
                            el.set_loop(true);
                            el
                        }
                        Err(e) => {
                            log::error!("Failed to create music audio element: {:?}", e);
                            panic!("Failed to initialize music playback");
                        }
                    }
                });
                audio_element.set_current_time(0.0);
                match audio_element.play() {
                    Ok(_) => {
                        log::info!("Started music playback");
                        frontend_log("info", "StartMusic: play() called".to_string());
                    }
                    Err(e) => {
                        log::error!("Failed to start music playback: {:?}", e);
                        frontend_log(
                            "warn",
                            format!("StartMusic: play() failed: {:?}", e),
                        );
                    }
                }
                false
            }
            MusicContextAction::StopMusic => {
                if let Some(audio_element) = &self.audio_element {
                    if let Err(e) = audio_element.pause() {
                        log::warn!("Failed to pause music: {:?}", e);
                    }
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
