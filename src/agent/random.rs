use crate::{
    agent::{Action, Agent},
    magpie::othello::{OthelloBoard, Stone, StoneExt},
};
use rand::{seq::IteratorRandom, SeedableRng};
use rand_pcg::Pcg64;
use rand_seeder::Seeder;

pub struct RandomAgent {
    rng: Pcg64,
}

impl RandomAgent {
    pub fn new() -> RandomAgent {
        RandomAgent {
            rng: Pcg64::from_entropy(),
        }
    }

    pub fn with_seed(seed: &str) -> RandomAgent {
        RandomAgent {
            rng: Seeder::from(seed).make_rng(),
        }
    }
}

impl Agent for RandomAgent {
    fn play(&mut self, stone: Stone, board: &OthelloBoard) -> Action {
        board
            .moves_for(stone)
            .stones()
            .choose(&mut self.rng)
            .map(Action::Move)
            .unwrap_or(Action::Pass)
    }
}
