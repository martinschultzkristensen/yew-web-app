// audio_player.rs

use yew::prelude::*;
use yew::{Component, ComponentLink, html, ShouldRender};

pub struct AudioPlayer {
    link: ComponentLink<Self>,
}

impl Component for AudioPlayer {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            link: _ctx.link().clone(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        // Handle messages (no messages to handle in this example)
        false
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let on_keydown = self.link.callback(|event: KeyboardEvent| {
            if event.key() == "r" {
                play_audio();
            }
        });

        html! {
            <div onkeydown={on_keydown}>
            
                <p> {"Press 'r' to play audio"} </p>
            </div>
        }

    }
            
}

    fn play_audio() {
        // Play the audio here
        // For simplicity, let's assume playing audio involves some functionality
        // You'd use a library like web-sys to interact with the Web Audio API
        // For demonstration, we'll just print a message
        println!("Audio is played!");
    }