use wasm_bindgen::prelude::*;
use wasm_bindgen::{JsValue};
use serde::{Serialize};
use rand_core::{RngCore, SeedableRng};

use super::board::{Board};
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

#[wasm_bindgen]
#[derive(Serialize)]
pub struct SimulationOptions {
  seed: u32,
  initial_civs: u32,
  turns: u32,
}

#[wasm_bindgen(readonly)]
#[derive(Serialize)]
pub struct Simulation {
  civs: Vec<Civilization>,
  truth: Board,
  simulation: Board,
  turn: usize,
  simulation_options: SimulationOptions,
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
  
  pub fn new(adjacencies: Vec<Vec<usize>>, simulation_options: SimulationOptions) -> Simulation {
    let mut truth = Board::new(&adjacencies);
    let mut simulation = Board::new(&adjacencies);

    let mut civs: Vec<Civilization> = Vec::with_capacity(simulation_options.initial_civs as usize);
    for _ in 0..simulation_options.initial_civs {
      civs.push(Civilization::spawn(&civs, &mut truth, &mut simulation));
    }

    Simulation {
      turn: 0,
      civs,
      truth,
      simulation,
      simulation_options,
    }
  }
}


#[wasm_bindgen]
impl Simulation {
  #[wasm_bindgen(constructor)]
  pub fn js_new(adjacencies: &JsValue, seed: u32, initial_civs: u32) -> Simulation {
    log!("Constructor called from JS!");
    let adjacencies: Vec<Vec<usize>> = adjacencies.into_serde().unwrap();
    let simulation_options = SimulationOptions { seed, initial_civs, turns: 0 };

    Simulation::new(adjacencies, simulation_options)
  }

  pub fn simulate(mut self, turns: u32) -> Simulation {
    log!("Simulate {} turns", turns);
    for turn in 0..turns {
      log!("# {}", turn);
      for mut civ in self.civs.iter_mut() {
        let action = civ.choose_action(&mut self.simulation);
        self.truth.apply(action, civ);
        log!("  | {}'s turn", civ.name);
      }
    }
    self
  }

  pub fn as_js_value(&self) -> JsValue {
    JsValue::from_serde(&self).unwrap()
  }
}