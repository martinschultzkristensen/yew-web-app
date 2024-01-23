use yew::prelude::*;

#[function_component(IntroScreen)]
pub fn intro_screen() -> Html {
    let intro_video = get_intro_video();
    let x_to_exit = exit_video();
    html! {
        <div onkeydown={x_to_exit} tabindex="0">
            <VideosList videos={intro_video} />
        </div>
    }
}