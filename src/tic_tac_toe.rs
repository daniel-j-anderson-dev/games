pub mod board;
pub mod cell;

pub use self::{
    board::Board,
    cell::{Cell, CellIndex},
};

pub use serde::{Serialize, Deserialize};

#[derive(Default, Debug, Deserialize, Serialize, Clone, Copy)]
pub struct GameState {
    pub board: Board,
    pub is_x_turn: bool,
    pub is_over: bool,
}

