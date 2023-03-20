#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod common;
mod entities;
mod game;

use common::filesystem::check_save_exists;
use game::events::{
    app_close, app_open, disable_rule, enable_rule, update_adultcare_capacity,
    update_business_budget, update_business_tax_rate, update_childcare_capacity,
    update_eldercare_capacity, update_healthcare_budget, update_rule, update_tax_rate,
    update_welfare_budget,
};
use game::{manager::create_game, structs::GameState};
use std::sync::{Arc, Mutex};

#[tokio::main]
async fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            create_game,
            check_save_exists,
            app_close,
            app_open,
            enable_rule,
            disable_rule,
            update_rule,
            update_tax_rate,
            update_business_tax_rate,
            update_healthcare_budget,
            update_welfare_budget,
            update_business_budget,
            update_childcare_capacity,
            update_adultcare_capacity,
            update_eldercare_capacity
        ])
        .manage(Arc::new(Mutex::new(GameState::default())))
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
