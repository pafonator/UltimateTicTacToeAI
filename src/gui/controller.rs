use minimax::Strategy;

use crate::game::game_uttt::UtttEvaluator;
use crate::template::tic_tac_toe::{GridSlot, PieceType};
use crate::game::node_uttt::UtttState;
use crate::gui::app::{SmallBoard, Player, CellState};

/// Convert GUI board representation into an `UtttState`.
pub fn grid_to_utttstate(
    boards: &Vec<SmallBoard>,
    current_player: Player,
    active_board: Option<usize>,
) -> UtttState {
    let mut state = UtttState::default();

    for b in 0..9 {
        let outer_r = (b / 3) as usize;
        let outer_c = (b % 3) as usize;
        for c in 0..9 {
            let inner_r = (c / 3) as usize;
            let inner_c = (c % 3) as usize;
            let piece = match boards[b].cells[c] {
                CellState::Empty => PieceType::Empty,
                CellState::Occupied(Player::X) => PieceType::X,
                CellState::Occupied(Player::O) => PieceType::O,
            };
            state.ultra_grid.grid[outer_r][outer_c].grid[inner_r][inner_c] = piece;
        }
    }

    state.crosses_turn = current_player == Player::X;
    state.current_play_slot = if let Some(idx) = active_board {
        GridSlot(idx as u8)
    } else {
        GridSlot::ANY_SLOT
    };

    state
}

async fn run(state: &UtttState, timeout: std::time::Duration) -> Option<(GridSlot, GridSlot)>{
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
