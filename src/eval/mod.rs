use crate::magpie::othello::OthelloBoard;
use crate::magpie::othello::Stone;

pub mod stone_difference;

pub trait EvaluationFunction {
    fn evaluate(&mut self, stone: Stone, board: &OthelloBoard) -> i32;
}
