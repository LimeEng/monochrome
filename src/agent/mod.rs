use crate::magpie::othello::OthelloBoard;
use crate::magpie::othello::Stone;

pub mod unmotivated;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Action {
    Move(u64),
    Pass,
}

pub trait Agent {
    fn play(&mut self, stone: Stone, board: &OthelloBoard) -> Action;
}
