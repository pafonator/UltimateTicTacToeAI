use std::{fmt::Debug, hash::Hash};
use serde::{Deserialize, Serialize};

use crate::template::tic_tac_toe::{Grid3x3, GridSlot, PieceType};

use super::tools::string_ultragrid;

pub type UltraGrid = Grid3x3<Grid3x3<PieceType>>;

#[derive(Debug, Clone, Hash, Eq, PartialEq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct UtttState {
    pub ultra_grid: UltraGrid,
    pub crosses_turn: bool,
    pub current_play_slot: GridSlot,
}
impl Default for UtttState {
    fn default() -> Self {
        return Self {
            ultra_grid: UltraGrid::default(),
            crosses_turn: true,
            current_play_slot: GridSlot::ANY_SLOT,
        }
    }
}
impl ToString for UtttState {
    fn to_string(&self) -> String {
        let mut str = String::new();
        let play_slot: (u8, u8) = self.current_play_slot.into();
        str += &string_ultragrid(&self.ultra_grid);
        str += format!("\nCurrent player: {}\n", if self.crosses_turn { "X" } else { "O" }).as_str();
        str += format!("Current play slot: {:?}\n", play_slot).as_str();
        return str
    }
}