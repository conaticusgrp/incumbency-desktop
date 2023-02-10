use std::{sync::{Mutex, Arc}, collections::HashMap};
use serde_json::json;
use uuid::Uuid;
use crate::{entities::{business::{Business, ProductType}, person::{person::{Person, Job, Birthday}, debt::{Debt, DebtType}}}, as_decimal_percent, common::{util::{Date, SlotArray, set_decimal_count, chance_one_in, generate_unemployed_salary, get_healthcare_group}, config::Config, errors::{IncResult, Error}}};
use tauri::{Manager, AppHandle};
use super::{structs::{GameState, GameStateRules, HealthcareState, FinanceData, BusinessData}, events::{update_app, App, json_get_i64}};

const GOVERNMENT_START_BALANCE: u32 = 12000000; // TODO: changeme

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

            births_in_last_month: SlotArray::new(30),
            deaths_in_last_month: SlotArray::new(30),

            total_possible_purchases: 0,
            purchases: 0,

            rules: GameStateRules::default(),
            open_apps: HashMap::new(),
            healthcare: HealthcareState::default(),

            finance_data: FinanceData::default(),
            welfare_budget: (GOVERNMENT_START_BALANCE as f64 * 0.15) as i64,
            welfare_owed: 0,

            business_budget: (GOVERNMENT_START_BALANCE as f64 * 0.1) as i64,
            business_owed: 0,

            spare_budget: (GOVERNMENT_START_BALANCE as f64 * 0.55) as i64,

            average_welfare: 100.,
            average_welfare_unemployed: 100.,

            business_data: BusinessData::default(),
            unemployed_count: 0,
        }
    }
}

impl GameState {
    pub fn check_healthcare_capacity(&self, new_total_capacity: i32, error_checker_failed: &mut bool) {
        if new_total_capacity > self.healthcare.total_capacity {
            return;
        }

        let lost_capacity =  self.healthcare.total_capacity - new_total_capacity;
        let remaining_capacity = self.healthcare.total_capacity - self.healthcare.get_current_capacity();
        if remaining_capacity - lost_capacity < 0 {
            *error_checker_failed = true;
        }
    }

    pub fn get_spare_budget(&self) -> i64 {
        self.government_balance - (self.healthcare.budget + self.welfare_budget + self.business_budget) 
    }

    pub fn day_pass(&mut self, day: i32, app_handle: Option<&tauri::AppHandle>, config: &Config) -> IncResult<()> {
        let date = self.date.clone();
        let mut food_coverage = 0;
        let mut unemployed_food_coverage = 0;

        let mut total_monthly_income: i64 = 0;

        self.healthcare.age_ranges = json!({
            "0-18": 0,
            "19-29": 0,
            "30-44": 0,
            "45-60": 0,
            "61-84": 0,
            "85+": 0,
        });

        let mut total_welfare = 0;
        let mut total_welfare_unemployed = 0;
        self.unemployed_count = 0; // Does not include the homeless
        let mut death_ages_total = 0;

        let mut death_queue: Vec<Uuid> = Vec::new();
        let mut new_birth_count: i32 = 0;

        for per in self.people.values_mut() {
            let key = match per.age {
                a if a <= 18 => "0-18",
                a if a <= 29 => "19-29",
                a if a <= 44 => "30-44",
                a if a <= 60 => "45-60",
                a if a <= 84 => "61-84",
                _ => "85+",
            };

            let int64 = json_get_i64(&self.healthcare.age_ranges, key)?;
            *self.healthcare.age_ranges.get_mut(key).ok_or(
                Error::Danger(format!("Could not find age range '{}' in age ranges.", key))
            )? = json!(int64 + 1); // Increment counter of age ranges for state

            let die = per.day_pass(day, &mut self.healthcare, &date, &mut self.businesses, &mut self.purchases, &mut self.total_possible_purchases, &self.rules, &mut food_coverage, &mut unemployed_food_coverage)?;

            total_monthly_income += (per.salary / 12) as i64;

            if die {
                death_queue.push(per.id);
                continue;
            }

            total_welfare += per.welfare;
            if per.job == Job::Unemployed && !per.homeless && per.age >= 18 {
                total_welfare_unemployed += per.welfare; 
                self.unemployed_count += 1;
            }

            if per.due_birth(&date, &mut self.healthcare, &self.rules) {
                new_birth_count += 1;
            }
        }

        self.welfare_owed += ((food_coverage + unemployed_food_coverage) * 4) as i64;
        self.finance_data.average_monthly_income = (total_monthly_income / self.people.len() as i64) as i32;

        for id in death_queue.iter() {
            let per = self.people.get_mut(id).ok_or(Error::DangerUnexpected)?;
            death_ages_total += per.age;

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

            self.people.remove(id);
        }

        if death_queue.len() > 0 {
            self.healthcare.life_expectancy = death_ages_total / death_queue.len() as i32;
        }
        
        self.deaths_in_last_month.push(death_queue.len());

        for _ in 0..new_birth_count {
            let infant = Person::new_infant(config, self.tax_rate, &self.rules.tax_rule, date.clone())?;
            self.people.insert(infant.id, infant);
        }

        self.births_in_last_month.push(new_birth_count);

        self.healthcare.births_per_month = 0;
        self.healthcare.deaths_per_month = 0;

        for day_amount in &self.births_in_last_month.array {
            self.healthcare.births_per_month += *day_amount as i32; 
        }

        for day_amount in &self.deaths_in_last_month.array {
            self.healthcare.deaths_per_month += *day_amount as i32;
        }

        self.average_welfare = total_welfare as f32 / self.people.len() as f32;
        self.average_welfare = set_decimal_count(self.average_welfare, 2);

        self.average_welfare_unemployed = total_welfare_unemployed as f32 / self.unemployed_count as f32;
        self.average_welfare_unemployed = set_decimal_count(self.average_welfare_unemployed, 3);

        self.spare_budget = self.get_spare_budget();
        self.emit_daily_events(app_handle);

        Ok(())
    }

    pub fn emit_daily_events(&self, app_handle: Option<&AppHandle>) {
        if app_handle.is_none() { return }

        let app_handle = app_handle.unwrap();

        update_app(App::Finance, json!({
            "government_balance": self.government_balance,
            "spare_budget": self.spare_budget,
            "used_hospital_capacity": self.healthcare.get_current_capacity(),
        }), &app_handle);

        update_app(App::Healthcare, json!({
            "population": self.people.len() as i32,
            "used_capacity": self.healthcare.get_current_capacity(),
            "total_capacity": self.healthcare.total_capacity,
            "age_ranges": self.healthcare.age_ranges,
            "child_care": self.healthcare.childcare,
            "adult_care": self.healthcare.adultcare,
            "elder_care": self.healthcare.eldercare,
            "births_per_month": self.healthcare.births_per_month,
            "deaths_per_month": self.healthcare.deaths_per_month
        }), &app_handle);

        update_app(App::Welfare, json!({
            "average_welfare": self.average_welfare,
            "average_unemployed_welfare": self.average_welfare_unemployed,
        }), &app_handle);

        update_app(App::Business, json!({
            "business_count": self.businesses.len() as i32,
        }), &app_handle);

        app_handle.emit_all("debug_payload",  json! ({
            "Population": self.people.len(),
            "Average Welfare": self.average_welfare,
            "Government Balance": self.government_balance,
            "Monthly Births": self.healthcare.births_per_month,
            "Monthly Deaths": self.healthcare.deaths_per_month,
            "Unemployed Count": self.unemployed_count,
        })).unwrap();
    }

    pub fn month_pass(&mut self, app_handle: &AppHandle) -> IncResult<()> {
        self.finance_data.expected_person_income = 0;

        for person in self.people.values_mut() {
            person.business_this_month = None;

            let tax_rate = Person::get_tax_rate(&self.rules.tax_rule, self.tax_rate, person.salary);
            person.calculate_demand(person.salary, None, tax_rate)?;

            match person.job {
                Job::BusinessOwner(bid) | Job::Employee(bid) => {
                    let business = self.businesses.get_mut(&bid);
                    if business.is_some() {
                        let business = business.unwrap();

                        let tax_payment = (person.salary as f32 / 12.) * tax_rate;
                        self.finance_data.expected_person_income += tax_payment as i64;

                        person.pay_tax(&mut self.government_balance, (person.salary as f32 / 12.) * tax_rate);
                        person.business_pay(business, business.employee_salary as f64 / 12.);

                        if person.age >= 65 && chance_one_in(60) {
                            // Retired
                            person.set_salary(15800); // TODO: vary pension salary
                            person.job = Job::Retired;

                            let emp_idx = business.employees.iter().position(|&id| id == person.id);
                            if emp_idx.is_some() {
                                let idx = emp_idx.unwrap();
                                business.employees.remove(idx);
                            }
                        }
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

        let mut total_business_income: u128 = 0;
        let mut total_employees: u64 = 0;

        let funded_businesses = &mut 0;

        for business in self.businesses.values_mut() {
            Business::check_funding(&self.rules.business_funding_rule, business, funded_businesses);
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

            total_business_income += business.last_month_income as u128;
            total_employees = business.employees.len() as u64;
        }

        self.business_data.average_monthly_income = (total_business_income / self.businesses.len() as u128) as i64;
        self.business_data.average_employees = (total_employees / self.businesses.len() as u64) as i32; 

        let mut remaining_market_percentage: f32 = 100.;
        let mut cost_per_percent = 0.;

        let purchase_rate = self.purchases as f32 / self.total_possible_purchases as f32;

        let mut demand: f32 = 0.;
        for person in self.people.values() { // TODO: do this for all product types
            demand += person.demand[&ProductType::Leisure];
        }

        let people_cpy = &mut self.people.clone();
        let mut unemployed_people: &mut Vec<&mut Person> = &mut people_cpy.values_mut().filter(|p| p.job == Job::Unemployed && p.age >= 18).collect();

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

            let business = self.businesses.get_mut(&bid).ok_or(
                Error::Danger("Could not get business from reinvestment budgets list.".to_string())
            )?;

            business.get_new_market(assigned_percent, cost_per_percent, &mut self.people, &mut unemployed_people, demand, purchase_rate)?;
            business.last_month_balance = business.balance;

            remaining_market_percentage -= assigned_percent;
        }

        // Sync people to unemployed people
        for per_cpy in unemployed_people {
            if per_cpy.job != Job::Unemployed {
                let per = self.people.get_mut(&per_cpy.id).unwrap();
                per.job = per_cpy.job.clone();
                per.set_salary(per_cpy.salary);
            }
        }

        for id in bus_removal_queue {
            self.businesses.remove(&id);
        }

        update_app(App::Finance, json!({
            "average_monthly_income": self.finance_data.average_monthly_income,
            "expected_person_income": self.finance_data.expected_person_income,
            "expected_business_income": self.finance_data.expected_business_income,
        }), app_handle);

        update_app(App::Healthcare, json!({
            "life_expectancy": self.healthcare.life_expectancy,
        }), app_handle);

        update_app(App::Business, json!({
            "average_employees": self.business_data.average_employees,
            "average_monthly_income": self.business_data.average_monthly_income,
        }), app_handle);

        self.government_balance -= self.welfare_owed + (*funded_businesses as i64 * self.rules.business_funding_rule.fund) + self.healthcare.budget;

        self.healthcare.month_unhospitalised_count = 0;
        self.total_possible_purchases = 0;
        self.purchases = 0;

        Ok(())
    }
}