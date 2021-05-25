#[allow(unused_imports)]
use web_sys::console;
use std::collections::HashMap;
// use std::iter::FromIterator;
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

enum Node {
  Action(ActionNode),
  Result(ResultNode),
}

struct ActionNode {
  visited: u32,
  index: usize,
  cumulative_value: HashMap<usize, f64>,
  action: Action,
  true_child: Option<usize>,
  false_child: Option<usize>,
  parent: Option<usize>,
}

impl ActionNode {
  pub fn new(parent: Option<usize>, index: usize, action: Action) -> Self {
    Self {
      parent,
      index,
      action,
      visited: 0,
      cumulative_value: HashMap::new(),
      true_child: None,
      false_child: None
    }
  }

  pub fn expand(&mut self, mut i: usize) -> Vec<Node> {
    self.true_child = Some(i + 1);
    self.false_child = Some(i + 2); 

    vec![
      Node::Result(ResultNode::new(Some(self.index), self.true_child.unwrap())),
      Node::Result(ResultNode::new(Some(self.index), self.false_child.unwrap())),
    ]
  }

  pub fn select(&self, board: &mut Board) -> usize {
    let turn = board.turn_order[board.turn];

    if board.check(&self.action, turn) {
      self.true_child.unwrap()
    } else {
      self.false_child.unwrap()
    }
  }
}

struct ResultNode {
  visited: u32,
  index: usize,
  cumulative_value: HashMap<usize, f64>,
  expanded: bool,
  children: HashMap<Action, usize>,
  parent: Option<usize>,
}

impl ResultNode {
  const ROLLOUT_DEPTH: u32 = 8;

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

  pub fn expand(&mut self, board: &mut Board, mut i: usize) -> Vec<Node> {
    self.expanded = true;
    let civ = board.turn_order[board.turn];
    let civ = board.civs.get(&civ).unwrap();

    civ.get_actions(&board.cells)
      .drain(..)
      .map(|action| {
        let index = i;
        i += 1;
        self.children.insert(action, index);
        Node::Action(
          ActionNode::new(Some(self.index), index, action.clone())
        )
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

    // simulate with random actions forever, until depth `n`, then return value.
    // unimplementend!()

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
    let mut nodes: Vec<Node> = vec![
      Node::Result(ResultNode::new(None, root))
    ];

    if let Node::Result(n) = nodes[root] {
      let children = n.expand(board, idx);
      idx += children.len();
      nodes.extend(children);
    }

    let mut current = 0;

    // LOOP
    for _ in 0..ITERATIONS {

      // SELECT
      loop {
        if let Node::Result(n) = nodes[current] {
          current = n.select(board, |child, civ|
            Self::ucb_from_nodes(&nodes[child], civ, &Node::Result(n))
          );
        }

        if let Node::Action(n) = nodes[current] {
          current = n.select(board);
        }

        // stop SELECT if
        // result node && leaf node (leaf nodes aren't expanded)
        if let Node::Result(n) = nodes[current] {
          if !n.expanded { break }
        }
      }
      
      // EXPAND
      if let Node::Result(n) = nodes[current] {
        // if we've been here before: EXPAND
        if n.visited > 0 {
          let children = n.expand(board, idx);
          idx += children.len();

          let action_children = children.iter_mut()
            .flat_map(|n| {
              if let Node::Action(n) = n {
                let c = n.expand(idx);
                idx += c.len();
                c
              } else { Vec::new() }
            })
            .collect::<Vec<Node>>();
          nodes.extend(children);
          nodes.extend(action_children);

          current = *n.children.values().nth(0).unwrap();
          if let Node::Action(n) = nodes[current] {
            current = n.select(board);
          }
        }
      }

      let value = match nodes[current] {
        Node::Result(n) => Some(n.rollout(board, 0)),
        _ => None,
      };
      let value = value.unwrap();

      
      // BACKPROPAGATE
      while let Some(parent) = match nodes[current] {
        Node::Action(n) => n.parent,
        Node::Result(n) => n.parent,
      } {
        match nodes[current] {
          Node::Action(n) => {
            hm_addition(&mut n.cumulative_value, &value);
            n.visited += 1;
            current = parent;
          },
          Node::Result(n) => {
            hm_addition(&mut n.cumulative_value, &value);
            n.visited += 1;
            current = parent;
          },
        }
      }
    }

    let n = match nodes[0] {
      Node::Result(n) => Some(n),
      _ => None,
    };
    let n = n.unwrap();

    let (action, ..) = n.children.iter()
      .max_by(|(k1, &v1), (k2, &v2)| {
        match (nodes[v1], nodes[v2]) {
          (Node::Action(a), Node::Action(b)) => a.visited.cmp(&b.visited),
          _ => Less,
        }
      })
      .unwrap();

    action.clone()
  }

  fn ucb_from_nodes(node: &Node, civ: usize, parent: &Node) -> f64 {
    let visited = match node {
      Node::Action(n) => n.visited,
      Node::Result(n) => n.visited,
    };

    if visited == 0 {
      return f64::INFINITY;
    }

    let cumulative_value = match node {
      Node::Action(n) => n.cumulative_value.get(&civ).unwrap_or(&0.0),
      Node::Result(n) => n.cumulative_value.get(&civ).unwrap_or(&0.0),
    };

    let parent_visited = match parent {
      Node::Action(n) => n.visited,
      Node::Result(n) => n.visited,
    };

    let avg_val = *cumulative_value / visited as f64;

    Self::ucb(avg_val, visited, parent_visited)
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