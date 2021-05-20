#[allow(unused_imports)]
use web_sys::console;
use std::iter::FromIterator;
// use std::collections::HashMap;
use rand_core::{RngCore};

use super::board::{Board, Action};
// use super::civ::Civilization;

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


// struct State {
//   visited: u32,
//   value: f64,
//   children: HashMap<Action, State>,
// }

// impl State {
  // pub fn avg_value(&self) -> f64 {
    // self.score / self.visited as f64
  // }

  // pub fn ucb(&self, N: u32) -> f64 {
  //   if self.visited == 0 { f64::MAX } else { 1. }
  // }

  // pub fn reveal() -> State {
  //   State {
  //     visited: 0,
  //     value: 0.,
  //     children: HashMap::new()
  //   }
  // }

  // pub fn expand(&mut self, civ: &mut Civilization) {
    // self.children
  // }
// }

pub struct MCTS<'a> {
  board: &'a Board,  
}

impl<'a> MCTS<'a> {
  pub fn search(board: &mut Board, civ_id: usize) -> Action {
    let civ = board.civs.get_mut(&civ_id).unwrap();
    
    let candidates = Vec::from_iter(civ.neighbor_territory.clone());
    let territory = civ.rng.next_u32() as usize % candidates.len();
    let territory = candidates[territory];
    let defender = board.cells[territory].owner_civ_id;
    Action::Occupy(territory, defender)

    // unimplemented!()
  }

  // pub fn new(board: &mut Board) -> MCTS {
  //   MCTS {
  //     board
  //   }
  // }

  // fn select(&mut self, expansion: Expansion) {
    // SELECT some node from expansion.children

    // match node {
    //   Leaf(l) => self.expand(l),
    //   Expansion(e) => self.select(e),
    // }
  // }

  // fn expand(&mut self, leaf: Leaf) {
    
  // }

  // fn rollout(depth: usize) {
    // pick action, uniformly random.
    // if depth > threshold || terminal state

  // }

  // fn backpropagate(state: &Board) {

  // }
}