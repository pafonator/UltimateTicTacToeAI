use crate::template::tic_tac_toe::{GridSlot, PieceType};

use super::node_uttt::UltraGrid;


pub fn string_ultragrid(grid: &UltraGrid) -> String {
    let mut s = String::new();
    s += "\n";
    for i in 0..9 {
        for j in 0..9 {
            let piece = grid.get(
                GridSlot::from((i/3 as u8 ,j/3 as u8))
            ).get(GridSlot::from((i%3 as u8,j%3 as u8 )));
            if j%3 == 0 && j != 0 { s += "|"; }
            if piece == &PieceType::Empty {
                s += " . ";
            }else if piece == &PieceType::X {
                s += " X ";
            }else if piece == &PieceType::O {
                s += " O ";
            }

        }
        s += "\n";
        if i%3 == 2 && i != 8 { s += "-----------------------------\n"; }
    }
    return s;
}