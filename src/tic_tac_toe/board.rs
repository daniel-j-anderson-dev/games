use std::ops::{Index, IndexMut};
use serde::Serialize;

use super::cell::{Cell, CellIndex};

#[derive(Debug, Default, Serialize, Clone, Copy)]
pub struct Board {
    board: [[Cell; 3]; 3],
}
impl Board {
    pub fn set_cell(&mut self, index: CellIndex, value: Cell) {
        self[index] = value;
    }
    pub fn get_cell(&self, index: CellIndex) -> Cell {
        return self[index];
    }
    /// Returns a cell representing the winner
    /// - [Cell::Empty] means no winner
    /// - [Cell::X] means `X`s win
    /// - [Cell::O] means `O`s win
    pub fn get_winner(&self) -> Cell {
        // TODO:
        unimplemented!();
    }

    // convince methods for iterating over every cell on the board
    pub fn iter(&self) -> impl Iterator<Item = Cell> {
        return self.board.into_iter().flatten();
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
        return self.board.iter_mut().flatten();
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
        return self.board.index_mut(i.row()).index_mut(i.column());
    }
}
impl Index<CellIndex> for Board {
    type Output = Cell;
    fn index(&self, i: CellIndex) -> &Self::Output {
        return self.board.index(i.row()).index(i.column());
    }
}