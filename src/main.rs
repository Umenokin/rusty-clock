mod components;
mod views;

use crate::views::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
