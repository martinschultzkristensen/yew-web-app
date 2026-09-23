//src/components/molecules/sound_effects.rs
use log;
use serde_wasm_bindgen::to_value;
use std::collections::HashMap;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::{spawn_local, JsFuture};
use web_sys::HtmlAudioElement;
use yew::prelude::*;

// Base URL of the local Axum media server started in local_media_server.rs; its
// "/static" route serves the same bundled resources/static/ directory the sound
// effect and music MP3s ship in.
const LOCAL_MEDIA_BASE_URL: &str = "http://127.0.0.1:17847";

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"], js_name = invoke)]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

// Writes into the backend's log file (tauri_plugin_log) instead of the browser
// console, which isn't practically reachable on a fullscreen kiosk. Fire-and-forget:
// logging failures shouldn't affect audio playback.
pub fn frontend_log(level: &str, message: String) {
    spawn_local({
        let level = level.to_string();
        async move {
            let args = to_value(&serde_json::json!({
                "level": level,
                "message": message,
            }))
            .expect("failed to serialize frontend_log args");
            invoke("frontend_log", args).await;
        }
    });
}

pub fn static_asset_url(filename: &str) -> String {
    format!("{LOCAL_MEDIA_BASE_URL}/static/{filename}")
}

// A WASM panic anywhere in the frontend otherwise only surfaces via
// console_error_panic_hook, i.e. the browser console -- which isn't reachable on
// this kiosk. That's a real blind spot: a startup hang has been seen where the
// backend log goes silent right after config load with no further activity,
// consistent with the frontend panicking during initial render before anything
// else could run. This hook keeps the normal console_error_panic_hook behavior
// (useful for `trunk serve`/local dev) and additionally forwards the panic
// message into danceOmatic.log via frontend_log, so a repeat of that hang leaves
// a trace of what actually happened.
pub fn install_panic_logging() {
    std::panic::set_hook(Box::new(|info| {
        console_error_panic_hook::hook(info);
        frontend_log("error", format!("WASM panic: {info}"));
    }));
}

// Surfaces MediaError (network/decode/src-not-supported) into danceOmatic.log.
// Without this a failed load is completely invisible: the element simply never
// produces sound and play() reports nothing.
fn attach_error_logging(audio: &HtmlAudioElement, effect_name: &str) {
    let element = audio.clone();
    let name = effect_name.to_string();
    let on_error = Closure::<dyn FnMut()>::new(move || {
        let detail = match element.error() {
            Some(err) => format!("code={} message={}", err.code(), err.message()),
            None => "no MediaError set".to_string(),
        };
        log::error!("Media error for {}: {}", name, detail);
        frontend_log("warn", format!("PlaySound {}: media error ({})", name, detail));
    });
    audio.set_onerror(Some(on_error.as_ref().unchecked_ref()));
    // The element outlives this call (it is cached in SoundEffectsProvider), so the
    // closure has to outlive it too.
    on_error.forget();
}

#[derive(Clone, Debug, PartialEq)]
pub struct SoundEffectsContext {
    pub play_sound: Callback<String>,
}

pub enum SoundEffectsAction {
    PlaySound(String),
}

#[derive(Properties, PartialEq)]
pub struct SoundEffectsProviderProps {
    #[prop_or_default]
    pub children: Children,
}

#[derive(Clone, PartialEq)]
pub struct SoundEffectsProvider {
    sound_effects_context: SoundEffectsContext,
    // One reusable <audio> element per effect, created on first play and rewound
    // on every play after that. Two reasons it is a cache rather than a fresh
    // element per play:
    //
    // 1. Lifetime. A bare `HtmlAudioElement` that is never inserted into the DOM
    //    is kept alive only by WebKit's "a *playing* media element stays alive"
    //    rule -- which does not cover an element still loading. Over Bluetooth the
    //    sink takes ~1s to come up, which is exactly that window.
    // 2. Pipeline churn. Every new element builds a fresh GStreamer
    //    playbin+pipewiresink. Reusing one keeps a single, already-negotiated
    //    pipeline per effect instead of renegotiating the Bluetooth sink each time
    //    (see the concurrent-negotiation deadlock documented in music_context.rs).
    elements: HashMap<String, HtmlAudioElement>,
}

impl Component for SoundEffectsProvider {
    type Message = SoundEffectsAction;
    type Properties = SoundEffectsProviderProps;

    fn create(ctx: &Context<Self>) -> Self {
        log::info!("Initializing SoundEffectsProvider");
        let link = ctx.link().clone();
        let play_sound = Callback::from(move |effect_name: String| {
            link.send_message(SoundEffectsAction::PlaySound(effect_name))
        });

        let sound_effects_context = SoundEffectsContext { play_sound };

        Self {
            sound_effects_context,
            elements: HashMap::new(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            SoundEffectsAction::PlaySound(effect_name) => {
                // Played through a plain <audio> element pointed at the local media
                // server (WebKitGTK's normal media pipeline) instead of the Web Audio
                // decode+play path, which has proven unreliable over Bluetooth on this
                // WebKitGTK/PipeWire setup (first silent, then distorted) despite
                // several targeted PipeWire/GStreamer fixes. The <video> element's
                // audio, using this same native pipeline, has stayed clean throughout.
                let url = static_asset_url(&effect_name);

                if !self.elements.contains_key(&effect_name) {
                    match HtmlAudioElement::new_with_src(&url) {
                        Ok(audio) => {
                            audio.set_preload("auto");
                            attach_error_logging(&audio, &effect_name);
                            self.elements.insert(effect_name.clone(), audio);
                        }
                        Err(e) => {
                            log::error!(
                                "Failed to create audio element for {}: {:?}",
                                effect_name,
                                e
                            );
                            frontend_log(
                                "warn",
                                format!(
                                    "PlaySound {}: failed to create <audio> element: {:?}",
                                    effect_name, e
                                ),
                            );
                            return false;
                        }
                    }
                }

                let audio = &self.elements[&effect_name];
                audio.set_current_time(0.0);
                match audio.play() {
                    Ok(promise) => {
                        log::info!("Started playing sound: {} ({})", effect_name, url);
                        // play() resolving is the only signal that audio actually
                        // reached the sink; the Ok() here just means the promise was
                        // created, which is true even when loading or decoding fails.
                        let name = effect_name.clone();
                        spawn_local(async move {
                            match JsFuture::from(promise).await {
                                Ok(_) => frontend_log(
                                    "info",
                                    format!("PlaySound {}: playback started", name),
                                ),
                                Err(e) => frontend_log(
                                    "warn",
                                    format!("PlaySound {}: play() rejected: {:?}", name, e),
                                ),
                            }
                        });
                    }
                    Err(e) => {
                        log::error!(
                            "Failed to start audio playback for {}: {:?}",
                            effect_name,
                            e
                        );
                        frontend_log(
                            "warn",
                            format!("PlaySound {}: play() failed: {:?}", effect_name, e),
                        );
                    }
                }
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <ContextProvider<SoundEffectsContext> context={self.sound_effects_context.clone()}>
                { for ctx.props().children.iter() }
            </ContextProvider<SoundEffectsContext>>
        }
    }
}
