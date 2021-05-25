#[allow(unused_imports)]
use web_sys::console;
use std::collections::HashMap;
use std::iter::FromIterator;
use std::cmp::Ordering::Less;
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


fn hm_addition(a: &mut HashMap<usize, f64>, b: &HashMap<usize, f64>) {
  for (&key, &rhs) in b.iter() {
    let val = match a.get(&key) {
      Some(&value) => value,
      None => 0.,
    };
    a.insert(key, val + rhs);
  }
}

// enum Node {
//   Action(ActionNode),
//   Result(ResultNode),
// }

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
  // const EXPLORATION_FACTOR: u32 = 2;

  pub fn value(&self) -> HashMap<usize, f64> {
    self.cumulative_value
      .clone()
      .iter()
      .map(|(&key, value)| (key, value / self.visited as f64))
      .collect()
  }


  fn ucb(avg_val: f64, visits: u32, parent_visits: u32) -> f64 {
    const EXPLORATION: f64 = 2.0;
    let visits = visits as f64;
    let parent_visits = parent_visits as f64;

    avg_val + EXPLORATION * (parent_visits.ln() / visits).sqrt()
  }

  pub fn new(parent: Option<usize>, index: usize) -> Self {
    Self {
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

  pub fn expand(&mut self, board: &mut Board, mut i: usize) -> Vec<Self> {
    self.expanded = true;
    let civ = board.turn_order[board.turn];
    let civ = board.civs.get(&civ).unwrap();

    civ.get_actions(&board.cells)
      .drain(..)
      .map(|action| {
        let index = i;
        i += 1;
        self.children.insert(action, index);
        Self::new(Some(self.index), index)
      })
      .collect()
  }


  pub fn select(&self, board: &mut Board, ucb: impl Fn(usize, usize) -> f64) -> usize {
    let turn = board.turn_order[board.turn];

    let action = self.children.keys()
      .max_by(|k1, k2| {
        ucb(*self.children.get(k1).unwrap(), turn)
        .partial_cmp(
          &ucb(*self.children.get(k2).unwrap(), turn)
        )
        .unwrap_or(Less)
      });

    let action = action.unwrap();

    *self.children.get(&action).unwrap()
  }


  pub fn rollout(&mut self, board: &Board, depth: u32) -> HashMap<usize, f64> {
    if depth > Self::ROLLOUT_DEPTH {
      return board.civs.iter()
        .map(|(&id, civ)| (id, civ.score()))
        .collect();
    }

    // unimplementend!()

    // simulate with random actions forever, until depth `n`, then return value.

    self.rollout(board, depth + 1)
  }
}

pub struct MCTS;

impl MCTS {
  pub fn search(board: &mut Board, civ_id: usize) -> Action {
    const ITERATIONS: u32 = 10;

    let Board { ref mut civs, ref cells, .. } = board;

    let civ = civs.get_mut(&civ_id).unwrap();
    let actions = civ.get_actions(cells);
    let action = board.rng.next_u32() as usize % actions.len();
    let action = actions[action].clone();
    
    let mut idx = 0;

    let root = idx;
    let mut nodes = vec![Node::new(None, root)];
    let children = nodes[root].expand(board, idx);
    idx += children.len();
    nodes.extend(children);

    let mut current = 0;

    // LOOP
    for _ in 0..ITERATIONS {

      // SELECT
      while !nodes[current].expanded {
        // match nodes[current].kind {
        // 
        // }
        let current_visits = nodes[current].visited;
        current = nodes[current].select(board, |child, civ|
          Self::ucb_from_nodes(&nodes[child], civ, &nodes[current])
        );
      }
      
      // EXPAND
      if nodes[current].is_leaf() {
        let children = nodes[current].expand(board, idx);
        idx += children.len();
        // let stochastic_nodes = Nodes::expand_stochastic(children, idx);
        nodes.extend(children);

        // current = nodes[current].select(board);
      }

      // ROLLOUT
      let value = nodes[current].rollout(board, 0);

      // BACKPROPAGATE
      while let Some(parent) = nodes[current].parent {
        hm_addition(&mut nodes[current].cumulative_value, &value);
        nodes[current].visited += 1;
        current = parent;
      }
    }
    
    let (action, ..) = nodes[0].children.iter()
      .max_by(|(k1, &v1), (k2, &v2)| nodes[v1].visited.cmp(&nodes[v2].visited))
      .unwrap();

    action.clone()
  }

  fn ucb_from_nodes(node: &Node, civ_turn: usize, parent: &Node) -> f64 {
    if node.visited == 0 {
      return f64::INFINITY;
    }

    Self::ucb(
      *node.cumulative_value.get(&civ_turn).unwrap_or(&0.0) / node.visited as f64,
      node.visited,
      parent.visited,
    )
  }

  fn ucb(avg_val: f64, visits: u32, parent_visits: u32) -> f64 {
    const EXPLORATION: f64 = 2.0;
    let visits = visits as f64;
    let parent_visits = parent_visits as f64;

    avg_val + EXPLORATION * (parent_visits.ln() / visits).sqrt()
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