
use std::string;

use minimax::{Evaluator, Game, Winner};

use crate::template::tic_tac_toe::{Grid3x3, GridSlot, PieceType};

use super::{ node_uttt::{UtttState}, playable::Playable};


#[derive(Debug, PartialEq)]
pub enum BoardScore {
    WinnerX,
    WinnerO,
    Draw,
    InProgress(f64),
}

pub trait GridScore {
    fn score(&self) -> BoardScore;
}
impl GridScore for PieceType {
    fn score(&self) -> BoardScore {
        if *self == PieceType::X {
            return BoardScore::WinnerX
        } else if *self == PieceType::O {
            return BoardScore::WinnerO
        } else {
            return BoardScore::InProgress(0.0)
        };
    }
}
impl<T> GridScore for Grid3x3<T> where T: GridScore{

    fn score(&self) -> BoardScore {

        let line_score: fn([&T; 3]) -> BoardScore = |line: [&T; 3]| {
            let mut score: f64 = 0.0;
            let mut has_x = 0;
            let mut has_o = 0;
            for &cell in &line {
                let cell_score = cell.score();
                match cell_score {
                    BoardScore::WinnerX => {
                        has_x += 1; 
                        score += 1.0;
                    },
                    BoardScore::WinnerO => {
                        has_o += 1;
                        score -= 1.0;
                    },
                    BoardScore::Draw => {
                        return BoardScore::Draw;
                    },
                    BoardScore::InProgress(val) => {
                        score += val;
                    }
                }
            }
            if has_x == 3 {
                return BoardScore::WinnerX;
            } else if has_o == 3 {
                return BoardScore::WinnerO;
            }else if has_x > 0 && has_o > 0 {
                return BoardScore::Draw;
            }
            return BoardScore::InProgress(score/3.0);
        };

        let mut score = 0.0;
        let mut is_winnable = false;
        let mut l_score: BoardScore;

        // Check lines
        for line_slots in GridSlot::ALL_LINES {
            l_score = line_score(line_slots.map(|slot| self.get(slot)));
            match l_score {
                BoardScore::WinnerX => return BoardScore::WinnerX,
                BoardScore::WinnerO => return BoardScore::WinnerO,
                BoardScore::Draw => score += 0.0,
                BoardScore::InProgress(val) => {
                    is_winnable = true;
                    score += val
                }
            }
        }

        if is_winnable {
            return BoardScore::InProgress(score/8.0);
        }else {
            return BoardScore::Draw;
        }
    }
}



impl Game for UtttState{
    type S = UtttState;
    type M = (GridSlot,GridSlot);

    fn generate_moves(state: &Self::S, moves: &mut Vec<Self::M>) {
        let primary_slot = state.current_play_slot;
        if primary_slot == GridSlot::ANY_SLOT {
            for s1 in GridSlot::ALL_SLOTS {
                if !state.ultra_grid.get(s1).is_playable() {
                    continue;
                }
                for s2 in GridSlot::ALL_SLOTS {
                    if state.ultra_grid.get(s1).get(s2) == &PieceType::Empty {
                        moves.push((s1, s2));
                    }
                }
            }
        }else{
            for s in GridSlot::ALL_SLOTS {
                if state.ultra_grid.get(primary_slot).get(s) == &PieceType::Empty {
                    moves.push((primary_slot, s));
                }
            }
        }
    }

    fn apply(state: &mut Self::S, m: Self::M) -> Option<Self::S> {
        let mut new_state_grid = state.ultra_grid.clone();
        let (i,j) = m;
        let last_piece_placed = if state.crosses_turn { PieceType::X } else { PieceType::O };
        *new_state_grid.get_mut(i).get_mut(j) = last_piece_placed;

        let current_play_slot;
        if !new_state_grid.get(j ).is_playable() {
            current_play_slot = GridSlot::ANY_SLOT;
        }else {
            current_play_slot = j;
        }
        
        let new_state = UtttState { 
            ultra_grid: new_state_grid, 
            crosses_turn: !state.crosses_turn, 
            current_play_slot
        };

        return Some(new_state)
    }

    fn get_winner(state: &Self::S) -> Option<Winner> {
        let score = state.ultra_grid.score();
        match score {
            BoardScore::WinnerX => {
                if state.crosses_turn {
                    return Some(Winner::PlayerToMove)
                } else {
                    return Some(Winner::PlayerJustMoved)
                }
            },
            BoardScore::WinnerO => {
                if state.crosses_turn {
                    return Some(Winner::PlayerJustMoved)
                } else {
                    return Some(Winner::PlayerToMove)
                }
            },
            BoardScore::Draw => return Some(Winner::Draw),
            BoardScore::InProgress(_) => return None,
        }
    }
}

pub struct UtttEvaluator {
    name: String
}
impl Evaluator for UtttEvaluator{
    type G = UtttState;

    fn evaluate(&self, s: &<Self::G as Game>::S) -> minimax::Evaluation {

        let score = s.ultra_grid.score();
        let val = match score {
            BoardScore::WinnerX => 1.0,
            BoardScore::WinnerO => -1.0,
            BoardScore::Draw => 0.0,
            BoardScore::InProgress(val) => val,
        };
        let res = (val * (i16::MAX-1) as f64) as minimax::Evaluation;
        if s.crosses_turn {
            return res;
        }else{
            return -res;
        }
    }
}
impl UtttEvaluator {
    pub fn new(name: String) -> Self {
        return Self {
            name
        }
    }
}