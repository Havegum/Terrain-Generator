// use wasm_bindgen::prelude::*;
use std::collections::HashSet;
// use super::mcts::SimulatedWorld;

#[allow(unused_imports)]
use wasm_bindgen::{JsValue};
#[allow(unused_imports)]
use web_sys::console;

use rand_core::{RngCore, SeedableRng};
use rand_pcg::Pcg32;
use std::sync::atomic::{AtomicUsize, Ordering};
use serde::{Serialize};

use super::board::{Board, ActionType};

#[allow(unused_macros)]
macro_rules! log {
    ( $( $t:tt )* ) => {
      console::log_1(
        &JsValue::from_str(
          format!( $( $t )* ).as_str()
        )
      );
    }
}

const NAMES: [&str; 10] = ["tidux", "houga", "finar", "omho", "trokzoq", "filphond", "xuwgoll", "smaqjaul", "sirpheneo", "sprakloomu"];
const COLORS: [&str; 10] = ["#fe5900", "#7d1a6e", "#fb4e93", "#406087", "#bf711e", "#7d49e2", "#79300f", "#9a95e5", "#dd858d", "#9f04fc"];

// From https://users.rust-lang.org/t/idiomatic-rust-way-to-generate-unique-id/33805
fn get_id() -> usize {
    static COUNTER:AtomicUsize = AtomicUsize::new(0);
    COUNTER.fetch_add(1, Ordering::Relaxed)
}

// #[derive(Serialize)]
// pub struct Color(u8, u8, u8);

#[derive(Serialize,Debug)]
pub struct Civilization {
    pub id: usize,
    pub name: String,
    pub color: String,
    pub territory: HashSet<usize>,
    #[serde(skip_serializing)]
    pub rng: Pcg32,
}

impl Civilization {
    pub fn new(id: usize, name: String, color: String, territory: HashSet<usize>) -> Civilization {
        Civilization {
            id,
            name,
            color,
            territory,
            rng: Pcg32::seed_from_u64(id as u64),
        }
    }

    fn new_distinct(_civs: &Vec<Civilization>) -> Civilization {
        // TODO: generate names and colors. Ensure unique.
        let id = get_id();
        let name = NAMES[id % NAMES.len()].to_string();
        let color = COLORS[id % COLORS.len()].to_string();
        let territory = HashSet::new();
        
        Civilization::new(id, name, color, territory)
    }

    pub fn spawn(civs: &Vec<Civilization>, truth: &mut Board, simulation: &mut Board) -> Civilization {
        let mut civ = Self::new_distinct(civs);
        loop {
            // TODO: better spawn location than random
            let candidate = civ.rng.next_u32() as usize % truth.cells.len();
            if truth.cells[candidate].owner_civ_id == None {
                civ.territory.insert(candidate);
                truth.cells[candidate].owner_civ_id = Some(civ.id);
                simulation.cells[candidate].owner_civ_id = Some(civ.id);
                return civ
            }
        }
    }

    pub fn choose_action(&mut self, simulation: &mut Board) -> ActionType {
        let mut candidates = HashSet::new();

        for i in self.territory.iter() {
            candidates.extend(&simulation.cells[*i].adjacent);
        }

        let candidates: Vec<usize> = candidates
            .into_iter()
            .filter(|i| !self.territory.contains(i))
            .collect();
        let random = self.rng.next_u32() as usize % candidates.len();
        ActionType::Occupy(candidates[random])
        
        // Perceptions of others must be fresh here. Maybe just call it just before finding actions
        // self.perceive_priorities(&mut simulation.civilizations);
        // let suggested_action: ActionType = simulation.find(self.id, &self.priorities, &self.perceptions);
        // suggested_action
        // unimplemented!();
    }

    pub fn score(&self) -> f64 {
        self.territory.len() as f64
    }
}

impl PartialEq for Civilization {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
