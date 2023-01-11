use std::{time::Duration};
use crate::{common::{payloads::{PayloadNewDay, NewGame}}};
use tauri::{State, Manager};

use super::{generation::generate_game, state_manager::GameStateSafe};

#[tauri::command] // TODO: Take in game name as argument and call "create_save(name)"
pub async fn create_game(state_mux: State<'_, GameStateSafe>, app_handle: tauri::AppHandle) -> Result<(), ()> {

    generate_game(&state_mux);
    {
        let state = state_mux.lock().unwrap();
        app_handle.emit_all("game_created", NewGame { population: state.people.len() as i32 }).unwrap();
    } // need these or state will never unlock;
    start_game_loop(&state_mux, &app_handle);


    Ok(())
}

#[tauri::command]
pub fn set_tax(state_mux: State<'_, GameStateSafe>, tax_rate: f32) {
    let mut state = state_mux.lock().unwrap();
    state.tax_rate = tax_rate / 100.;
}

pub async fn start_game_loop(state_mux: &GameStateSafe, app_handle: &tauri::AppHandle) {
    let mut interval = tokio::time::interval(Duration::from_millis(10)); // TODO: put me back to seconds
    let mut day = 1;

    loop {
        interval.tick().await;
        day += 1;
        app_handle.emit_all("new_day", PayloadNewDay { day }).unwrap();

        let state = &mut state_mux.lock().unwrap();

        state.day_pass(day);
 
        if day % 31 == 0 {
            let tax_rate = state.tax_rate; // Dont need to .clone on basic types like f32
            state.month_pass(day / 31, tax_rate);
        }
    }
}