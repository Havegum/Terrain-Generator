// use wasm_bindgen::prelude::*;
use std::collections::{HashSet, HashMap};
// use std::iter::FromIterator;
// use super::mcts::SimulatedWorld;

#[allow(unused_imports)]
use wasm_bindgen::{JsValue};
#[allow(unused_imports)]
use web_sys::console;

use rand_core::{SeedableRng};
use rand_pcg::Pcg32;
use std::sync::atomic::{AtomicUsize, Ordering};
use serde::{Serialize};

use super::board::{Board, Action, Cell};

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

const NAMES: [&str; 4] = ["Yellow", "Purple", "Red", "Blue"]; // "trokzoq", "filphond", "xuwgoll", "smaqjaul", "sirpheneo", "sprakloomu"];
const COLORS: [&str; 4] = ["#ffc107", "#9c27b0", "#f44336", "#005aa1"]; //, "#bf711e", "#7d49e2", "#79300f", "#9a95e5", "#dd858d", "#9f04fc"];

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

    pub fn spawn(board: &mut Board, starting_location: Vec<usize>) {
        let civ = Self::new_distinct(&board.civs);
        let civ_id = civ.id;
        board.civs.insert(civ.id, civ);
        for territory in starting_location {
            board.add_territory(civ_id, territory);
        }
        board.turn_order.push(civ_id);
    }
    
    pub fn get_actions(&self, cells: &Vec<Cell>) -> Vec<Action> {
        let occupy = self.neighbor_territory.iter()
            .map(|&i| Action::Occupy(i, cells[i].owner_civ_id));
        
        occupy.collect()
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
