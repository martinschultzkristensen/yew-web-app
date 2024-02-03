use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub struct Audio {
    pub id: usize,
    pub url: String,
}

#[derive(Properties, PartialEq)]
pub struct AudiosListProps {
    pub audios: Vec<Audio>,
    pub current_index: usize, // New property for the current index
}

#[function_component(AudioList)]
pub fn audio_list(
    AudiosListProps {
        audios,
        current_index,
    }: 
    &AudiosListProps,) -> Html {
    // Use the current_index to display the corresponding audio
    let current_audio = &audios[*current_index];

    html! {
        <div>
            <audio src={format!("{}", current_audio.url)} autoplay=true loop=true />
        </div>
    }
}
