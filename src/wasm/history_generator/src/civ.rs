// use wasm_bindgen::prelude::*;
use std::collections::{HashSet, HashMap};
use std::iter::FromIterator;
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
    pub neighbor_territory: HashSet<usize>,
    #[serde(skip_serializing)]
    pub rng: Pcg32,
}

impl Civilization {
    pub fn new(id: usize, name: String, color: String) -> Civilization {
        Civilization {
            id,
            name,
            color,
            territory: HashSet::new(),
            neighbor_territory: HashSet::new(),
            rng: Pcg32::seed_from_u64(id as u64),
        }
    }

    fn new_distinct(_civs: &HashMap<usize, Civilization>) -> Civilization {
        // TODO: generate names and colors. Ensure unique.
        let id = get_id();
        let name = NAMES[id % NAMES.len()].to_string();
        let color = COLORS[id % COLORS.len()].to_string();

        Civilization::new(id, name, color)
    }

    pub fn spawn(
        civs: &HashMap<usize, Civilization>,
        truth: &mut Board,
        simulation: &mut Board,
        starting_location: Vec<usize>,
    ) -> Civilization {
        let mut civ = Self::new_distinct(civs);
        for territory in starting_location {
            civ.add_territory(truth, territory);
            simulation.cells[territory].owner_civ_id = Some(civ.id);
        }
        civ
        // loop {
        //     // TODO: better spawn location than random
        //     let candidate = civ.rng.next_u32() as usize % truth.cells.len();
        //     if truth.cells[candidate].owner_civ_id == None {
        //         civ.territory.insert(candidate);
        //         truth.cells[candidate].owner_civ_id = Some(civ.id);
        //         simulation.cells[candidate].owner_civ_id = Some(civ.id);
        //         return civ
        //     }
        // }
    }

    pub fn choose_action(&mut self, simulation: &mut Board) -> ActionType {
        let candidates = Vec::from_iter(self.neighbor_territory.clone());

        log!("{:?}", self.territory);
        let random = self.rng.next_u32() as usize % candidates.len();

        ActionType::Occupy(candidates[random])
        
        // Perceptions of others must be fresh here. Maybe just call it just before finding actions
        // self.perceive_priorities(&mut simulation.civilizations);
        // let suggested_action: ActionType = simulation.find(self.id, &self.priorities, &self.perceptions);
        // suggested_action
        // unimplemented!();
    }

    pub fn add_territory(&mut self, board: &mut Board, territory: usize) {
        self.territory.insert(territory);
        board.cells[territory].owner_civ_id = Some(self.id);
        self.neighbor_territory.extend(
            board.cells[territory].adjacent.difference(&self.territory).collect::<HashSet<&usize>>()
        );
        self.neighbor_territory.remove(&territory);
    }

    pub fn remove_territory(&mut self, board: &mut Board, territory: usize) {
        self.territory.remove(&territory);
        board.cells[territory].owner_civ_id = None;

        let mut neighbors_neighbours = board.cells[territory].adjacent.clone();
        // neighbors_neighbours.insert(territory);

        for &n in neighbors_neighbours.iter() {
            let neighbours_owned_cell = board.cells[n].adjacent
                .iter()
                .any(|&nn| board.cells[nn].owner_civ_id == Some(self.id));

            if neighbours_owned_cell {
                self.neighbor_territory.insert(n);
            }
        }
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
