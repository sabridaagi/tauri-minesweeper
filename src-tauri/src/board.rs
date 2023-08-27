use rand::seq::SliceRandom;
use rand::thread_rng;
use serde::{Deserialize, Serialize};

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
    pub cells: Vec<u8>,
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
        let cells = vec![0; width * height];
        Board {
            width,
            height,
            cells,
        }
    }

    pub fn hide_cells(&self, opened_cells: &Vec<u16>) -> Board {
        // cloning to return a new board
        let mut cloned_board = self.clone();

        for (index, cell) in cloned_board.cells.iter_mut().enumerate() {
            if !opened_cells.contains(&(index as u16)) {
                *cell = 10;
            }
        }

        cloned_board
    }

    pub fn genetare_bombs(&mut self, number_bombs: usize) {
        let mut rng = thread_rng();

        let mut indices: Vec<usize> = (0..self.cells.len()).collect();
        indices.shuffle(&mut rng);

        for index in indices.iter().take(number_bombs) {
            self.cells[*index] = 9
        }

        self.update_neighbors_count();
    }

    pub fn generate_response(&self, opened_cells: &Vec<u16>) -> String {
        serde_json::to_string(&self.hide_cells(&opened_cells)).expect("Failed to serialize board")
    }

    fn update_neighbors_count(&mut self) {
        // TODO
    }
}
