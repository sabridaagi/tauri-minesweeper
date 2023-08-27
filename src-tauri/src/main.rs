// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::MutexGuard;

mod board;
use board::*;

// Game state for the tauri app
#[derive(Default)]
struct AppState {
    game: Mutex<Game>
}

#[derive(Serialize, Deserialize, Default)] // JSON
pub struct Game {
    pub status: u8, // 0: menu, 1: in-game, 2: paused
    pub board: Board,
    pub opened_cells: Vec<u16> // indexes of opened cells
}

// Generate the board and return it as a JSON string
#[tauri::command]
fn generate_board(
    app_state: tauri::State<Arc<AppState>>,
    width: usize, 
    height: usize, 
    number_bombs: usize) -> String {

    // Getting app state and creating the board
    let mut game_state: MutexGuard<Game> = app_state.game.lock().unwrap();
    game_state.opened_cells.clear();
    
    let mut board = Board::new(width, height);

    // Generate bombs map
    board.genetare_bombs(number_bombs);

    // Save it in the game state
    game_state.board = board;

    game_state.board.generate_response(&game_state.opened_cells)
}

// Cell clicked
#[tauri::command]
fn cell_clicked(app_state: tauri::State<Arc<AppState>>, x: usize, y: usize) -> String { 
    let mut game_state: MutexGuard<Game> = app_state.game.lock().unwrap();
    
    // calculate the new index
    let new_value: u16 = (y * game_state.board.width + x) as u16;

    if !game_state.opened_cells.contains(&new_value) {
        game_state.opened_cells.push(new_value);
    }

    game_state.board.generate_response(&game_state.opened_cells)
}

// Update the game status
#[tauri::command]
fn update_game_state(app_state: tauri::State<Arc<AppState>>, new_status: u8) -> u8 {
    let mut game: MutexGuard<Game> = app_state.game.lock().unwrap();
    game.status = new_status;
    return game.status;
}

fn main() {
    tauri::Builder::default()
        .manage(Arc::new(AppState::default()))
        .invoke_handler(tauri::generate_handler![
            generate_board,
            cell_clicked,
            update_game_state,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
