use std::{time::Duration, sync::{Mutex, Arc}};

use crate::entities::{business::Business, person::Person};

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
        state.day_pass(day);
        
        if day % 30 == 0 {
            // new month
        }
    }
}