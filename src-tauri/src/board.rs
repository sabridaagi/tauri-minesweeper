use serde::{Deserialize, Serialize};
use rand::Rng;

/*
 * A board has a width, height and a Vector of cells.
 * This is the main interaction with the front-end
 * It sends the Board with the updated cells values
 */
#[derive(Serialize, Deserialize, Default)]
pub struct Board {
    pub width: usize,
    pub height: usize,

    /*
    *  0    : empty
    *  1-8  : number of bombs around
    *  9    : bomb
    *  10   : hidden
    */
    pub cells: Vec<u8>
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
    pub fn new(width: usize, height: usize) -> Board {
        let cells = vec![0; width * height ];
        Board { width, height, cells }
    }
    
    pub fn hide_cells(&self, opened_cells: &Vec<u16>) -> Board {
        let mut cloned_board = self.clone();

        for (index, cell) in cloned_board.cells.iter_mut().enumerate() {
            if !opened_cells.contains(&(index as u16)) {
                *cell = 10;
            }
        }
    
        cloned_board
    }

    pub fn generate_response(&self, opened_cells: &Vec<u16>) -> String {
        serde_json::to_string(&self.hide_cells(&opened_cells)).expect("Failed to serialize board")
    }

    pub fn genetare_bombs(&self, number_bombs: &usize) {
        let mut new_board = self.clone();
        new_board.cells.clear();
    
        // Generate random bomb positions
        let mut rng = rand::thread_rng();
        let mut bomb_positions = Vec::new();
    
        // Noise
        while bomb_positions.len() < *number_bombs {
            let position = rng.gen_range(0..(self.width * self.height));
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
    }
}

/*
* function generating the number of bombs around
*/
fn count_neighbor_bombs(board: &Board, position: usize) -> u8 {
    return 1
}