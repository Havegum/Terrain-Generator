use serde::{Serialize};
use std::collections::{HashSet, HashMap};
use std::iter::FromIterator;
use rand_core::{RngCore};

use super::civ::Civilization;

pub enum ActionType {
  Occupy(usize),
  // Defend,
}

pub struct Action {
  civ_id: usize,
  action: ActionType,
  successful: bool,
}

#[derive(Serialize)]
pub struct Board {
  pub cells: Vec<Cell>,
  #[serde(skip_serializing)]
  pub history: Vec<Action>,
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

  pub fn apply (&mut self, action: ActionType, civ_id: usize, civs: &mut HashMap<usize, Civilization>) -> Action {
    let successful: bool;
    match action {
      ActionType::Occupy(i) => {
        match self.cells[i].owner_civ_id {
          Some(other) => {
            let successful = civs.get_mut(&civ_id).unwrap().rng.next_u32() > u32::MAX / 2;

            if successful {
              civs.get_mut(&civ_id).unwrap().add_territory(self, i);
              civs.get_mut(&other).unwrap().remove_territory(self, i);
            }
            return Action { action, civ_id, successful };
          }
          None => {
            civs.get_mut(&civ_id).unwrap().add_territory(self, i);
            return Action { action, civ_id, successful: true };
          }
        }
      }
    };
  }
}