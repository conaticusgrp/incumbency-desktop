use std::{sync::{Mutex, Arc}};
use tauri::State;
use crate::{generation::generate_game, entities::{business::Business, person::Person}, util::{percentage_based_output_int}};

pub struct GameState {
  tax_rate: f32,
  businesses: Vec<Business>,
  people: Vec<Person>,
  gdp: f32,
}

pub type GameStateSafe = Arc<Mutex<GameState>>;

impl Default for GameState {
  fn default() -> Self {
      Self {
        tax_rate: 24.,
        businesses: Vec::new(),
        people: Vec::new(),
        gdp: 0.
      }
  }
}

const DATA_PATH: &str = "./data";
const SAVES_PATH: &str = "./data/saves";

fn check_data_directories() {
  if !std::path::Path::new(DATA_PATH).exists() {
    std::fs::create_dir(DATA_PATH).unwrap();
    std::fs::create_dir(SAVES_PATH).unwrap();

    return;
  }

  if !std::path::Path::new(SAVES_PATH).exists() {
    std::fs::create_dir(SAVES_PATH).unwrap();
  }
}

#[tauri::command]
pub fn check_save_exists(name: String) -> bool {
  check_data_directories();
  std::path::Path::new(&format!("{}/{}", SAVES_PATH, name)).exists()
}

#[tauri::command]
pub fn create_game(state: State<'_, GameStateSafe>, name: String) {
  std::fs::create_dir(format!("{}/{}", SAVES_PATH, name)).unwrap();
  generate_game(&state);
}