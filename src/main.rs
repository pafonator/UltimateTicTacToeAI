mod game;
mod template;
use std::{env, process};

use game::game_uttt::UtttEvaluator;
use game::node_uttt::UtttState;
use log::{debug, info};
use minimax::Strategy;
use template::tic_tac_toe::GridSlot;

extern crate minimax;

fn minimax(state: &UtttState, depth: u8) -> Option<(GridSlot, GridSlot)>{
    let evaluator = UtttEvaluator::new("UtttEvaluator".to_string());
    let mut strategy = minimax::Negamax::new(evaluator, depth);

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

    // Check if a JSON string is provided as an argument
    if args.len() != 3 {
        eprintln!("Usage: {} <Depth> <UtttState_JSON>", args[0]);
        process::exit(1);
    }

    let depth = args[1].parse::<u8>().expect("Failed to parse depth");
    let input_json = &args[2]; // The second argument is the JSON input

    debug!("Getting root Node from input JSON");
    // Deserialize the input JSON into UtttState
    let state: UtttState = serde_json::from_str(&input_json).expect("Failed to deserialize UtttState");
    
    let action = minimax(&state, depth);

    let json = serde_json::to_string(&action).unwrap();
    info!("Serialized best move:\n");
    println!("[RESULT] {}", json);
}
