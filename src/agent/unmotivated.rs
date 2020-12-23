use crate::agent::Action;
use crate::agent::Agent;
use crate::magpie::othello_board::OthelloBoard;
use crate::magpie::stone::Stone;

pub struct UnmotivatedAgent;

impl Agent for UnmotivatedAgent {
    fn play(&mut self, _stone: Stone, _board: &OthelloBoard) -> Action {
        Action::Pass
    }
}
