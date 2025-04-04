//main.rs
use yew_app::DanceOmatic;
use cfg_if::cfg_if;

//Environment	Logger	Output
// Web (Trunk/Yew) --> wasm_logger --> Browser Console
// Tauri Native	--> env_logger or tauri::log --> Terminal / Dev Tools

fn main() {
    cfg_if! {
        if #[cfg(target_arch = "wasm32")] {
            wasm_logger::init(wasm_logger::Config::default());
            console_error_panic_hook::set_once();
        }
    else {
        // For native (Tauri backend, CLI, etc.)
        env_logger::init();
    }
}
    yew::Renderer::<DanceOmatic>::new().render();
}
