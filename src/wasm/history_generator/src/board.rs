use serde::{Serialize};
use std::collections::{HashSet, HashMap};
use std::iter::FromIterator;
use rand_core::{RngCore};

use super::civ::Civilization;

use web_sys::console;
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

#[derive(Debug)]
pub enum ActionType {
  Occupy(usize, Option<usize>),
  // Defend,
}

#[derive(Debug)]
pub struct Action {
  civ_id: usize,
  action: ActionType,
  successful: bool,
}

#[derive(Serialize)]
pub struct Board {
  pub cells: Vec<Cell>,
  #[serde(skip_serializing)]
  pub history: Vec<Vec<Action>>,
}

#[derive(Serialize)]
pub struct Cell {
  index: usize,
  pub owner_civ_id: Option<usize>,
  pub adjacent: HashSet<usize>,
}

impl Board {
  pub fn new(adjacencies: &Vec<Vec<usize>>) -> Board {
    let cells = adjacencies
      .iter()
      .enumerate()
      .map(|(index, adjacent)| Cell {
        index,
        owner_civ_id: None,
        adjacent: HashSet::from_iter(adjacent.clone()),
      }).collect();

    Board { cells, history: Vec::new() }
  }

  pub fn apply(&mut self, action: ActionType, civ_id: usize, civs: &mut HashMap<usize, Civilization>) {
    let action = match action {
      ActionType::Occupy(territory, defender) =>
        self.apply_occupy(territory, civ_id, defender, civs),
    };

    self.history.last_mut().unwrap().push(action);
  }
  
  pub fn undo(&mut self, action: &Action, civs: &mut HashMap<usize, Civilization>) {
    if action.successful {
      match action.action {
        ActionType::Occupy(territory, defender) =>
          self.undo_occupy(territory, action.civ_id, defender, civs),
      };
    }
  }
}

// Occupy
impl Board {
  pub fn apply_occupy(&mut self, territory: usize, aggressor: usize, defender: Option<usize>, civs: &mut HashMap<usize, Civilization>) -> Action {
    match defender {
      
      Some(defender) => {
        let successful = civs.get_mut(&aggressor).unwrap().rng.next_u32() > u32::MAX / 2;

        if successful {
          civs.get_mut(&defender).unwrap().remove_territory(self, territory);
          civs.get_mut(&aggressor).unwrap().add_territory(self, territory);
        }
        let action = ActionType::Occupy(territory, Some(defender));
        Action { action, civ_id: aggressor, successful }
      },
      None => {
        civs.get_mut(&aggressor).unwrap().add_territory(self, territory);
        let action = ActionType::Occupy(territory, None);
        Action { action, civ_id: aggressor, successful: true }
      }
    }
  }

  pub fn undo_occupy(&mut self, territory: usize, aggressor: usize, defender: Option<usize>, civs: &mut HashMap<usize, Civilization>) {
    civs.get_mut(&aggressor).unwrap().remove_territory(self, territory);

    if let Some(defender) = defender {
      civs.get_mut(&defender).unwrap().add_territory(self, territory);
    }
  }
}