use std::{time::Duration, vec};
use crate::{common::{payloads::{PayloadNewDay}, config::{load_config, Config}, errors::{IncResult, Error, Severity}}};
use serde_json::json;
use tauri::{State, Manager};

use super::{generation::{generate_game, stabilize_game}, state_manager::{GameStateSafe}};

#[tauri::command] // TODO: Take in game name as argument and call "create_save(name)"
pub async fn create_game(state_mux: State<'_, GameStateSafe>, app_handle: tauri::AppHandle) -> IncResult<()> {
    let config = load_config().unwrap();
    app_handle.emit_all("loading_status", json!({
        "Generating Game": []
    })).unwrap();

    generate_game(&state_mux, &config, &app_handle)?;

    app_handle.emit_all("loading_status", json!({
        "Checking everything is stable": ["Checking busineses", "Checking jobs & salaries", "Checking economy is stable", "Checking welfare is sufficient", "Checking hospital capacity is sufficient"]
    })).unwrap();
    
    stabilize_game(&state_mux, &config, &app_handle)?;
    
    app_handle.emit_all("game_generated", ()).unwrap();
    app_handle.emit_all("open_debugger_app", ()).unwrap(); // Only in debug mode

    start_game_loop(&state_mux, &app_handle, &config).await;
    Ok(())
}

#[tauri::command]
pub fn reset_game(app_handle: tauri::AppHandle) {
    // do stuff
}

fn emit_error(app_handle: &tauri::AppHandle, error: &Error) {
    app_handle.emit_all("error", &error).unwrap();
}

pub async fn start_game_loop(state_mux: &GameStateSafe, app_handle: &tauri::AppHandle, config: &Config) {
    let mut interval = tokio::time::interval(Duration::from_micros(1));

    loop {
        interval.tick().await;

        let state = &mut state_mux.lock().unwrap();

        state.date.new_day();
        let date_string = state.date.get_date_string();
        app_handle.emit_all("new_day", PayloadNewDay { date: date_string }).unwrap();

        let day = state.date.day;
        let day_res = state.day_pass(day, Some(app_handle), config);
        
        match day_res {
            Err(err) => {
                if err.severity() == Severity::Fatal as u8 {
                    break;
                }

                emit_error(&app_handle, &err);
            },

            Ok(_) => (),
        }

        if state.date.on_new_month {
            let month_res = state.month_pass(app_handle);

            match month_res {
                Err(err) => {
                    if err.severity() == Severity::Fatal as u8 {
                        break;
                    }

                    emit_error(&app_handle, &err);
                },

                Ok(_) => (),
            }
        }
    }
}