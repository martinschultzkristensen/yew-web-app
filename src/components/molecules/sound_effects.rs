//src/components/molecules/sound_effects.rs
use yew::prelude::*;
use web_sys::HtmlAudioElement;
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
pub struct SoundEffectsContext {
    pub effects: HashMap<String, NodeRef>,
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
        let link = ctx.link().clone();
        let play_sound = Callback::from(move |effect_name: String| {
            link.send_message(SoundEffectsAction::PlaySound(effect_name))
        });

        let mut effects = HashMap::new();
        effects.insert("uiToAboutChoreo".to_string(), NodeRef::default());
        effects.insert("buttonSelect".to_string(), NodeRef::default());
        effects.insert("toggleUpDown".to_string(), NodeRef::default());


        let sound_effects_context = SoundEffectsContext {
            effects,
            play_sound,
        };

        Self { sound_effects_context }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
    let SoundEffectsAction::PlaySound(effect_name) = msg;

    if let Some(audio_ref) = self.sound_effects_context.effects.get(&effect_name) {
        if let Some(audio) = audio_ref.cast::<HtmlAudioElement>() {
            let _ = audio.set_current_time(0.0);
            let _ = audio.play();
        }
    }
    false // false Prevents unnecessary re-renders, but I think it has no effect true or false!
}


    fn view(&self, ctx: &Context<Self>) -> Html {
        let ui_to_about_choreo = self.sound_effects_context.effects.get("uiToAboutChoreo").unwrap();
        let button_select = self.sound_effects_context.effects.get("buttonSelect").unwrap();
        let toggle_up_down = self.sound_effects_context.effects.get("toggleUpDown").unwrap();

        html! {
            <ContextProvider<SoundEffectsContext> context={self.sound_effects_context.clone()}>
                <audio ref={ui_to_about_choreo.clone()} src="/static/uiToAboutChoreo.mp3" />
                <audio ref={button_select.clone()} src="/static/BtnStart.mp3" />
                <audio ref={toggle_up_down.clone()} src="/static/button-124476.mp3" />
                { for ctx.props().children.iter() }
            </ContextProvider<SoundEffectsContext>>
        }
    }
}