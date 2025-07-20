mod game;
mod template;
use std::os::linux::raw::stat;
use std::time::Duration;
use std::{env, process};

use game::game_uttt::UtttEvaluator;
use game::node_uttt::UtttState;
use log::{debug, info};
use minimax::Strategy;
use template::tic_tac_toe::GridSlot;

extern crate minimax;

fn run(state: &UtttState, timeout: std::time::Duration) -> Option<(GridSlot, GridSlot)>{
    let evaluator = UtttEvaluator;

    let parallel_opt = minimax::ParallelOptions::new();
    parallel_opt.with_num_threads(16);
    //parallel_opt.with_background_pondering();
    let mut iter_opt = minimax::IterativeOptions::new();
    iter_opt.verbose = true;

    let mut strategy = minimax::ParallelSearch::new(evaluator,iter_opt,parallel_opt);
    strategy.set_timeout(timeout);

    let best_move = strategy.choose_move(state);
    //let best_move_seq = strategy.principal_variation();
    return best_move;
}


fn main() {
    // Initialize the logger with a default level
    env_logger::Builder::new()
        .filter_level(log::LevelFilter::Debug)
        .target(env_logger::Target::Stdout) 
        .init();
    
    // Get the command-line arguments
    let args: Vec<String> = env::args().collect();

    let state: UtttState;
    let duration :u64;
    if args.len() == 1 {
        // Look for optional --state argument
        duration = 10;
        state = UtttState::default();
    } else if args.len() != 3 {
        eprintln!("Usage: {} <Duration> <UtttState_JSON>", args[0]);
        process::exit(1);
    } else {
        duration = args[1].parse::<u64>().expect("Failed to parse duration");
        let input_json = &args[2]; // The second argument is the JSON input

        debug!("Getting root Node from input JSON");
        // Deserialize the input JSON into UtttState
        state = serde_json::from_str(&input_json).expect("Failed to deserialize UtttState");
    }
    
    let action = run(&state, Duration::new(duration,0));

    let json = serde_json::to_string(&action).unwrap();
    info!("Serialized best move:\n");
    println!("[RESULT] {}", json);
}

