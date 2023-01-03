use std::{time::Duration, sync::{Mutex, Arc}};

use crate::entities::{business::Business, person::Person};

pub struct GameState {
  pub tax_rate: f32,
  pub businesses: Vec<Business>,
  pub people: Vec<Person>,
  pub gdp: f32,
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

pub async fn start_game_loop(state_mux: &GameStateSafe) {
    let mut state = state_mux.lock().unwrap();

    let mut interval = tokio::time::interval(Duration::from_secs(1));
    let mut day = 1;

    loop {
        interval.tick().await;
        day += 1;

        // new day
        for person in state.people.iter_mut() {
            person.day_pass(&mut state, day);
        }
        
        if day % 30 == 0 {
            // new month
        }
    }
}