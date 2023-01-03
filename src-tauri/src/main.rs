#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod entities;
mod generation;
mod new_game;
mod util;
mod config;
mod game;

use game::GameState;
use new_game::*;
use std::sync::{Arc, Mutex};

#[tokio::main]
async fn main() { 
  tauri::Builder::default()
  .invoke_handler(tauri::generate_handler![create_game, check_save_exists])
    .manage(Arc::new(Mutex::new(GameState::default())))
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
