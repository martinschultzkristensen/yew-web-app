use yew::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::js_sys::Promise;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "window"])]
    fn setFullscreen(fullscreen: bool) -> Promise;
}

#[function_component(FullscreenEnforcer)]
pub fn fullscreen_enforcer() -> Html {
    use_effect_with(
        (),
        |_| {
            wasm_bindgen_futures::spawn_local(async {
                let promise = setFullscreen(true);
                let _ = JsFuture::from(promise).await;
            });
            || ()
        },
    );

    html! {}  // This component doesn't render anything
}
