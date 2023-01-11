use std::sync::{Mutex, Arc};

use crate::{entities::{business::{Business, ProductType}, person::person::{Person, Job}}, as_decimal_percent};

#[derive(Clone)]
pub struct GameState {
  pub tax_rate: f32,
  pub business_tax_rate: f32,
  pub businesses: Vec<Business>,
  pub people: Vec<Person>,
  pub gdp: f32,
  pub government_balance: f32,
}

// Spend 4M35S fixing cons tabs - Kventis
impl GameState {
    pub fn day_pass(&mut self, day: i32) {
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

    pub fn month_pass(&mut self, _month: i32, tax_rate: f32) {

        for person in self.people.iter_mut() {
            // TODO: pay wants on a monthly basis
            let income = person.salary as f32;
            person.balance += income;

            person.pay_tax(&mut self.government_balance, income * tax_rate);

            match person.job {
            Job::BusinessOwner(bus_idx) => {
                let business = &mut self.businesses[bus_idx];
                business.pay_owner(person);
            },

            Job::Employee(bus_idx) => {
                let business = &mut self.businesses[bus_idx];
                person.business_pay(business, business.employee_salary as f32);
            },

            _ => (),
            };

            for i in 0..person.debts.len() {
            // TODO: Add functionality based on spending behaviour
                let debt = &mut person.debts[i];
                if !debt.required_to_pay { continue }

                if debt.owed < debt.minimum_monthly_payoff {
                    person.balance -= debt.owed;
                    person.debts.remove(i);
                    continue;
                }

          // Add functionality to welfare if they can't afford debts
                 person.balance -= debt.minimum_monthly_payoff;
            }
        }

        let mut reinvestment_budgets: Vec<(usize, f32)> = Vec::new();
        let mut total_reinvestment_budget = 0.;

        for i in 0..self.businesses.len() {
            let business = &mut self.businesses[i];

            let month_profits = business.balance - business.last_month_balance;
            business.pay_tax(&mut self.government_balance, month_profits * tax_rate);

            if month_profits < 0. { continue }

            let reinvesment_budget = business.balance * as_decimal_percent!(business.marketing_cost_percentage);
            reinvestment_budgets.push((i, reinvesment_budget));
            total_reinvestment_budget += reinvesment_budget;
        }

        let mut remaining_market_percentage: f32 = 100.;
        let mut cost_per_percent = 0.;

        for i in 0..reinvestment_budgets.len() {
            let (bus_idx, budget) = &reinvestment_budgets[i];

            let maximum_percentage = (budget / total_reinvestment_budget) * 100.;
        
            if i == 0 {
            cost_per_percent = budget / maximum_percentage;
            }

            let mut assigned_percent = budget / cost_per_percent;

            if (remaining_market_percentage - assigned_percent) < 0. {
            assigned_percent = remaining_market_percentage;
            }
            let demand = self.get_demand(&self.businesses[*bus_idx].product_type);
            let business = &mut self.businesses[*bus_idx];

            business.get_new_market(assigned_percent, cost_per_percent, &mut self.people, demand, *bus_idx);
            business.last_month_balance = business.balance;

            remaining_market_percentage -= assigned_percent;
        }
    }

    fn get_demand(&self, product_type: &ProductType) -> f32 {
        let mut total: f32 = 0.;

        for person in self.people.iter() {
            total += person.wants[&product_type];
        }

        total
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
            gdp: 0.,
            government_balance: 0.,
        }
    }
}