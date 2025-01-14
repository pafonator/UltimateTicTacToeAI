use crate::template::tic_tac_toe::{Grid3x3, GridSlot, PieceType};

use super::game_uttt::{BoardScore, GridScore};

pub trait Playable {
    fn is_playable(&self) -> bool;
}

impl Playable for PieceType {
    fn is_playable(&self) -> bool {
        return *self == PieceType::Empty;
    }
}

impl<T> Playable for Grid3x3<T> where T: Playable + GridScore {
    fn is_playable(&self) -> bool {
        let winner = self.score();
        match winner {
            BoardScore::WinnerX => return false,
            BoardScore::WinnerO => return false,
            _ => {}
        }

        for s in GridSlot::ALL_SLOTS {
            if self.get(s).is_playable() { return true; }
        }
        return false
    }
}