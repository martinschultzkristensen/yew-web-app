//src/components/molecules/sound_effects.rs
use log;
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::spawn_local;
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
                // A new element is created per play so overlapping sounds (e.g. rapid
                // scroll clicks) don't cut each other off.
                let url = static_asset_url(&effect_name);
                match HtmlAudioElement::new_with_src(&url) {
                    Ok(audio) => match audio.play() {
                        Ok(_) => {
                            log::info!("Started playing sound: {} ({})", effect_name, url);
                            frontend_log(
                                "info",
                                format!("PlaySound {}: play() called ({})", effect_name, url),
                            );
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
                    },
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
