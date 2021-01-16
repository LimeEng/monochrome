use crate::magpie::othello::{OthelloBoard, Stone};

pub mod random;
pub mod unmotivated;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Action {
    Move(u64),
    Pass,
}

pub trait Agent {
    fn play(&mut self, stone: Stone, board: &OthelloBoard) -> Action;
}
