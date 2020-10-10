use std::collections::HashMap;

pub struct Tree {
    pub children: HashMap<SimulatedAction, Tree>,
    pub actions: Vec<ActionType>,
    pub visits: u64,
    pub score: f64,
    pub board: Option<Board>,
    // pub entry_action: SimulatedAction,
}


impl Tree {
    pub fn new () -> Tree {
        let children = HashMap::new(); // Initialize empty
        let actions = vec![]; // Get actions
        let visits = 1;
        let score = 0.0;

        Tree { children, actions, visits, score }
    }
}
