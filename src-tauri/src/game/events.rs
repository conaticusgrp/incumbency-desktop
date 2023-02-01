use tauri::State;

use super::state_manager::GameStateSafe;

#[derive(PartialEq, Eq, Hash)]
pub enum App {
    Finance = 2,
    Healthcare = 3,
    Welfare = 4,
    Business = 5,
}

pub fn get_app_from_id(app_id: u8) -> App {
    match app_id {
        a if a == App::Finance as u8 => App::Finance,
        a if a == App::Healthcare as u8 => App::Healthcare,
        a if a == App::Welfare as u8 => App::Welfare,
        a if a == App::Business as u8 => App::Business,
        _ => unreachable!(),
    }
}

#[tauri::command]
pub fn app_open(state_mux: State<'_, GameStateSafe>, app_id: u8) {
    let mut state = state_mux.lock().unwrap();

    let app = get_app_from_id(app_id); 
    *state.open_apps.entry(app).or_insert(true) = true;
}

#[tauri::command]
pub fn app_close(state_mux: State<'_, GameStateSafe>, app_id: u8) {
    let mut state = state_mux.lock().unwrap();

    let app = get_app_from_id(app_id); 
    *state.open_apps.entry(app).or_insert(false) = false;
}