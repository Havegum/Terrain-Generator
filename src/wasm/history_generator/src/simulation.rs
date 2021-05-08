use wasm_bindgen::prelude::*;
use wasm_bindgen::{JsValue};
use serde::{Serialize, Deserialize};
use rand_core::{RngCore, SeedableRng};
use rand_pcg::Pcg32;
use std::collections::HashMap;

use super::board::{Board, Action};
use super::civ::Civilization;

use web_sys::console;

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
  turns: u32,
}

#[wasm_bindgen(readonly)]
#[derive(Serialize)]
pub struct Simulation {
  civs: HashMap<usize, Civilization>,
  move_order: Vec<usize>, 
  board: Board,
  turn: usize,
  #[serde(skip_serializing)]
  rng: Pcg32,
}


// js:
// world = new SimulatedWorld({ seed: 1234, initialCivs?: 4 });
// world = world.simulate({ turns: 1000 })

// rust:
// create initial civs
// create new world
// create new simulation
// 
// for each turn:
  // for each civ:
    // mcts(&simulation, &civs, n: u32, k: u32) simulate `n` actions `k` times
    // select and attempt 1 action
    // reset simulation


impl Simulation {
  fn new(world: World, simulation_options: SimulationOptions) -> Simulation {
    let mut rng = Pcg32::seed_from_u64(simulation_options.seed as u64);

    let mut board = Board::new(world);

    let mut move_order = Vec::with_capacity(simulation_options.initial_civs as usize);
    let mut civs = HashMap::with_capacity(simulation_options.initial_civs as usize);
    for _ in 0..simulation_options.initial_civs {
      let starting_location = loop {
        // TODO: better spawn location than random
        let candidate = rng.next_u32() as usize % board.cells.len();
        if board.cells[candidate].owner_civ_id == None {
          log!("Starting at {}", candidate);
          break candidate;
        }
      };

      let civ = Civilization::spawn(&civs, &mut board, vec![starting_location]);
      move_order.push(civ.id);
      civs.insert(civ.id, civ);
    }

    Simulation {
      turn: 0,
      civs,
      move_order,
      board,
      rng,
    }
  }
}


#[wasm_bindgen]
impl Simulation {
  #[wasm_bindgen(constructor)]
  pub fn constructor(world: &JsValue, seed: u32, initial_civs: u32) -> Simulation {
    log!("Constructor called from JS!");
    let world: World = world.into_serde().unwrap();

    let simulation_options = SimulationOptions { seed, initial_civs, turns: 0 };

    Simulation::new(world, simulation_options)
  }

  pub fn simulate(mut self, turns: u32) -> Simulation {
    for _ in 0..turns {
      log!("__________\nPLAYING TURN {}", self.turn);
      self.board.history.push(Vec::new());
      for &id in self.move_order.iter() {
        let action = if let Some(civ) = self.civs.get_mut(&id) {
          log!("| {}'s turn", civ.name);
          Some(civ.choose_action(&mut self.board))
        } else { None };

        if let Some(action) = action {
          self.board.apply(action, id, &mut self.civs);
        }
      }
      self.turn += 1;
    }
    self
  }

  pub fn revert(mut self, turns: u32) -> Simulation {
    log!("Reverting {} turns", turns);
    let i = self.board.history.len().saturating_sub(turns as usize);

    let turns: Vec<Vec<Action>> = self.board.history
      .drain(i..)
      .collect();
    
    for actions in turns {
      self.turn -= 1;
      for action in actions.iter().rev() {
        self.board.undo(&action, &mut self.civs);
      }
    }
    
    self
  }

  pub fn as_js_value(&self) -> JsValue {
    JsValue::from_serde(&self).unwrap()
  }
}