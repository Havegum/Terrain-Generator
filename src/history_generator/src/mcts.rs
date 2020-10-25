use super::civ::{Civilization, CivKnowledge, Priorities};
use super::board::ActionType;
use std::collections::HashMap;


pub struct SimulatedWorld {
    pub civilizations: Vec<Civilization>,
}

impl SimulatedWorld {
    pub fn new() -> Self {
        SimulatedWorld {
            civilizations: Vec::new(),
        }
    }

    pub fn find(&self, id: u32, priorities: &Priorities, perceptions: &HashMap<u32, CivKnowledge>) -> ActionType {
        ActionType::Grow
    }
}
