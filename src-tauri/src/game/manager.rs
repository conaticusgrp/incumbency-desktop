use std::{time::Duration, sync::{Arc, Mutex}};
use crate::{entities::{business::Business, person::{Person, Job}}, common::{filesystem::create_save, payloads::PayloadNewDay}};
use tauri::{State, Manager};

use super::generation::generate_game;

pub struct GameState {
  pub tax_rate: f32,
  pub businesses: Vec<Business>,
  pub people: Vec<Person>,
  pub gdp: f32,
}

impl GameState {
  fn day_pass(&mut self, day: i32) {
    for i in 0..self.people.len() {
      let purchase_days = &self.people[i].purchase_days;
      let business_this_month = self.people[i].business_this_month;


      let quantity_opt = purchase_days.get(&day);
      if let Some(quantity) = quantity_opt {
          let business = self.businesses.get_mut(business_this_month).unwrap();
          let item_cost = business.product_price as f32;

          for _ in 0..*quantity {
              if self.people[i].can_afford(item_cost) {
                  self.people[i].balance -= item_cost;
                  *self.people[i].wants.get_mut(&business.product_type).unwrap() -= item_cost;
                  business.balance += item_cost;
                  dbg!(business.balance);

                  // TODO - fulfill the welfare of purchasing the item
              }

              // TODO: handle welfare on not affording an item
          }
      }
    }
  }

  fn month_pass(&mut self, month: i32) {
      for i in 0..self.people.len() {
        self.people[i].balance += self.people[i].salary as f32;

        match self.people[i].job {
          // TODO: Handle business owner

          Job::Employee(bus_idx) => {
            self.people[bus_idx].balance += self.businesses[bus_idx].employee_salary as f32;
            self.businesses[bus_idx].balance -= self.businesses[bus_idx].employee_salary as f32;
          },
          _ => (),
        };
      }
  }
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

#[tauri::command]
pub async fn create_game(state: State<'_, GameStateSafe>, app_handle: tauri::AppHandle, name: String) -> Result<(), ()> {
  create_save(name);
  generate_game(&state);
  start_game_loop(&state, &app_handle).await;

  Ok(())
}

pub async fn start_game_loop(state_mux: &GameStateSafe, app_handle: &tauri::AppHandle) {
    let mut interval = tokio::time::interval(Duration::from_secs(1));
    let mut day = 1;

    loop {
        interval.tick().await;
        day += 1;
        app_handle.emit_all("new_day", PayloadNewDay { day }).unwrap();

        let mut state = state_mux.lock().unwrap();
        state.day_pass(day);
        
        if day % 30 == 0 {
          state.month_pass(day / 30);
            // handle new month
        }
    }
}