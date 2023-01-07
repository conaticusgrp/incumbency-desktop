use std::{time::Duration, sync::{Arc, Mutex}};
use crate::{entities::{business::Business, person::{Person, Job}}, common::{filesystem::create_save, payloads::{PayloadNewDay, NewGame}}};
use tauri::{State, Manager};

use super::generation::generate_game;

#[derive(Clone)]
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
          Job::BusinessOwner(bus_idx) => {
            self.people[i].balance += self.businesses[bus_idx].employee_salary as f32;
            self.businesses[bus_idx].balance -= self.businesses[bus_idx].employee_salary as f32;
          },

          Job::Employee(bus_idx) => {
            self.people[i].balance += self.businesses[bus_idx].employee_salary as f32;
            self.businesses[bus_idx].balance -= self.businesses[bus_idx].employee_salary as f32;
          },

          _ => (),
        };
      }

      for i in 0..self.businesses.len() {
        let month_profits = self.businesses[i].balance - self.businesses[i].last_month_balance;
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


#[tauri::command] // TODO: Take in game name as argument and call "create_save(name)"
pub async fn create_game(state_mux: State<'_, GameStateSafe>, app_handle: tauri::AppHandle) -> Result<(), ()> {
  // create_save(name);
  generate_game(&state_mux);
  start_game_loop(&state_mux, &app_handle).await;

  let state = state_mux.lock().unwrap();
  app_handle.emit_all("game_created", NewGame { population: state.people.len() as i32 }).unwrap();

  Ok(())
}

#[tauri::command]
pub fn set_tax(state_mux: State<'_, GameStateSafe>, tax_rate: f32) {
  let mut state = state_mux.lock().unwrap();
  state.tax_rate = tax_rate;
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