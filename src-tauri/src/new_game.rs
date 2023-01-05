use tauri::State;
use crate::{generation::generate_game, game::{start_game_loop, GameStateSafe}};

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
pub async fn create_game(state: State<'_, GameStateSafe>, name: String) -> Result<(), ()> {
  std::fs::create_dir(format!("{}/{}", SAVES_PATH, name)).unwrap();

  generate_game(&state).await;
  start_game_loop(&state).await;

  Ok(())
}