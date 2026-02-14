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

fn run(state: &UtttState, timeout: std::time::Duration) -> Option<(GridSlot, GridSlot)>{
    let evaluator = UtttEvaluator;

    //let parallel_opt = minimax::ParallelOptions::new();
    //parallel_opt.with_num_threads(16);
    //parallel_opt.with_background_pondering();
    let mut iter_opt = minimax::IterativeOptions::new();
    iter_opt.verbose = true;

    let mut strategy = minimax::IterativeSearch::new(evaluator,iter_opt);
    //let mut strategy = minimax::ParallelSearch::new(evaluator,iter_opt,parallel_opt);
    strategy.set_timeout(timeout);

    let best_move = strategy.choose_move(state);
    //let best_move_seq = strategy.principal_variation();
    return best_move;
}

fn main() {
    console_error_panic_hook::set_once();

    // Initialize wasm-logger so logs appear in browser console
    wasm_logger::init(wasm_logger::Config::default());

    mount_to_body(|| view! { <App/> })
}