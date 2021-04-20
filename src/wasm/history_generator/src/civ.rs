use rand_core::{RngCore, SeedableRng};
use rand_pcg::Pcg32;
use std::collections::HashMap;
use serde::{Serialize};

use super::board::ActionType;
// use super::mcts::SimulatedWorld;

#[derive(Serialize)]
pub struct Color(u8, u8, u8);

#[derive(Serialize)]
pub struct Civilization {
    id: u32,
    name: String,
    color: Color,
    territory: Vec<usize>,
    #[serde(skip_serializing)]
    rng: Pcg32,
}

impl Civilization {
    pub fn new(id: u32, name: String, color: Color, territory: Vec<usize>) -> Civilization {
        Civilization {
            id,
            name,
            color,
            territory,
            rng: Pcg32::seed_from_u64(id as u64),
        }
    }

    // pub fn find_action(&mut self, simulation: &mut SimulatedWorld) -> ActionType {
        // Perceptions of others must be fresh here. Maybe just call it just before finding actions
        // self.perceive_priorities(&mut simulation.civilizations);
        // let suggested_action: ActionType = simulation.find(self.id, &self.priorities, &self.perceptions);
        // suggested_action
        // unimplemented!();
    // }

    pub fn score(&self) -> f64 {
        self.territory.len() as f64
    }
}

impl PartialEq for Civilization {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
