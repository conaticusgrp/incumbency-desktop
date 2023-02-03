use std::{sync::{Mutex, Arc}, collections::HashMap};
use serde_json::json;
use uuid::Uuid;
use crate::{entities::{business::{Business}, person::{person::{Person, Job, Birthday}, debt::{Debt, DebtType}}}, as_decimal_percent, common::{util::{Date, SlotArray, set_decimal_count, chance_one_in, generate_unemployed_salary, get_healthcare_group}, config::Config}};
use tauri::{Manager, AppHandle};
use super::{structs::{GameState, GameStateRules, HealthcareGroup, HealthcareState, FinanceData}, events::{update_app, App}};

const GOVERNMENT_START_BALANCE: u32 = 12000000; // TODO: changeme
// const POPULATION_DAILY_INCREASE_PERCENTAGE: f32 = 4.8125e-5; // Based on real world statistics - TODO: make me more dynamic
const POPULATION_DAILY_INCREASE_PERCENTAGE: f32 = 1e-4; // Based on real world statistics - TODO: make me more dynamic

pub type GameStateSafe = Arc<Mutex<GameState>>;

impl Default for GameState {
    fn default() -> Self {
        Self {
            tax_rate: 0.24, // 24% default
            business_tax_rate: 0.22, // 22% default - TODO: emit warning if the tax is raised above 30% - this is the maximum tax rate businesses will tolerate
            businesses: HashMap::new(),
            people: HashMap::new(),
            date: Date::default(),

            government_balance: GOVERNMENT_START_BALANCE as i64,
            population_counter: 0.,

            births_in_last_month: SlotArray::new(30),
            deaths_in_last_month: SlotArray::new(30),

            total_possible_purchases: 0,
            purchases: 0,

            rules: GameStateRules::default(),
            open_apps: HashMap::new(),
            healthcare: HealthcareState::default(),

            finance_data: FinanceData::default(),
            welfare_budget: (GOVERNMENT_START_BALANCE as f64 * 0.15) as i64,
            business_budget: (GOVERNMENT_START_BALANCE as f64 * 0.1) as i64,
            spare_budget: (GOVERNMENT_START_BALANCE as f64 * 0.55) as i64,
        }
    }
}

impl GameState {
    /// Returns whether the new budget is possible, if not it also returns the minimum healthcare budget
    // pub fn set_healthcare_budget(&mut self, budget: f32) -> (bool, Option<f32>) {
    //     if budget < self.healthcare {
    //         let minimum_investment = (self.hospital_total_capacity - self.hospital_current_capacity) as f32 * self.cost_per_hospital_capacity;
    //         if budget < minimum_investment {
    //             return (false, Some(minimum_investment));
    //         }
    //     }

    //     let previous_total_capacity = self.hospital_total_capacity;
    //     self.hospital_total_capacity = (budget / self.cost_per_hospital_capacity) as i32;
    //     self.hospital_current_capacity += self.hospital_total_capacity - previous_total_capacity;

    //     (true, None)
    // }

    pub fn get_spare_budget(&self) -> i64 {
        self.government_balance - (self.healthcare.budget + self.welfare_budget + self.business_budget) 
    }

    pub fn day_pass(&mut self, day: i32, app_handle: Option<&tauri::AppHandle>, config: &Config) {
        let mut death_queue: Vec<Uuid> = Vec::new(); // Queue of people who are going to die :) - we need this because rust memory

        let date = self.date.clone();
        let mut food_coverage = 0;
        let mut unemployed_food_coverage = 0;

        let total_monthly_income: i64 = 0;

        for per in self.people.values_mut() {
            per.day_pass(day, &mut self.healthcare, &date, &mut death_queue, &mut self.businesses, &mut self.purchases, &mut self.total_possible_purchases, &self.rules, &mut food_coverage, &mut unemployed_food_coverage);

            total_monthly_income += (per.salary / 12) as i64;

            if per.age >= 18 && per.job == Job::Unemployed {
                // TODO: make this be affected by other factors
                if !per.homeless && chance_one_in(500 * 365) { // 1 in 500 chance every year
                    per.homeless = true;
                }
            }
        }

        self.finance_data.average_monthly_income = (total_monthly_income / self.people.len() as i64) as i32;

        let population_before_deaths = self.people.len() as i32;

        for id in &death_queue {
            let per = self.people.get(&id).unwrap();
            let healthcare_group = get_healthcare_group(per.age, &mut self.healthcare);
            healthcare_group.current_capacity += 1;

            if let Job::Employee(bid) = per.job {
                let business = self.businesses.get_mut(&bid);
                if business.is_some() {
                    let business = business.unwrap();
                    let employee_idx = business.employees.iter().position(|emp_id| per.id == *emp_id).unwrap();
                    business.employees.remove(employee_idx);
                }
            }

            self.people.remove(&id);
            self.population_counter -= 1.;
        }

        self.deaths_in_last_month.push(death_queue.len());

        self.population_counter += (self.people.len() as f32 * POPULATION_DAILY_INCREASE_PERCENTAGE) as f64;

        let mut new_birth_count = self.population_counter.floor() as i32 - population_before_deaths;
        if new_birth_count < 0 {
            new_birth_count = 0;
        }

        for _ in 0..new_birth_count {
            let infant = Person::new_infant(Birthday::from(&date), config, self.tax_rate, &self.rules.tax_rule);
            self.people.insert(infant.id, infant);
        }

        self.births_in_last_month.push(new_birth_count);

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

            for person in self.people.values_mut() {
                person.get_welfare();

                total_welfare_percentage += person.welfare;
                if person.homeless { homeless_count += 1; continue }

                if person.job == Job::Unemployed && person.age >= 18 {
                    unemployed_count += 1;
                }
            }

            let average_welfare = total_welfare_percentage as f32 / self.people.len() as f32;
            let average_welfare = set_decimal_count(average_welfare, 2);

            self.spare_budget = self.get_spare_budget();

            update_app(App::Finance, json!({
                "government_balance": self.government_balance,
                "spare_budget": self.spare_budget,
                "used_hospital_capacity": self.healthcare.get_current_capacity(),
            }), &app);

            // TODO - send me daily
            app.emit_all("debug_payload",  json! ({
                "Population": self.people.len(),
                "Average Welfare": average_welfare,
                "Government Balance": self.government_balance,
                "Monthly Births": births_total,
                "Monthly Deaths": deaths_total,
                "Homeless Count": homeless_count,
                "Unemployed Count": unemployed_count,
            })).unwrap();
        }
    }

    pub fn month_pass(&mut self, app_handle: &AppHandle) {
        self.finance_data.expected_person_income = 0;

        for person in self.people.values_mut() {
            person.business_this_month = None;

            let tax_rate = Person::get_tax_rate(&self.rules.tax_rule, self.tax_rate, person.salary);
            person.calculate_demand(person.salary, None, tax_rate);

            match person.job {
                Job::BusinessOwner(bid) | Job::Employee(bid) => {
                    let business = self.businesses.get_mut(&bid);
                    if business.is_some() {
                        let business = business.unwrap();

                        let tax_payment = (person.salary as f32 / 12.) * tax_rate;
                        self.finance_data.expected_person_income += tax_payment as i64;

                        person.pay_tax(&mut self.government_balance, (person.salary as f32 / 12.) * tax_rate);
                        person.business_pay(business, business.employee_salary as f64 / 12.);
                    } else {
                        person.job = Job::Unemployed;
                        person.set_salary(generate_unemployed_salary());
                    }

                    // business.pay_owner(person);
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
                    person.get_monthly_debt_cost();

                    continue;
                }

                debt.owed -= debt.minimum_monthly_payoff;

                // Add functionality to welfare if they can't afford debts
                person.balance -= debt.minimum_monthly_payoff;
            }
        }

        let mut reinvestment_budgets: Vec<(Uuid, f64)> = Vec::new();
        let mut total_reinvestment_budget = 0.;

        let mut bus_removal_queue: Vec<Uuid> = Vec::new();

        self.finance_data.expected_business_income = 0;

        for business in self.businesses.values_mut() {
            Business::check_funding(&self.rules.business_funding_rule, business);
            business.last_month_income = business.balance - business.last_month_balance;
            
            // TODO: make me more varied
            if business.balance <= 0. {
                bus_removal_queue.push(business.id);
            }

            let tax_rate = Business::get_tax_rate(&self.rules.business_tax_rule, business.last_month_income, self.business_tax_rate);
            let tax_cost = business.last_month_income * tax_rate as f64;
            self.finance_data.expected_business_income += tax_cost as i64;

            business.pay_tax(&mut self.government_balance, business.last_month_income * tax_rate as f64);
            let reinvesment_budget = business.balance * as_decimal_percent!(business.marketing_cost_percentage) as f64;

            if reinvesment_budget > 0. {
                total_reinvestment_budget += reinvesment_budget;
                reinvestment_budgets.push((business.id, reinvesment_budget));
            }
        }

        let mut remaining_market_percentage: f32 = 100.;
        let mut cost_per_percent = 0.;

        let purchase_rate = self.purchases as f32 / self.total_possible_purchases as f32;

        for i in 0..reinvestment_budgets.len() {
            let (bid, budget) = &reinvestment_budgets[i];

            let maximum_percentage = (budget / total_reinvestment_budget) * 100.;
        
            if i == 0 {
                cost_per_percent = (budget / maximum_percentage) as f32;
            }

            let mut assigned_percent = (budget / cost_per_percent as f64) as f32;

            if (remaining_market_percentage - assigned_percent) < 0. {
                assigned_percent = remaining_market_percentage;
            }

            let mut demand: f32 = 0.;

            let business = self.businesses.get_mut(&bid).unwrap();

            for person in self.people.values() {
                demand += person.demand[&business.product_type];
            }

            business.get_new_market(assigned_percent, cost_per_percent, &mut self.people, demand, purchase_rate);
            business.last_month_balance = business.balance;

            remaining_market_percentage -= assigned_percent;
        }

        for id in bus_removal_queue {
            self.businesses.remove(&id);
        }

        self.government_balance -= self.healthcare.budget as i64;
        self.healthcare.month_unhospitalised_count = 0;
        self.total_possible_purchases = 0;
        self.purchases = 0;

        update_app(App::Finance, json!({
            "average_monthly_income": self.finance_data.average_monthly_income,
            "expected_person_income": self.finance_data.expected_person_income,
            "expected_business_income": self.finance_data.expected_business_income,
        }), app_handle);
    }
}