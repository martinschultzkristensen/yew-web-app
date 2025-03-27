//main.rs
use yew_app::DanceOmatic;

fn main() {
    env_logger::init();
    yew::Renderer::<DanceOmatic>::new().render();
}
