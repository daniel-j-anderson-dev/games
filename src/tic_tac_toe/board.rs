use std::ops::{Index, IndexMut};
use serde::{Deserialize, Serialize};

use super::cell::{Cell, CellIndex};

#[derive(Debug, Default, Serialize, Deserialize, Clone, Copy)]
pub struct Board {
    cells: [[Cell; 3]; 3],
}
impl Board {
    pub fn set_cell(&mut self, index: CellIndex, value: Cell) {
        self[index] = value;
    }
    pub fn get_cell(&self, index: CellIndex) -> Cell {
        return self[index];
    }
    pub fn get_winner(&self) -> Option<Winner> {
        // Check columns
        match [self[CellIndex::TOP_LEFT], self[CellIndex::MIDDLE_LEFT], self[CellIndex::BOTTOM_LEFT]] {
            [Cell::X, Cell::X, Cell::X] => return Some(Winner::X),
            [Cell::O, Cell::O, Cell::O] => return Some(Winner::O),
            _ => (),
        };
        match [self[CellIndex::TOP_MIDDLE], self[CellIndex::CENTER], self[CellIndex::BOTTOM_MIDDLE]] {
            [Cell::X, Cell::X, Cell::X] => return Some(Winner::X),
            [Cell::O, Cell::O, Cell::O] => return Some(Winner::O),
            _ => (),
        };
        match [self[CellIndex::TOP_RIGHT], self[CellIndex::MIDDLE_RIGHT], self[CellIndex::BOTTOM_RIGHT]] {
            [Cell::X, Cell::X, Cell::X] => return Some(Winner::X),
            [Cell::O, Cell::O, Cell::O] => return Some(Winner::O),
            _ => (),
        }
        // Check rows
        // Check diagonals
        // TODO:
        unimplemented!();
    }
    // convince methods for iterating over every cell on the board
    pub fn iter(&self) -> impl Iterator<Item = Cell> {
        return self.cells.into_iter().flatten();
    }
    pub fn iter_enumerated(&self) -> impl Iterator<Item = (CellIndex, Cell)> {
        return self.iter().enumerate().map(|(i, cell)| {
            (
                CellIndex::try_from(i).expect("`Board::iter` has 9 elements so `i` will never be larger than eight"),
                cell,
            )
        })
    }
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Cell> {
        return self.cells.iter_mut().flatten();
    }
    pub fn iter_mut_enumerated(&mut self) -> impl Iterator<Item = (CellIndex, &mut Cell)> {
        return self.iter_mut().enumerate().map(|(i, cell)| {
            (
                CellIndex::try_from(i).expect("`Board::iter_mut` has 9 elements so `i` will never be larger than eight"),
                cell,
            )
        })
    }
}
impl IndexMut<CellIndex> for Board {
    /// Never panics because `i.row()` and `i.column()` are granted valid by [CellIndex]
    fn index_mut(&mut self, i: CellIndex) -> &mut Self::Output {
        return self.cells.index_mut(i.row()).index_mut(i.column());
    }
}
impl Index<CellIndex> for Board {
    type Output = Cell;
    fn index(&self, i: CellIndex) -> &Self::Output {
        return self.cells.index(i.row()).index(i.column());
    }
}

pub enum Winner {
    X,
    O,
}