// src/components/music_context.rs
use yew::prelude::*;
use web_sys::HtmlAudioElement;



#[derive(Clone, Debug, PartialEq)]
pub struct MusicContext {
    pub audio_ref: NodeRef,
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
}

impl Component for MusicContextProvider {
    type Message = MusicContextAction; 
    type Properties = MusicContextProviderProps;

    //note: is there a more elegant way than cloning after each move |_|? Thoughts: consumes 
    fn create(ctx: &Context<Self>) -> Self {
        let link = ctx.link().clone();
        let start_music = Callback::from(move |_| link.send_message(MusicContextAction::StartMusic));
        let link = ctx.link().clone();
        let stop_music = Callback::from(move |_| link.send_message(MusicContextAction::StopMusic));


        let music_context = MusicContext {
            audio_ref: NodeRef::default(),
            start_music,
            stop_music,
        };

        Self { music_context }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        if let Some(audio) = self.music_context.audio_ref.cast::<HtmlAudioElement>() {
            match msg {
                MusicContextAction::StartMusic => {
                    let _ = audio.play().unwrap();
                }
                MusicContextAction::StopMusic => {
                    let _ = audio.pause();
                    audio.set_current_time(0.0);
            }}
        }
        true
    }
    

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <ContextProvider<MusicContext> context={self.music_context.clone()}>
                <audio 
                    ref={self.music_context.audio_ref.clone()} 
                    src="/static/8bit-menusong-short-ed.mp3" 
                    loop=true 
                />
                { for ctx.props().children.iter() }
            </ContextProvider<MusicContext>>
        }
    }
}