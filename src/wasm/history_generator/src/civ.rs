use wasm_bindgen::prelude::*;
use wasm_bindgen::{JsValue};
use web_sys::console;

use rand_core::{RngCore, SeedableRng};
use rand_pcg::Pcg32;
use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};
use serde::{Serialize};

use super::board::ActionType;
// use super::mcts::SimulatedWorld;

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
const COLORS: [&str; 10] = ["#b69cfd", "#6e9f23", "#ca55c1", "#96b299", "#c6523e", "#0ba47e", "#e91451", "#11677e", "#c19384", "#5756a0"];

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
    pub territory: Vec<usize>,
    #[serde(skip_serializing)]
    rng: Pcg32,
}

impl Civilization {
    pub fn new(id: usize, name: String, color: String, territory: Vec<usize>) -> Civilization {
        Civilization {
            id,
            name,
            color,
            territory,
            rng: Pcg32::seed_from_u64(id as u64),
        }
    }

    pub fn new_distinct(civs: &Vec<Civilization>) -> Civilization {
        let id = get_id();
        let name = NAMES[id % NAMES.len()].to_string();
        let color = COLORS[id % COLORS.len()].to_string();
        let territory: Vec<usize> = vec![];
        
        Civilization::new(id, name, color, territory)
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
