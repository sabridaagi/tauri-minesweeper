use serde::{Deserialize, Serialize};
use rand::Rng;

/// * 0 is an empty cell
/// * 1-8 Represent the number of bombs around
/// * 9 is a bomb cell
/// * 10 is unopened
pub type Cell = u16;

/// static state of a board, does not change after init
#[derive(Serialize, Deserialize, Default)]
pub struct Board {
    pub width: usize,
    pub height: usize,
    pub cells: Vec<Cell>
}

impl Clone for Board {
    fn clone(&self) -> Board {
        Board {
            width: self.width,
            height: self.height,
            cells: self.cells.clone(),
        }
    }
}

impl Board {
    /// Generate the board
    pub fn new(width: usize, height: usize) -> Board {
        Board { width, height, cells: vec![0; width * height] }
    }

    /// Generates the bombs depending on the number of bombs
    pub fn generate_bombs(&self, number_bombs: usize) {
        // TODO
    }

    /// Generates a new board but with hidden cells
    pub fn hide_unopened_cells(&self, opened_cells: &Vec<u16>) -> Board {
        let mut hidden_board = self.clone();

        // TODO

        hidden_board
    }
}

/*
* Hides unopened cells
*/
pub fn hide_unopened_cells(board: &Board, opened_cells: &Vec<u16>) -> Board {
    let mut new_board = board.clone();

    //hide all cells
    for cell in new_board.cells.iter_mut() {
        *cell = 10;
    }

    for &opened_cell_position in opened_cells {
        if let Some(cell) = new_board.cells.get_mut(opened_cell_position as usize) {
            *cell = board.cells[opened_cell_position as usize];
        }
    }

    return new_board;
}
/*
* Generate bombs 
*/
pub fn genetare_bombs_map(board: &Board, number_bombs: u16) -> Board {
    let mut new_board = board.clone();
    new_board.cells.clear();

    // Generate random bomb positions
    let mut rng = rand::thread_rng();
    let mut bomb_positions = Vec::new();

    // Noise
    while bomb_positions.len() < number_bombs as usize {
        let position = rng.gen_range(0..(board.width * board.height));
        if !bomb_positions.contains(&position) {
            bomb_positions.push(position);
        }
    }

    for i in 0..new_board.width * new_board.height {
        let cell = if bomb_positions.contains(&i) {
            9 // Bomb cell has value 9
        } else {
            0 // Empty
        };
        new_board.cells.push(cell);
    }

    for i in 0..new_board.width * new_board.height {
        if new_board.cells[i] == 0 {
            new_board.cells[i] = count_neighbor_bombs(&new_board, i);
        }
    }

    return new_board;
}

/*
* function generating the number of bombs around
*/
fn count_neighbor_bombs(board: &Board, position: usize) -> u16 {
    // TODO
    3
}