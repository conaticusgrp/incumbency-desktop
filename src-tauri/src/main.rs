#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

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
fn check_save_exists(name: String) -> bool {
  check_data_directories();
  std::path::Path::new(&format!("{}/{}", SAVES_PATH, name)).exists()
}

#[tauri::command]
fn create_game(name: String) {
  std::fs::create_dir(format!("{}/{}", SAVES_PATH, name)).unwrap();
}

#[tauri::command]
fn list_saves() -> Vec<String> {
  check_data_directories();
  std::fs::read_dir(SAVES_PATH)
    .unwrap()
    .map(|entry| entry.unwrap().file_name().into_string().unwrap())
    .collect()
}

fn main() {
  tauri::Builder::default()
  .invoke_handler(tauri::generate_handler![create_game, check_save_exists, list_saves])
    .run(tauri::generate_context!())
    .expect("Error while running tauri application");
}
