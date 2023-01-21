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

pub fn create_save(name: String) {
  std::fs::create_dir(format!("{}/{}", SAVES_PATH, name)).unwrap();
}

#[tauri::command]
pub fn check_save_exists(name: String) -> bool {
  check_data_directories();
  std::path::Path::new(&format!("{}/{}", SAVES_PATH, name)).exists()
}