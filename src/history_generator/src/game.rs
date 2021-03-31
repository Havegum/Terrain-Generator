struct Game {
    board: Board,
    civilizations: Vec<Civilization>,
    simulation: Simulation,
}

impl Game {
    pub fn new(cells: Vec<BoardCell>, adjacencies: Vec<Vec<usize>>) -> Game {
        let board = Board::new(cells, adjacencies);

        let simulation = Simulation {
            board: board.clone(),
            civilizations: Vec::new(),
        };


        Game {
            board,
            simulation,
            civilizations: Vec::new(),
        }
    }
}
