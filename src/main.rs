mod game;
mod template;
mod gui;

use gui::app::App;
use leptos::*;

extern crate minimax;

fn main() {
    console_error_panic_hook::set_once();

    // Initialize wasm-logger so logs appear in browser console
    wasm_logger::init(wasm_logger::Config::default());

    mount_to_body(|| view! { <App/> })
}