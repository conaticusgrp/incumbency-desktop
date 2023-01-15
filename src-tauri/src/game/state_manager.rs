use std::{sync::{Mutex, Arc}};
use rand::Rng;
use crate::{entities::{business::{Business, ProductType}, person::person::{Person, Job}}, as_decimal_percent, common::util::{Date}};

#[derive(Clone)]
pub struct GameState {
  pub tax_rate: f32,
  pub business_tax_rate: f32,
  pub businesses: Vec<Business>,
  pub people: Vec<Person>,
  pub gdp: f32,
  pub date: Date,

  pub government_balance: f32,
  pub healthcare_investment: f32,

  pub hospital_total_capacity: i32,
  pub hospital_current_capacity: i32,
  pub cost_per_hospital_capacity: i32, // This is the cost per person capacity in a hospital for the government, each month
  pub month_unhospitalised_count: i32, // Number of patient that could not go to hospital because of the full capacity
}

const GOVERNMENT_START_BALANCE: f32 = 12000000.; // TODO: changeme

pub type GameStateSafe = Arc<Mutex<GameState>>;

impl Default for GameState {
    fn default() -> Self {
        Self {
            tax_rate: 0.24, // 24% default
            business_tax_rate: 0.22, // 22% default - TODO: emit warning if the tax is raised above 30% - this is the maximum tax rate businesses will tolerate
            businesses: Vec::new(),
            people: Vec::new(),
            gdp: 0.,
            date: Date::default(),

            government_balance: GOVERNMENT_START_BALANCE,
            healthcare_investment: GOVERNMENT_START_BALANCE * 0.07, // For now, 7% of the government budget should be spent on hospitals
            
            hospital_total_capacity: 0,
            hospital_current_capacity: 0,
            cost_per_hospital_capacity: 0,
            month_unhospitalised_count: 0,
        }
    }
}

impl GameState {
    /// Returns whether the new investment is possible, if not it also returns the minimum healthcare investment
    pub fn set_healthcare_investment(&mut self, investment: f32) -> (bool, Option<f32>) {
        if investment < self.healthcare_investment {
            let minimum_investment = self.healthcare_investment - (self.hospital_current_capacity as f32 * self.cost_per_hospital_capacity as f32);
            if investment < minimum_investment {
                return (false, Some(minimum_investment));
            }
        }

        let previous_total_capacity = self.hospital_total_capacity;
        self.hospital_total_capacity = self.cost_per_hospital_capacity * investment as i32;
        self.hospital_current_capacity += self.hospital_total_capacity - previous_total_capacity;

        (true, None)
    }

    pub fn day_pass(&mut self, day: i32) {
        let mut death_queue: Vec<usize> = Vec::new(); // Queue of people who are going to die :) - we need this because rust memory

        let date = self.date.clone();

        for per in self.people.iter_mut() {
            per.check_birthday(&date);
            per.balance -= per.daily_food_spending as f32;

            let mut rng = rand::thread_rng();

            let loss_chance: f32 = match per.daily_food_spending { // Chance that the individual will lose 1% of their health
                1 => 50.,
                2 => 25.,
                3 => 0.9,
                4 => 0.6,
                _ => unreachable!(),
            };

            let maximum = 100 / loss_chance as i32;
            let has_loss = rng.gen_range(0..=maximum) == maximum;
            if has_loss {
                per.remove_health(1, &mut self.hospital_current_capacity, &mut self.month_unhospitalised_count);
            }

            if let Some(ref mut days) = per.days_until_death {
                *days -= 1;
                if *days <= 0 {
                    death_queue.push(per.idx);
                    continue;
                }
            }

            if let Some(ref mut days) = per.days_left_in_hospital {
                *days -= 1;
                if *days <= 0 {
                    per.days_left_in_hospital = None;
                    self.hospital_current_capacity += 1;
                }
            }

            per.replenish_health();

            let business_this_month = per.business_this_month;

            let quantity_opt = per.purchase_days.get(&day);

            if let Some(quantity) = quantity_opt {
                let business = self.businesses.get_mut(business_this_month).unwrap();
                let item_cost = (business.product_price * quantity) as f32;

                for _ in 0..*quantity {
                    if per.can_afford(item_cost) {
                        per.balance -= item_cost;
                        *per.demand.get_mut(&business.product_type).unwrap() -= item_cost;
                        business.balance += item_cost;
                        // TODO - fulfill the welfare of purchasing the item
                    }
                    // TODO: handle welfare on not affording an item
                }
            }
        }

        for person_idx in death_queue {
            self.people.remove(person_idx);
        }
    }

    pub fn month_pass(&mut self, tax_rate: f32) {
        for person in self.people.iter_mut() {
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

            person.calculate_daily_food_spending();
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

        self.month_unhospitalised_count = 0;
        self.government_balance -= self.healthcare_investment;
    }

    fn get_demand(&self, product_type: &ProductType) -> f32 {
        let mut total: f32 = 0.;

        for person in self.people.iter() {
            total += person.demand[&product_type];
        }

        total
    }
}