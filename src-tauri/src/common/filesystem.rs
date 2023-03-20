use super::errors::IncResult;

const DATA_PATH: &str = "./data";
const SAVES_PATH: &str = "./data/saves";

fn check_data_directories() -> IncResult<()> {
    if !std::path::Path::new(DATA_PATH).exists() {
        std::fs::create_dir(DATA_PATH)?;
        std::fs::create_dir(SAVES_PATH)?;

        return Ok(());
    }

    if !std::path::Path::new(SAVES_PATH).exists() {
        std::fs::create_dir(SAVES_PATH)?;
    }

    Ok(())
}

// pub fn create_save(name: String) -> IncResult<()> {
//   std::fs::create_dir(format!("{}/{}", SAVES_PATH, name))?;
//   Ok(())
// }

#[tauri::command]
pub fn check_save_exists(name: String) -> IncResult<bool> {
    check_data_directories()?;
    Ok(std::path::Path::new(&format!("{}/{}", SAVES_PATH, name)).exists())
}
