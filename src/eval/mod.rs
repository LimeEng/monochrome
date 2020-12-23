use crate::magpie::othello_board::OthelloBoard;
use crate::magpie::stone::Stone;

pub mod stone_difference;

pub trait EvaluationFunction {
    fn evaluate(&mut self, stone: Stone, board: &OthelloBoard) -> i32;
}
