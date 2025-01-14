use crate::template::tic_tac_toe::{Grid3x3, GridSlot, PieceType};

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

pub fn parse_ultragrid(input: &str) -> Grid3x3<Grid3x3<PieceType>> {
    let mut ultra_grid = Grid3x3 {
        grid: [[Grid3x3 {
            grid: [[PieceType::Empty; 3]; 3],
        }; 3]; 3],
    };


    let mut char_idx = 0;
    for big_row in 0..3 {
        for small_row in 0..3 {
            for big_col in 0..3 {
                for small_col in 0..3 {
                    let mut piece: Option<PieceType>;
                    loop {
                        piece = match input.chars().nth(char_idx).unwrap() {
                            'X' => Some(PieceType::X),
                            'O' => Some(PieceType::O),
                            '.' => Some(PieceType::Empty),
                            _ => None,
                        };
                        char_idx += 1;
                        if piece.is_some() {break;}
                    }
                    ultra_grid.grid[big_row][big_col].grid[small_row][small_col] = piece.unwrap();
                }
            }
        }
    }

    ultra_grid
}