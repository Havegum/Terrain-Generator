#[allow(unused_imports)]
use web_sys::console;
use std::collections::HashMap;
use std::iter::FromIterator;
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


fn hm_addition(a: &mut HashMap<usize, f64>, b: HashMap<usize, f64>) -> HashMap<usize, f64> {
  for (&key, &rhs) in b.iter() {
    match a.get(&key) {
      Some(value) => a.insert(key, value + rhs),
      None => a.insert(key, rhs),
    };
  }
  a
}


struct Node {
  visited: u32,
  index: usize,
  cumulative_value: HashMap<usize, f64>,
  expanded: bool,
  children: HashMap<Action, usize>,
  parent: Option<usize>,
}

impl Node {
  const ROLLOUT_DEPTH: u32 = 8;
  const EXPLORATION_FACTOR: u32 = 2;

  pub fn value(&self) -> HashMap<usize, f64> {
    self.cumulative_value
      .clone()
      .iter()
      .map(|(&key, value)| (key, value / self.visited as f64))
      .collect()
  }

  pub fn ucb(&self, _n: u32) -> f64 {
    if self.visited == 0 { f64::MAX } else { 1. }
  }

  pub fn new(parent: Option<usize>, index: usize) -> Node {
    Node {
      parent,
      index,
      visited: 0,
      cumulative_value: HashMap::new(),
      expanded: false,
      children: HashMap::new(),
    }
  }

  pub fn is_leaf(&self) -> bool {
    self.visited == 0
  }

  pub fn expand(&mut self, board: &mut Board, mut get_index: impl FnMut() -> usize) -> Vec<Node> {
    self.expanded = true;
    let civ = board.turn_order[board.turn];
    let civ = board.civs.get(&civ).unwrap();

    civ.get_actions(&board.cells)
      .drain(..)
      .map(|action| {
        let index = get_index();
        self.children.insert(action, index);
        Node::new(Some(self.index), index)
      })
      .collect()
  }


  pub fn select(&mut self, board: &mut Board) -> usize {
    let mut actions = self.children.keys();

    // SELECT RANDOM ACTION FOR NOW
    let action = board.rng.next_u32() as usize % actions.len();
    let action = actions.nth(action).unwrap().clone();

    *self.children.get_mut(&action).unwrap()
  }


  pub fn rollout(&mut self, board: &mut Board, depth: u32) -> HashMap<usize, f64> {
    if depth > Node::ROLLOUT_DEPTH {
      return HashMap::new();
    }

    // unimplementend!()

    // simulate with random actions forever, until depth `n`, then return value.

    self.rollout(board, depth + 1)
  }

  fn backpropagate(&mut self) {
    unimplemented!()
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
    
    let mut i = 0;
    let mut get_index = || { i += 1; i - 1 };

    let root = get_index();
    let mut nodes = vec![Node::new(None, root)];
    let children = nodes[root].expand(board, get_index);
    nodes.extend(children);

    let mut current = 0;
    
    // LOOP
    {
      // SELECT
      while !nodes[current].expanded {
        current = nodes[current].select(board);
      }
      
      // EXPAND
      if nodes[current].visited > 0 {
        let children = nodes[current].expand(board, get_index);
        nodes.extend(children);
        current = nodes[current].select(board);
      }

      // ROLLOUT
      let value = nodes[current].rollout(board, 0);

      // BACKPROPAGATE
      while let Some(parent) = nodes[current].parent {
        hm_addition(nodes[current].cumulative_value, value);
        nodes[current].visited += 1;
        current = parent;
      }
    }
    

    action
  }


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