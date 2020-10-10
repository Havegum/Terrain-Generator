use super::civ::Civilization;

pub struct BoardCell<'a> {
    // adjacent: Vec<BoardCell> // Maybe this is a function?
    index: usize,
    owner: Option<&'a Civilization>,
    // resources: f64,
}

pub struct Board<'a> {
    cells: Vec<BoardCell<'a>>,
    adjacencies: Vec<Vec<usize>>,
    history: Vec<BoardMutation<'a>>,
}

pub enum BoardMutation<'a> {
    Ownership { cell: usize, prev: Option<&'a Civilization>, next: Option<&'a Civilization> },
    None,
}

impl<'a> Board<'a> {
    pub fn new(cells: Vec<BoardCell>, adjacencies: Vec<Vec<usize>>) -> Board {
        Board { cells, adjacencies, history: Vec::new() }
    }

    pub fn r#do(&mut self, action: SimulatedAction<'a >) {
        if !action.successful { return; }

        if let ActionType::Occupy(cell) = action.action {
            self.cells[cell].owner = Some(action.civ);
            self.history.push(
                BoardMutation::Ownership {
                    cell: cell,
                    prev: self.cells[cell].owner,
                    next: Some(action.civ),
                }
            );
        }
    }

    pub fn undo(&mut self, action: &SimulatedAction) {
        if !action.successful { return; }

        if let ActionType::Occupy(occupiedCell) = action.action {
            let last = self.history.last().unwrap();

            if let BoardMutation::Ownership { cell, prev, next } = last {
                if next.unwrap() == action.civ && *cell == occupiedCell {
                    self.cells[*cell].owner = *prev;
                    self.history.pop();
                }
            }
        }
    }

    // pub fn adjacent(self, cell: usize) -> &Vec<BoardCell> {
    //
    // }
}

pub enum ActionType {
    Occupy(usize),
    Grow,
    Defend,
    // Found (city)
    // Others?
}

pub struct SimulatedAction<'a> {
    civ: &'a Civilization,
    action: ActionType,
    successful: bool,
}
