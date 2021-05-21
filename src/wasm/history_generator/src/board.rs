use serde::{Serialize};
use std::collections::{HashSet, HashMap};
use std::iter::FromIterator;
use rand_core::{RngCore, SeedableRng};
use rand_pcg::Pcg32;

use super::civ::Civilization;
use super::simulation::World;
use super::mcts::MCTS;

#[allow(unused_imports)]
use web_sys::console;
#[allow(unused_imports)]
use wasm_bindgen::{JsValue};
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Action {
  Occupy(usize, Option<usize>),
  // Defend,
}

// impl Eq for Action {
//   fn eq(&self, other: &Self) -> bool {
//     match self {
//       Action::Occupy(self_cell) =>
//         if let Action::Occupy(other_cell) = other {
//           self_cell == other_cell
//         } else { false }
//     }
//   }
// }

#[derive(Debug)]
pub struct Move {
  civ_id: usize,
  action: Action,
  successful: bool,
}

#[derive(Serialize)]
pub struct Board {
  pub cells: Vec<Cell>,
  #[serde(skip_serializing)]
  pub history: Vec<Vec<Move>>,
  pub civs: HashMap<usize, Civilization>,
  pub turn_order: Vec<usize>,
  pub turn: usize,
  pub round: usize,
  #[serde(skip_serializing)]
  pub rng: Pcg32,
}

#[derive(Serialize)]
pub struct Cell {
  index: usize,
  pub owner_civ_id: Option<usize>,
  pub adjacent: HashSet<usize>,
  pub height: f64,
}

impl Board {
  pub fn new(world: World, civs: HashMap<usize, Civilization>, turn_order: Vec<usize>) -> Board {
    let cells = world.adjacencies
      .iter()
      .zip(world.heights.iter())
      .enumerate()
      .map(|(index, (adjacent, &height))| Cell {
        index,
        owner_civ_id: None,
        adjacent: HashSet::from_iter(adjacent.clone()),
        height,
      }).collect();
    
    let id = 5;
    let rng = Pcg32::seed_from_u64(id as u64);

    Board { cells, history: Vec::new(), civs, turn_order, turn: 0, round: 0, rng }
  }

  pub fn play_turn(&mut self) {
    log!("================\nStart of round {}", self.round);
    log!("_______________\nStart of turn {}", self.turn);
    if self.turn == 0 {
      self.history.push(Vec::new());
    }

    let id = self.turn_order[self.turn];
    let action = MCTS::search(self, id);
    self.turn = (self.turn + 1) % self.turn_order.len();

    if self.turn == 0 {
      self.round += 1;
    }

    self.apply(action, id);

  }

  pub fn apply(&mut self, action: Action, civ_id: usize) {
    let action = match action {
      Action::Occupy(territory, defender) =>
        self.apply_occupy(territory, civ_id, defender),
    };

    self.history.last_mut().unwrap().push(action);
  }

  pub fn undo(&mut self, r#move: &Move) {
    if r#move.successful {
      match r#move.action {
        Action::Occupy(territory, defender) =>
          self.undo_occupy(territory, defender),
      };
    }
  }

  pub fn undo_last(&mut self) {
    if let Some(turn) = self.history.last_mut() {
      match turn.pop() {
        None => {
          self.history.pop();
          self.round = self.round.saturating_sub(1);
          self.undo_last();
        },
        Some(r#move) => {
          self.undo(&r#move);
          match self.turn.checked_sub(1) {
            Some(n) => self.turn = n,
            None => self.turn = self.turn_order.len() - 1,
          }
        },
      }
    }
  }
}

impl Board {
  pub fn add_territory(&mut self, civ_id: usize, territory: usize) {
    self.cells[territory].owner_civ_id = Some(civ_id);

    let civ = self.civs.get_mut(&civ_id).unwrap();
    civ.territory.insert(territory);

    let new_neighbors: HashSet<&usize> = self.cells[territory].adjacent
      .difference(&civ.territory)
      .collect();

    civ.neighbor_territory.extend(new_neighbors);
    civ.neighbor_territory.remove(&territory);
  }

  pub fn free_territory(&mut self, territory: usize) {
    if let Some(civ_id) = self.cells[territory].owner_civ_id {
      let Board { ref mut cells, ref mut civs, .. } = *self;

      let civ = civs.get_mut(&civ_id).unwrap();
      civ.territory.remove(&territory);
      cells[territory].owner_civ_id = None;

      let mut neighbors_neighbours = cells[territory].adjacent.clone();
      neighbors_neighbours.insert(territory);

      for &n in neighbors_neighbours.iter() {
        if civ.territory.contains(&n) { continue }
        civ.neighbor_territory.remove(&n);

        let neighbours_owned_cell = cells[n].adjacent
            .iter()
            .any(|&nn| cells[nn].owner_civ_id == Some(civ_id));

        if neighbours_owned_cell {
          civ.neighbor_territory.insert(n);
        }
      }
    }
  }
}

// Occupy
impl Board {
  pub fn apply_occupy(&mut self, territory: usize, aggressor: usize, defender: Option<usize>) -> Move {
    let successful = match defender {
      None => true, // No defender => auto succeed
      Some(_) => self.rng.next_u32() > u32::MAX / 2, // Otherwise, roll dice
    };

    if successful {
      if let Some(_) = defender {
        self.free_territory(territory);
      }
      self.add_territory(aggressor, territory);
    }

    let action = Action::Occupy(territory, defender);
    Move { action, civ_id: aggressor, successful }
  }

  pub fn undo_occupy(&mut self, territory: usize, defender: Option<usize>) {
    self.free_territory(territory);

    if let Some(defender) = defender {
      self.add_territory(defender, territory);
    }
  }
}