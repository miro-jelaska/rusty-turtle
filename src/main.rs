mod domain;
mod views;

use views::App;
use log::Level;

fn main() {
    console_log::init_with_level(Level::Debug)
        .expect("console_log could not be initialized.");
    yew::Renderer::<App>::new().render();
}
