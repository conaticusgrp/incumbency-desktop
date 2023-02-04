#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod entities;
mod game;
mod common;

use game::{manager::{create_game}, structs::GameState};
use common::filesystem::check_save_exists;
use std::sync::{Arc, Mutex};
use game::events::{app_open, app_close, enable_rule, disable_rule, update_rule, update_tax_rate, update_business_tax_rate, update_healthcare_budget, update_welfare_budget, update_business_budget, update_childcare_capacity, update_adultcare_capacity, update_eldercare_capacity};

#[tokio::main]
async fn main() { 
  tauri::Builder::default()
  .invoke_handler(tauri::generate_handler![create_game, check_save_exists, app_close, app_open, enable_rule, disable_rule, update_rule, update_tax_rate, update_business_tax_rate, update_healthcare_budget, update_welfare_budget, update_business_budget, update_childcare_capacity, update_adultcare_capacity, update_eldercare_capacity])
    .manage(Arc::new(Mutex::new(GameState::default())))
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}