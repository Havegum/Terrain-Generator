use serde::{Serialize};

pub enum ActionType {
  Occupy(usize),
  Defend,
}

pub struct Action {
  civ_id: u32,
  action: ActionType,
  successful: bool,
}

#[derive(Serialize)]
pub struct Board {
  cells: Vec<Cell>,
}

#[derive(Serialize)]
pub struct Cell {
  index: usize,
  owner_civ_id: Option<usize>,
  adjacent: Vec<usize>,
}

impl Board {
  pub fn new(adjacencies: &Vec<Vec<usize>>) -> Board {
    let cells = adjacencies
      .iter()
      .enumerate()
      .map(|(index, adjacent)| Cell {
        index,
        owner_civ_id: None,
        adjacent: adjacent.clone(),
      }).collect();

    Board { cells }
  }
}