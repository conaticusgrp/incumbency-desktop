#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod entities;
mod game;
mod common;

use game::{manager::{create_game, set_tax}, state_manager::GameState};
use common::filesystem::check_save_exists;
use std::sync::{Arc, Mutex};

#[tokio::main]
async fn main() { 
  tauri::Builder::default()
  .invoke_handler(tauri::generate_handler![create_game, check_save_exists, set_tax])
    .manage(Arc::new(Mutex::new(GameState::default())))
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}