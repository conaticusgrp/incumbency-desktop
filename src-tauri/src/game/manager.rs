use std::{time::Duration, sync::{Arc, Mutex}, borrow::{Borrow, BorrowMut}};
use crate::{entities::{business::Business, person::{Person, Job}}, common::{filesystem::create_save, payloads::{PayloadNewDay, NewGame}}, as_decimal_percent, percentage_of};
use tauri::{State, Manager};

use super::generation::generate_game;

#[derive(Clone)]
pub struct GameState {
  pub tax_rate: f32,
  pub business_tax_rate: f32,
  pub businesses: Vec<Business>,
  pub people: Vec<Person>,
  pub gdp: f32,
}

impl GameState {
  fn day_pass(&mut self, day: i32) {
    for i in 0..self.people.len() {
      let person = &mut self.people[i];
      let business_this_month = person.business_this_month;

      let quantity_opt = person.purchase_days.get(&day);
      if let Some(quantity) = quantity_opt {
          let business = self.businesses.get_mut(business_this_month).unwrap();
          let item_cost = (business.product_price * quantity) as f32;

          for _ in 0..*quantity {
              if person.can_afford(item_cost) {
                  person.balance -= item_cost;
                  *person.wants.get_mut(&business.product_type).unwrap() -= item_cost;
                  business.balance += item_cost;

                  // TODO - fulfill the welfare of purchasing the item
              }

              // TODO: handle welfare on not affording an item
          }
      }
    }
  }

  fn month_pass(&mut self, month: i32, tax_rate: f32) {
      for person in self.people.iter_mut() {
        let mut income = person.salary as f32;
        income -= income * tax_rate;

        person.balance += income;

        match person.job {
          Job::BusinessOwner(bus_idx) => {
            let business = &mut self.businesses[bus_idx];
            person.balance += business.employee_salary as f32;
            business.balance -= business.employee_salary as f32;
          },

          Job::Employee(bus_idx) => {
            let business = &mut self.businesses[bus_idx];
            person.balance += business.employee_salary as f32;
            business.balance -= business.employee_salary as f32;
          },

          _ => (),
        };
      }

      let mut reinvestment_budgets: Vec<(usize, f32)> = Vec::new();
      let mut total_reinvestment_budget = 0.;

      for i in 0..self.businesses.len() {
        let business = &mut self.businesses[i];

        let month_profits = business.balance - business.last_month_balance;
        business.balance -= month_profits * tax_rate;

        if month_profits < 0. { continue }
        let reinvesment_budget = business.balance * as_decimal_percent!(business.marketing_cost_percentage);
        reinvestment_budgets.push((i, reinvesment_budget));
        total_reinvestment_budget += reinvesment_budget;

        // add all businesses total
        // each business is assigned a market percentage (based on a calculated mean)
        // buy as much as the business can afford
      }

      let mut remaining_market_percentage: f32 = 100.;
      let mut cost_per_percent = 0.;

      for i in 0..reinvestment_budgets.len() {
        let (bus_idx, budget) = &reinvestment_budgets[i];
        let business = &mut self.businesses[*bus_idx];

        let maximum_percentage = (budget / total_reinvestment_budget) * 100.;
        
        if i == 0 {
          cost_per_percent = budget / maximum_percentage;
        }

        let mut assigned_percent = budget / cost_per_percent;

        if (remaining_market_percentage - assigned_percent) < 0. {
          assigned_percent = remaining_market_percentage;
        }

        business.balance -= assigned_percent * cost_per_percent; // remove from business balance

        remaining_market_percentage -= assigned_percent;
      }
  }
}

pub type GameStateSafe = Arc<Mutex<GameState>>;

impl Default for GameState {
  fn default() -> Self {
      Self {
        tax_rate: 0.24, // 24% default
        business_tax_rate: 0.22, // 22% default - TODO: emit warning if the tax is raised above 30% - this is the maximum tax rate businesses will tolerate
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
        let tax_rate = state.tax_rate.clone();

        state.day_pass(day);
 
        if day % 31 == 0 {
          state.month_pass(day / 31, tax_rate);
        }
    }
}