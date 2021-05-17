use web_sys::console;
use std::collections::HashMap;

use super::board::{Board, Action};
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


trait Node {
  
}

struct State {
  visited: u32,
  value: f64,
  children: HashMap<Action, State>,
}

impl State {
  pub fn avg_value(&self) -> f64 {
    self.score / self.visited as f64
  }

  pub fn ucb(&self, N: u32) -> f64 {
    if self.visited === 0 { f64::MAX }

  }

  pub fn reveal() -> State {
    State {
      visited: 0,
      value: 0.,
      children: HashMap::new()
    }
  }

  pub fn expand(&mut self, civ: &mut Civilization) {
    // self.children
  }
}


impl Mcts {
  pub fn search() -> Action {
    let mut self = Mcts::new();
    unimplemented!()
  }

  fn new() -> Mcts {
    unimplemented!()
  }

  fn select(&mut self, expansion: Expansion) {
    // SELECT some node from expansion.children

    // match node {
    //   Leaf(l) => self.expand(l),
    //   Expansion(e) => self.select(e),
    // }
  }

  fn expand(&mut self, leaf: Leaf) {
    
  }

  fn rollout(depth: usize) {
    // pick action, uniformly random.
    // if depth > threshold || terminal state

  }

  fn backpropagate(state: &Board) {

  }
}