use wasm_bindgen::prelude::*;
use wasm_bindgen::{JsValue};
use serde::{Serialize, Deserialize};
use web_sys::console;

use rand_core::{RngCore, SeedableRng};
use rand_pcg::Pcg32;
use std::collections::HashMap;

use super::board::{Board, Move};
use super::civ::Civilization;

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

#[derive(Deserialize)]
pub struct World {
  pub adjacencies: Vec<Vec<usize>>,
  pub heights: Vec<f64>,
  #[serde(rename = "seaLevel")] 
  pub sea_level: f64,
}

struct SimulationOptions {
  seed: u32,
  initial_civs: u32,
  // turns: u32,
}

// This struct mostly serves as the JS-WASM interface
#[wasm_bindgen(readonly)]
#[derive(Serialize)]
pub struct Simulation {
  board: Board,
}

impl Simulation {
  fn new(world: World, simulation_options: SimulationOptions) -> Simulation {
    let mut rng = Pcg32::seed_from_u64(simulation_options.seed as u64);

    let turn_order = Vec::with_capacity(simulation_options.initial_civs as usize);
    let civs = HashMap::with_capacity(simulation_options.initial_civs as usize);
    let mut board = Board::new(world, civs, turn_order);

    for _ in 0..simulation_options.initial_civs {
      let starting_location = loop {
        // TODO: better spawn location than random
        let candidate = rng.next_u32() as usize % board.cells.len();
        if board.cells[candidate].owner_civ_id == None {
          log!("Starting at {}", candidate);
          break candidate;
        }
      };

      Civilization::spawn(&mut board, vec![starting_location]);
    }
    Simulation { board }
  }
}


#[wasm_bindgen]
impl Simulation {
  #[wasm_bindgen(constructor)]
  pub fn constructor(world: &JsValue, seed: u32, initial_civs: u32) -> Simulation {
    log!("Constructor called from JS!");
    let world: World = world.into_serde().unwrap();

    let simulation_options = SimulationOptions { seed, initial_civs };

    Simulation::new(world, simulation_options)
  }

  #[wasm_bindgen(js_name = playRounds)]
  pub fn play_rounds(mut self, rounds: u32) -> Simulation {
    for _ in 0..rounds {
      for _ in 0..self.board.turn_order.len() {
        self.board.play_turn();

      }
    }
    self
  }

  #[wasm_bindgen(js_name = playTurns)]
  pub fn play_turns(mut self, turns: u32) -> Simulation {
    for _ in 0..turns {
      self.board.play_turn();
    }
    self
  }

  #[wasm_bindgen(js_name = revertRounds)]
  pub fn revert_rounds(mut self, rounds: u32) -> Simulation {
    log!("Reverting {} rounds", rounds);
    for _ in 0..rounds {
      for _ in 0..self.board.turn_order.len() {
        self.board.undo_last();
      }
    }
    self
  }

  #[wasm_bindgen(js_name = revertTurns)]
  pub fn revert_turns(mut self, turns: u32) -> Simulation {
    log!("Reverting {} turns", turns);
    for _ in 0..turns {
      self.board.undo_last();
    }
    self
  }

  pub fn as_js_value(&self) -> JsValue {
    JsValue::from_serde(&self).unwrap()
  }
}