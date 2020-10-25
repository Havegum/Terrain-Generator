use rand_core::{RngCore, SeedableRng};
use rand_pcg::Pcg32;
use std::collections::HashMap;

use super::board::ActionType;
use super::mcts::SimulatedWorld;

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

pub struct Civilization {
    id: u32,
    name: String,
    color: Color,
    income: f32,
    wealth: f32,
    priorities: Priorities,
    // secrecy: f32,
    // feature: add this as a constant uncertainty to others about their priorities
    // Secrecy will cool over time, and can be reheated with an action.
    perceptions: HashMap<u32, CivKnowledge>, // indexed by id ... probably
    rng: Pcg32,
}

impl Civilization {
    pub fn new(
        id: u32,
        name: String,
        color: Color,
        income: f32,
        wealth: f32,
        priorities: Priorities,
    ) -> Civilization {
        Civilization {
            id,
            name,
            color,
            income,
            wealth,
            priorities,
            perceptions: HashMap::new(),
            rng: Pcg32::seed_from_u64(id as u64),
        }
    }

    pub fn find_action(&mut self, simulation: &mut SimulatedWorld) -> ActionType {
        // Perceptions of others must be fresh here. Maybe just call it just before finding actions
        self.perceive_priorities(&mut simulation.civilizations);
        let suggested_action: ActionType = simulation.find(self.id, &self.priorities, &self.perceptions);
        suggested_action
    }

    pub fn perceive_priorities(&mut self, others: &mut Vec<Civilization>) {
        // Civ knowledge is persistent across turns. We don't reinitialize perceptions, but
        // rather update our beliefs (in the future, maybe in the bayesian sense).
        for foreign_civ in others {
            if foreign_civ == self {
                continue;
            }

            let knowledge = self.perceptions.get(&foreign_civ.id);
            match knowledge {
                None => {
                    let uncertainty = 1.;
                    let perceived_priorities =
                        foreign_civ.glean_priorities(uncertainty, &self.priorities);

                    let knowledge = CivKnowledge {
                        uncertainty,
                        priorities: perceived_priorities,
                    };
                    self.perceptions.insert(foreign_civ.id, knowledge);
                }
                Some(knowledge) => {
                    let perceived_priorities =
                        foreign_civ.glean_priorities(knowledge.uncertainty, &self.priorities);

                    //  TODO: uncertainty decays as a function of time, proximity, and interaction
                    let knowledge = CivKnowledge {
                        uncertainty: knowledge.uncertainty,
                        priorities: perceived_priorities,
                    };
                    self.perceptions.insert(foreign_civ.id, knowledge);
                }
            }
        }

        // TODO: remove dead civs?
    }

    pub fn glean_priorities(
        &mut self,
        uncertainty: f32,
        personal_priorities: &Priorities,
    ) -> Priorities {
        // We lean towards our own priorities. At least 25%, at most 75%
        let mut territory = personal_priorities.territory * (0.25 + uncertainty * 0.5);
        let mut income = personal_priorities.income * (0.25 + uncertainty * 0.5);
        let mut wealth = personal_priorities.wealth * (0.25 + uncertainty * 0.5);
        let mut population = personal_priorities.population * (0.25 + uncertainty * 0.5);

        territory += (self.rng(uncertainty) + self.priorities.territory) * (1.0 - uncertainty);
        income += (self.rng(uncertainty) + self.priorities.income) * (1.0 - uncertainty);
        wealth += (self.rng(uncertainty) + self.priorities.wealth) * (1.0 - uncertainty);
        population += (self.rng(uncertainty) + self.priorities.population) * (1.0 - uncertainty);

        let sum = territory + income + wealth + population;

        territory /= sum;
        income /= sum;
        wealth /= sum;
        population /= sum;

        Priorities {
            territory,
            income,
            wealth,
            population,
        }
    }

    fn rng(&mut self, range: f32) -> f32 {
        range * self.rng.next_u32() as f32 / u32::MAX as f32
    }
}

impl PartialEq for Civilization {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
