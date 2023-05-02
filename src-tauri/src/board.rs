use serde::{Deserialize, Serialize};

/*
 * For cells:
 *  0 is an empty cell
 *  1-8 Represent the number of bombs around
 *  9 is a bomb cell
 *  10 is unopenned
 *  11 is flagged
 */
pub type Cell = u8;

/*
 * A board has a width, height and a Vector of cells.
 * This is the main interaction with the front-end
 * It sends the Board with the updated cells values
 */
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

/*
* Generate the board at the beggining of the game
*/
pub fn create_board(width: usize, height: usize) -> Board {
    let cells = vec![0; width * height ]; // Vector
    return Board { width, height, cells }
}

/*
* Hides unopened cells
*/
pub fn hide_unopened_cells(board: &Board, opened_cells: &Vec<u16>) -> Board {
    // reveal all cells and return the new board
    let mut new_board = board.clone();

    // hide all cells
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