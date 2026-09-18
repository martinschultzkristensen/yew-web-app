//src/components/molecules/sound_effects.rs
use log;
use serde_wasm_bindgen::to_value;
use std::collections::HashMap;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::spawn_local;
use wasm_bindgen_futures::JsFuture;
use web_sys::{AudioBuffer, AudioContext};
use yew::prelude::*;

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

pub async fn get_audio_effect(effect_name: &str) -> Result<js_sys::Uint8Array, JsValue> {
    log::info!("Requesting audio effect from backend: {}", effect_name);
    let args = to_value(&serde_json::json!({
        "effectName": effect_name,
    }))
    .map_err(|e| format!("Failed to serialize args: {:?}", e))?;

    let result = invoke("get_audio_effect", args).await;
    log::info!("Received response from backend for: {}", effect_name);

    let array = js_sys::Uint8Array::new(&result);
    log::info!("Created Uint8Array with length: {}", array.length());
    Ok(array)
}

#[derive(Clone, Debug, PartialEq)]
pub struct SoundEffectsContext {
    pub audio_context: AudioContext,
    pub effects: HashMap<String, AudioBuffer>,
    pub play_sound: Callback<String>,
}

pub enum SoundEffectsAction {
    PlaySound(String),
    LoadEffect(String, AudioBuffer),
}

#[derive(Properties, PartialEq)]
pub struct SoundEffectsProviderProps {
    pub audio_context: AudioContext,
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
        // Shared with MusicContextProvider (created once in lib.rs) rather than each
        // provider creating its own AudioContext - see the comment in lib.rs for why:
        // two concurrent Web Audio streams attached to the same Bluetooth sink broke
        // both of them.
        let audio_context = ctx.props().audio_context.clone();
        let effects = HashMap::new();
        let link = ctx.link().clone();

        // Pre-load effects
        let effect_names = vec![
            "uiToAboutChoreo.mp3",
            "BtnStart.mp3",
            "button-124476.mp3",
            "coinSound.mp3",
        ];
        log::info!("About to load {} sound effects", effect_names.len());
        for name in effect_names {
            spawn_local({
                let audio_context = audio_context.clone();
                let link = link.clone();
                async move {
                    log::info!("Starting to load effect: {}", name);
                    match get_audio_effect(name).await {
                        Ok(data) => {
                            log::info!(
                                "Received data for effect: {} with length: {}",
                                name,
                                data.length()
                            );
                            let array_buffer = data.buffer();
                            log::info!("Created array buffer for: {}", name);

                            let raw_byte_len = data.length();
                            let decode_result = audio_context.decode_audio_data(&array_buffer);
                            match decode_result {
                                Ok(promise) => {
                                    log::info!("Starting decode for: {}", name);
                                    match JsFuture::from(promise).await {
                                        Ok(buffer) => {
                                            log::info!("Successfully decoded: {}", name);
                                            match buffer.dyn_into::<AudioBuffer>() {
                                                Ok(audio_buffer) => {
                                                    log::info!(
                                                        "Successfully created AudioBuffer for: {}",
                                                        name
                                                    );
                                                    frontend_log(
                                                        "info",
                                                        format!(
                                                            "decodeAudioData resolved for {}: raw_bytes={}, duration={}, length={}, channels={}",
                                                            name,
                                                            raw_byte_len,
                                                            audio_buffer.duration(),
                                                            audio_buffer.length(),
                                                            audio_buffer.number_of_channels()
                                                        ),
                                                    );
                                                    link.send_message(
                                                        SoundEffectsAction::LoadEffect(
                                                            name.to_string(),
                                                            audio_buffer,
                                                        ),
                                                    );
                                                }
                                                Err(e) => {
                                                    log::error!("Failed to convert to AudioBuffer for {}: {:?}", name, e);
                                                    frontend_log(
                                                        "warn",
                                                        format!(
                                                            "decodeAudioData resolved for {} (raw_bytes={}) but result wasn't an AudioBuffer: {:?}",
                                                            name, raw_byte_len, e
                                                        ),
                                                    );
                                                }
                                            }
                                        }
                                        Err(e) => {
                                            log::error!(
                                                "Failed to await decode for {}: {:?}",
                                                name,
                                                e
                                            );
                                            frontend_log(
                                                "warn",
                                                format!(
                                                    "decodeAudioData rejected for {} (raw_bytes={}): {:?}",
                                                    name, raw_byte_len, e
                                                ),
                                            );
                                        }
                                    }
                                }
                                Err(e) => {
                                    log::error!("Failed to start decode for {}: {:?}", name, e);
                                    frontend_log(
                                        "warn",
                                        format!(
                                            "decodeAudioData failed to start for {} (raw_bytes={}): {:?}",
                                            name, raw_byte_len, e
                                        ),
                                    );
                                }
                            }
                        }
                        Err(e) => {
                            log::error!("Failed to load audio effect {}: {:?}", name, e);
                        }
                    }
                }
            });
        }

        let play_sound = Callback::from(move |effect_name: String| {
            link.send_message(SoundEffectsAction::PlaySound(effect_name))
        });

        let sound_effects_context = SoundEffectsContext {
            audio_context,
            effects,
            play_sound,
        };

        Self {
            sound_effects_context,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            SoundEffectsAction::PlaySound(effect_name) => {
                let audio_context = &self.sound_effects_context.audio_context;
                let state_before = audio_context.state();
                log::info!(
                    "Attempting to play sound: {} (AudioContext state: {:?})",
                    effect_name,
                    state_before
                );
                // AudioContext can start (or drift back into) a "suspended" state -
                // e.g. the browser's autoplay policy, or the output device that was
                // default when the context was created going away. resume() is a
                // cheap no-op when already running, so it's safe to call on every
                // playback attempt rather than trying to track state ourselves.
                let resume_result = audio_context.resume();
                if let Err(e) = &resume_result {
                    log::warn!("AudioContext::resume() failed for {}: {:?}", effect_name, e);
                }
                frontend_log(
                    if resume_result.is_err() { "warn" } else { "info" },
                    format!(
                        "PlaySound {}: state before resume={:?}, resume()={}",
                        effect_name,
                        state_before,
                        if resume_result.is_ok() { "ok" } else { "failed" }
                    ),
                );
                if let Some(buffer) = self.sound_effects_context.effects.get(&effect_name) {
                    log::info!("Found audio buffer for: {}", effect_name);
                    let source = match self
                        .sound_effects_context
                        .audio_context
                        .create_buffer_source()
                    {
                        Ok(s) => s,
                        Err(e) => {
                            log::error!("Failed to create buffer source: {:?}", e);
                            return false;
                        }
                    };
                    source.set_buffer(Some(buffer));
                    // Some of the bundled sound effects are mastered right at 0dBFS
                    // (e.g. BtnStart.mp3, button-124476.mp3 measured -0.2/-0.4dB true
                    // peak). MP3 decoding can produce momentary reconstruction peaks
                    // above the encoded level even with no resampling involved, and
                    // AudioContext's destination hard-clips with zero headroom - which
                    // matched the distortion seen identically on every output device.
                    // A small gain reduction gives that overshoot somewhere to go
                    // before it clips.
                    let gain_node = match self.sound_effects_context.audio_context.create_gain() {
                        Ok(g) => {
                            g.gain().set_value(0.7);
                            g
                        }
                        Err(e) => {
                            log::error!("Failed to create gain node: {:?}", e);
                            return false;
                        }
                    };
                    let destination = self.sound_effects_context.audio_context.destination();
                    match source
                        .connect_with_audio_node(&gain_node)
                        .and_then(|_| gain_node.connect_with_audio_node(&destination))
                    {
                        Ok(_) => {
                            log::info!("Successfully connected audio nodes");
                            match source.start() {
                                Ok(_) => log::info!("Started playing sound: {}", effect_name),
                                Err(e) => log::error!("Failed to start audio playback: {:?}", e),
                            }
                        }
                        Err(e) => log::error!("Failed to connect audio nodes: {:?}", e),
                    }
                } else {
                    log::error!("No audio buffer found for: {}", effect_name);
                    log::info!(
                        "Available effects: {:?}",
                        self.sound_effects_context.effects.keys()
                    );
                }
                false
            }
            SoundEffectsAction::LoadEffect(name, buffer) => {
                log::info!("Loading effect: {}", name);
                self.sound_effects_context
                    .effects
                    .insert(name.clone(), buffer);
                log::info!("Successfully loaded effect: {}", name);
                true
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
