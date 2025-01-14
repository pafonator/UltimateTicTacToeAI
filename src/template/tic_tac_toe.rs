use std::cmp::Ordering;

use serde::{Deserialize, Serialize};


#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Default, Hash, Serialize, Deserialize)]
pub enum PieceType {
    X = 1,
    O = -1,
    #[default] Empty = 0
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash, Serialize, Deserialize)]
pub struct GridSlot(pub u8);
impl GridSlot {
    pub const ANY_SLOT: GridSlot = GridSlot(9);
    pub const ALL_SLOTS: [GridSlot; 9] = [
        GridSlot(0), GridSlot(1), GridSlot(2),
        GridSlot(3), GridSlot(4), GridSlot(5),
        GridSlot(6), GridSlot(7), GridSlot(8)
        ];
    pub const ALL_LINES: [[GridSlot; 3]; 8] = [
        [GridSlot(0), GridSlot(1), GridSlot(2)], //Horizontal
        [GridSlot(3), GridSlot(4), GridSlot(5)], 
        [GridSlot(6), GridSlot(7), GridSlot(8)], 
        [GridSlot(0), GridSlot(3), GridSlot(6)], //Vertical
        [GridSlot(1), GridSlot(4), GridSlot(7)], 
        [GridSlot(2), GridSlot(5), GridSlot(8)], 
        [GridSlot(0), GridSlot(4), GridSlot(8)], //Main diagonal
        [GridSlot(2), GridSlot(4), GridSlot(6)], //Anti diagonal
    ];
}
impl From<(u8, u8)> for GridSlot {
    fn from(value: (u8, u8)) -> Self {
        return GridSlot(value.0 * 3 + value.1)
    }
}
impl Into<(u8, u8)> for GridSlot {
    fn into(self) -> (u8, u8) {
        return (self.0 / 3, self.0 % 3)
    }
}
impl Into<u8> for GridSlot {
    fn into(self) -> u8 {
        return self.0
    }
}


#[derive(Debug, PartialEq, Eq, PartialOrd, Clone, Copy, Default, Hash, Serialize, Deserialize)]
pub struct Grid3x3<T>{
    pub grid: [[T; 3]; 3],
}
impl<T> Ord for Grid3x3<T> 
where T: Ord{
    fn cmp(&self, other: &Self) -> Ordering {
        for i in 0..3 {
            for j in 0..3 {
                let val = self.grid[i][j].cmp(&other.grid[i][j]);
                if val != Ordering::Equal {
                    return val;
                }
            }
        }
        return Ordering::Equal
    }
}

impl<T> Grid3x3<T>
{
    pub fn get(&self, slot : GridSlot) -> &T {
        let (i,j) = slot.into();
        return &self.grid[i as usize][j as usize];
    }

    pub fn get_mut(&mut self, slot : GridSlot) -> &mut T {
        let (i,j) = slot.into();
        return &mut self.grid[i as usize][j as usize];
    }
}