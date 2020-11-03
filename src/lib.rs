#![recursion_limit = "256"]
use wasm_bindgen::prelude::*;

mod app;
mod components;
mod pages;
mod route;

#[wasm_bindgen(start)]
pub fn run_app() {
    yew::App::<app::App>::new().mount_to_body();
    // yew::start_app::<App>();
}
