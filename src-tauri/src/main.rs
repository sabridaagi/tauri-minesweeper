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
    game_state: Mutex<Game>
}

#[derive(Serialize, Deserialize, Default)] // JSON
pub struct Game {
    pub status: u8, // 0: menu, 1: in-game, 2: paused
    pub board: Board,
    pub opened_cells: Vec<u16>
}

// Generate the board and return it as a JSON string
#[tauri::command]
fn generate_board(
    app_state: tauri::State<Arc<AppState>>,
    width: usize, 
    height: usize, 
    number_bombs: u16) -> String {

    // Getting app state and creating the board
    let mut game_state: MutexGuard<Game> = app_state.game_state.lock().unwrap();
    let mut board = Board::new(width, height);

    game_state.opened_cells.clear(); // Reset
    board = genetare_bombs_map(&board, number_bombs); // Generate bombs map

    // Save it in the game state
    game_state.board = board;

    // Board sent to the front-end
    // let hidden_board: Board = board.hide_unopened_cells(&game_state.opened_cells);
    return serde_json::to_string(&game_state.board).expect("Failed to serialize board");
}

// Cell clicked
#[tauri::command]
fn cell_clicked(app_state: tauri::State<Arc<AppState>>, x: usize, y: usize) -> String {
    println!("x: {}, y: {}", x , y);
    // see if this cell is already revealed
    // if revelad do nothing
    // if not add it to reveal it

    let mut game: MutexGuard<Game> = app_state.game_state.lock().unwrap();
    let new_value: u16 = (x * game.board.width + y) as u16;

    if !game.opened_cells.contains(&new_value) {
        game.opened_cells.push(new_value);
    }

    // Board sent to the front-end
    let hidden_board: Board = hide_unopened_cells(&game.board, &game.opened_cells);

    // we return the user_board
    return serde_json::to_string(&hidden_board).expect("Failed to serialize board");
}

// Cell right-clicked
fn cell_right_clicked(board: Board) {
    //TODO
    // we return the user_board
}

// Update the game status
#[tauri::command]
fn update_game_state(app_state: tauri::State<Arc<AppState>>, new_status: u8) -> u8 {
    let mut game_state: MutexGuard<Game> = app_state.game_state.lock().unwrap();
    game_state.status = new_status;
    return game_state.status;
}

fn main() {
    tauri::Builder::default()
        .manage(Arc::new(AppState::default()))
        .invoke_handler(tauri::generate_handler![
            generate_board,
            update_game_state,
            cell_clicked
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
