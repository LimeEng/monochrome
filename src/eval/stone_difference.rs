use crate::eval::EvaluationFunction;
use crate::magpie::othello_board::OthelloBoard;
use crate::magpie::stone::Stone;

pub struct StoneDifference;

impl EvaluationFunction for StoneDifference {
    fn evaluate(&mut self, stone: Stone, board: &OthelloBoard) -> i32 {
        let agent_stones = board.bits_for(stone).count_ones();
        let opponent_stones = board.bits_for(stone.flip()).count_ones();

        agent_stones as i32 - opponent_stones as i32
    }
}
