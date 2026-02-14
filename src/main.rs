mod game;
mod template;
mod gui;
use std::time::Duration;
use std::{env, process};

use gui::app::App;
use game::game_uttt::UtttEvaluator;
use game::node_uttt::UtttState;
use log::{debug, info};
use minimax::Strategy;
use template::tic_tac_toe::GridSlot;
use leptos::*;

extern crate minimax;

fn main() {
    console_error_panic_hook::set_once();

    // Initialize wasm-logger so logs appear in browser console
    wasm_logger::init(wasm_logger::Config::default());

    mount_to_body(|| view! { <App/> })
}