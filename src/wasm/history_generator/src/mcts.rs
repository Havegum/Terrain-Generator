#[allow(unused_imports)]
use web_sys::console;
use std::collections::HashMap;
use rand_core::{RngCore};

use super::board::{Board, Action, Cell};
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


struct Node {
  visited: u32,
  cumulative_value: f64,
  children: HashMap<Action, Node>,
}

impl Node {
  pub fn value(&self) -> f64 {
    self.cumulative_value / self.visited as f64
  }

  pub fn ucb(&self, _n: u32) -> f64 {
    if self.visited == 0 { f64::MAX } else { 1. }
  }

  pub fn new() -> Node {
    Node {
      visited: 0,
      cumulative_value: 0.,
      children: HashMap::new()
    }
  }

  pub fn expand(&mut self, board: &Board) {
    let civ = &board.turn_order[board.turn];

    if let Some(civ) = board.civs.get(civ) {
      for action in civ.get_actions(&board.cells).drain(..) {
        self.children.insert(action, Node::new());
      }
    }
  }

  pub fn select(&mut self, board: &mut Board) -> Action {
    let mut actions = self.children.keys();
    let action = board.rng.next_u32() as usize % actions.len();
    let action = actions.nth(action).unwrap().clone();
    action
  }
}

pub struct MCTS<'a> {
  board: &'a Board,  
}

impl<'a> MCTS<'a> {
  pub fn search(board: &mut Board, civ_id: usize) -> Action {
    let Board { ref mut civs, ref cells, .. } = board;

    let civ = civs.get_mut(&civ_id).unwrap();
    let actions = civ.get_actions(cells);
    let action = board.rng.next_u32() as usize % actions.len();
    let action = actions[action].clone();
    
    action
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