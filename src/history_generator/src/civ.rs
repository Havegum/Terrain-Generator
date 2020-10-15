use super::board::ActionType;
use std::collections::HashMap;

#[derive(PartialEq)]
pub struct Color(u8, u8, u8);

pub struct Priorities {
    territory: f32,
    income: f32,
    wealth: f32,
    population: f32,
}

pub struct CivKnowledge {
    priorities: Priorities,
    uncertainty: f32, // 0–1 .. probably
}

impl CivKnowledge {
    pub fn new() -> CivKnowledge {
        CivKnowledge {
            priorities: Priorities {
                territory: 0.25,
                income: 0.25,
                wealth: 0.25,
                population: 0.25,
            },
            uncertainty: 1.0,
        }
    }
}

#[derive(PartialEq)]
pub struct Civilization {
    name: String,
    color: Color,
    income: f32,
    wealth: f32,
    id: u32,
    perceptions: HashMap<u32, CivKnowledge>, // indexed by id ... probably
}

impl Civilization {
    pub fn find_action(&mut self, simulation: &mut SimulatedWorld) -> ActionType {
        // Perceptions of others must be fresh here. Maybe just call it just before finding actions
        // self.perceive_priorities(others);
        let suggestedAction: ActionType = simulation.find(self.priorities, self.perceptions);

        panic!("not implemented");
        suggestedAction
    }

    pub fn perceive_priorities(&mut self, others: Vec<Civilization>) {
        for foreign_civ in others {
            if !self.perceptions.contains_key(foreign_civ.id) {
                // Add to map, something like:
                self.perceptions.insert(foreign_civ.id, CivKnowledge::new());
            }

            // perceptions are randomly off relative to uncertainty
            // Probably leaning towards their own priorities if high uncertainty:
            // high uncertainty => 0.75 * personal_priorities + 0.25 perceived_priorities
            perceived_priorities.skew(uncertainty, personal_priorities);
            perceived_priorities.normalize();
        }

        panic!("not implemented");
    }
}
