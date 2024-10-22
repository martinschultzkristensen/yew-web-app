use yew::prelude::*;
use gloo_timers::callback::Timeout;

#[function_component(BtnExplainerGraphics)]
pub fn btn_explainer_graphics() -> Html {
    let is_start_visible = use_state(|| false);

    {
        let is_start_visible = is_start_visible.clone();
        use_effect(
            move || {
                let timeout = Timeout::new(5200, move || {
                    is_start_visible.set(true);
                });
                timeout.forget();
                || ()
            }
        );
    }

    let start_class = if *is_start_visible {
        "txt_start txt_start-visible"
    } else {
        "txt_start"
    };

    html! {
       <div class="btn-explainer-container">
            <object type="image/svg+xml" data="static/greenBtn.svg" class="green-btn"></object>
            <object type="image/svg+xml" data="static/yellow_btn.svg" class="yellow-btn"></object>
            <object type="image/svg+xml" data="static/start.svg" class={start_class}></object>
            <object type="image/svg+xml" data="static/pointFinger.svg" class="point-finger"></object>
        </div>
    }
}
