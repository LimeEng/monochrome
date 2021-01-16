use crate::magpie::othello::{OthelloBoard, Stone};

pub mod stone_difference;

pub trait EvaluationFunction {
    fn evaluate(&mut self, stone: Stone, board: &OthelloBoard) -> i32;
}
