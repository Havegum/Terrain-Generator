use super::civ::{Civilization, CivKnowledge, Priorities};
use super::board::ActionType;
use std::collections::HashMap;


pub struct Simulation {
    pub civilizations: Vec<Civilization>,
}

impl Simulation {
    pub fn new() -> Self {
        Simulation {
            civilizations: Vec::new(),
        }
    }

    pub fn find(&self, id: u32, priorities: &Priorities, perceptions: &HashMap<u32, CivKnowledge>) -> ActionType {
        ActionType::Grow
    }
}
