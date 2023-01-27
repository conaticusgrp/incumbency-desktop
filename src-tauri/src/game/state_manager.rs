use std::{sync::{Mutex, Arc}};
use maplit::hashmap;
use serde_json::json;
use crate::{entities::{business::{Business, ProductType}, person::{person::{Person, Job, Birthday}, debt::{Debt, DebtType}, self}}, as_decimal_percent, common::{util::{Date, SlotArray, set_decimal_count, percentage_chance, chance_one_in}, config::Config}};
use tauri::Manager;

#[derive(Clone)]
pub struct GameState {
  pub tax_rate: f32,
  pub business_tax_rate: f32,
  pub businesses: Vec<Business>,
  pub people: Vec<Person>,
  pub gdp: f32,
  pub date: Date,

  pub government_balance: u64, // This is expected to be quite large
  pub healthcare_investment: f64,

  pub hospital_total_capacity: i32,
  pub hospital_current_capacity: i32,
  pub cost_per_hospital_capacity: f64, // This is the cost per person capacity in a hospital for the government, each month
  pub month_unhospitalised_count: i32, // Number of patient that could not go to hospital because of the full capacity

  pub population_counter: f64,

  pub births_in_last_month: SlotArray<i32>,
  pub deaths_in_last_month: SlotArray<usize>,
}

const GOVERNMENT_START_BALANCE: u32 = 12000000; // TODO: changeme
const POPULATION_DAILY_INCREASE_PERCENTAGE: f32 = 4.8125e-5; // Based on real world statistics - TODO: make me more dynamic

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

            government_balance: GOVERNMENT_START_BALANCE as u64,
            healthcare_investment: GOVERNMENT_START_BALANCE as f64 * 0.07, // For now, 7% of the government budget should be spent on hospitals
            
            hospital_total_capacity: 0,
            hospital_current_capacity: 0,
            cost_per_hospital_capacity: 0.,
            month_unhospitalised_count: 0,

            population_counter: 0.,

            births_in_last_month: SlotArray::new(30),
            deaths_in_last_month: SlotArray::new(30),
        }
    }
}

impl GameState {
    /// Returns whether the new investment is possible, if not it also returns the minimum healthcare investment
    pub fn set_healthcare_investment(&mut self, investment: f64) -> (bool, Option<f64>) {
        if investment < self.healthcare_investment {
            let minimum_investment = (self.hospital_total_capacity - self.hospital_current_capacity) as f64 * self.cost_per_hospital_capacity;
            if investment < minimum_investment {
                return (false, Some(minimum_investment));
            }
        }

        let previous_total_capacity = self.hospital_total_capacity;
        self.hospital_total_capacity = (investment / self.cost_per_hospital_capacity) as i32;
        self.hospital_current_capacity += self.hospital_total_capacity - previous_total_capacity;

        (true, None)
    }

    pub fn day_pass(&mut self, day: i32, app_handle: Option<&tauri::AppHandle>, config: &Config) {
        let mut death_queue: Vec<usize> = Vec::new(); // Queue of people who are going to die :) - we need this because rust memory

        let date = self.date.clone();
        for per in self.people.iter_mut() {
            per.day_pass(day, &mut self.hospital_current_capacity, &mut self.month_unhospitalised_count, &date, &mut death_queue, &mut self.businesses);
            if per.age >= 18 && per.job == Job::Unemployed {
                // TODO: make this be affected by other factors
                if !per.homeless && chance_one_in(2000 * 365) { // 1 in 2000 chance every year
                    per.homeless = true;
                }
            }
        }

        let population_before_deaths = self.people.len() as i32;

        for id in &death_queue {
            let idx = self.people.iter().position(|p| p.id == *id).unwrap();
            self.people.remove(idx);
            let new_birth_count = 0;
            self.population_counter -= 1.;
        }

        self.deaths_in_last_month.push(death_queue.len());

        if let Some(app) = app_handle {
            app.emit_all("debug_payload",  hashmap! { "Population" => self.people.len() }).unwrap();
        }

        self.population_counter += (self.people.len() as f32 * POPULATION_DAILY_INCREASE_PERCENTAGE) as f64;

        let mut new_birth_count = self.population_counter.floor() as i32 - population_before_deaths;
        if new_birth_count < 0 {
            new_birth_count = 0;
        }

        for _ in 0..new_birth_count {
            let infant = Person::new_infant(self.people.len(), Birthday::from(&date), config);
            self.people.push(infant);
        }

        self.births_in_last_month.push(new_birth_count);
    }

    pub fn month_pass(&mut self, tax_rate: f32, app_handle: Option<&tauri::AppHandle>) {
        for person in self.people.iter_mut() {
            person.calculate_demand(person.salary as f32, None);

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
                if !Debt::required_to_pay(&person) {
                    break
                }

                let debt = &mut person.debts[i];

                if debt.debt_type == DebtType::Education && person.age < (18 + person.years_in_higher_education) {
                    continue
                }

                if debt.owed < debt.minimum_monthly_payoff {
                    person.balance -= debt.owed;
                    person.debts.remove(i);

                    continue;
                }

                debt.owed -= debt.minimum_monthly_payoff;

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

        self.government_balance -= self.healthcare_investment as u64;
        
        if let Some(app) = app_handle {
            let mut births_total = 0;
            let mut deaths_total = 0;

            for day_amount in &self.births_in_last_month.array {
                births_total += day_amount; 
            }

            for day_amount in &self.deaths_in_last_month.array {
                deaths_total += day_amount;
            }

            let mut total_welfare_percentage = 0;
            let mut unemployed_count = 0; // Does not include the homeless
            let mut homeless_count = 0;

            for person in self.people.iter() {
                total_welfare_percentage += person.get_welfare();
                if person.homeless { homeless_count += 1; continue }

                if person.job == Job::Unemployed {
                    unemployed_count += 1;
                }
            }

            let average_welfare = total_welfare_percentage as f32 / self.people.len() as f32;
            let average_welfare = set_decimal_count(average_welfare, 2);

            // TODO - send me daily
            app.emit_all("debug_payload",  json! ({
                "Average Welfare": average_welfare,
                "Government Balance": self.government_balance,
                "Monthly Births": births_total,
                "Monthly Deaths": deaths_total,
                "Homeless Count": homeless_count,
                "Unemployed Count": unemployed_count,
            })).unwrap();
        }

        self.month_unhospitalised_count = 0;
    }

    fn get_demand(&self, product_type: &ProductType) -> f32 {
        let mut total: f32 = 0.;

        for person in self.people.iter() {
            total += person.demand[product_type];
        }

        total
    }
}