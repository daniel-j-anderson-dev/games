pub mod board;
pub mod cell;

pub use self::{
    board::Board,
    cell::{Cell, CellIndex},
};

pub use serde::{Serialize, Deserialize};

#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
pub struct GameState {
    pub board: Board,
    pub is_x_turn: bool,
    pub winner: Option<Winner>,
}
impl Default for GameState {
    fn default() -> Self {
        return Self {
            board: Board::default(),
            is_x_turn: true,
            winner: None,
        }
    }
}
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Winner {
    X,
    O,
}
