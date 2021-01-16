use crate::{
    agent::{Action, Agent},
    magpie::othello::{OthelloBoard, Stone},
};

pub struct UnmotivatedAgent;

impl Agent for UnmotivatedAgent {
    fn play(&mut self, _stone: Stone, _board: &OthelloBoard) -> Action {
        Action::Pass
    }
}
