use super::{
    events::{json_get_i64, update_app, App, AppUpdateType},
    structs::{BusinessData, FinanceData, GameState, GameStateRules, HealthcareState},
};
use crate::{
    as_decimal_percent,
    common::{
        config::Config,
        errors::{Error, IncResult},
        util::{chance_one_in, generate_unemployed_salary, get_healthcare_group, Date, SlotArray},
    },
    entities::{
        business::{Business, ProductType},
        person::{
            debt::{Debt, DebtType},
            person::{Job, Person},
        },
    },
    game::events::{
        get_monthly_data,
        get_daily_data
    }
};
use serde_json::json;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};
use tauri::{AppHandle, Manager};
use uuid::Uuid;

const GOVERNMENT_START_BALANCE: u32 = 140000000;
const THREE_YEAR_DAYS: usize = 1080; // days in three game years
pub const EMPTY_DATA: i64 = -1;

pub type GameStateSafe = Arc<Mutex<GameState>>;

impl Default for GameState {
    fn default() -> Self {
        Self {
            tax_rate: 0.24,          // 24% default
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
            welfare_budget: 0,
            welfare_owed: 0,

            business_budget: 0,
            business_owed: 0,

            spare_budget: (GOVERNMENT_START_BALANCE as f64 * 0.) as i64,

            average_welfare: 100,
            average_welfare_unemployed: 100,

            business_data: BusinessData::default(),
            unemployed_count: 0,

            expected_balance: 0,

            // Daily updates

            population_graph_data: SlotArray::new_default(THREE_YEAR_DAYS, EMPTY_DATA),
            births_graph_data: SlotArray::new_default(THREE_YEAR_DAYS, EMPTY_DATA),
            deaths_graph_data: SlotArray::new_default(THREE_YEAR_DAYS, EMPTY_DATA),
            life_expectancy_graph_data: SlotArray::new_default(THREE_YEAR_DAYS, EMPTY_DATA),
            hospital_usage_capacity_graph_data: SlotArray::new_default(THREE_YEAR_DAYS, EMPTY_DATA),
            average_welfare_graph_data: SlotArray::new_default(THREE_YEAR_DAYS, EMPTY_DATA),
            average_unemployed_welfare_graph_data: SlotArray::new_default(THREE_YEAR_DAYS, EMPTY_DATA),
            government_balance_graph_data: SlotArray::new_default(THREE_YEAR_DAYS, EMPTY_DATA),
            government_balance_prediction_graph_data: SlotArray::new_default(THREE_YEAR_DAYS, EMPTY_DATA),

            // Monthly updates

            average_monthly_income_graph_data: SlotArray::new_default(THREE_YEAR_DAYS, EMPTY_DATA),
            government_losses_graph_data: SlotArray::new_default(THREE_YEAR_DAYS, EMPTY_DATA),
            business_count_graph_data: SlotArray::new_default(THREE_YEAR_DAYS, EMPTY_DATA),
            average_employees_graph_data: SlotArray::new_default(THREE_YEAR_DAYS, EMPTY_DATA),
            business_average_monthly_income_graph_data: SlotArray::new_default(THREE_YEAR_DAYS, EMPTY_DATA),
            unemployed_count_graph_data: SlotArray::new_default(THREE_YEAR_DAYS, EMPTY_DATA),
        }
    }
}

impl GameState {
    /// Resign from employed position, if the individual is employed
    pub fn resign_if_employed(&mut self, per: Person) {
        if let Job::Employee(bid) = per.job {
            let business = self.businesses.get_mut(&bid);

            if let Some(bus) = business {
                let employee_idx = bus
                    .employees
                    .iter()
                    .position(|emp_id| per.id == *emp_id)
                    .unwrap();
                bus.employees.remove(employee_idx);
            }
        }
    }

    pub fn check_healthcare_capacity(
        &self,
        new_total_capacity: i32,
        error_checker_failed: &mut bool,
    ) {
        let current_capacity = self.healthcare.get_current_capacity();

        if new_total_capacity >= current_capacity {
            return;
        }

        let lost_capacity = self.healthcare.total_capacity - new_total_capacity;
        let remaining_capacity =
            self.healthcare.total_capacity - self.healthcare.get_current_capacity();
        if remaining_capacity - lost_capacity < 0 {
            *error_checker_failed = true;
        }
    }

    pub fn get_spare_budget(&self) -> i64 {
        let spare_budget = self.government_balance
            - (self.healthcare.budget + self.welfare_budget + self.business_budget);
        if spare_budget < 0 {
            return 0;
        }

        spare_budget
    }

    pub fn day_pass(
        &mut self,
        day: i32,
        app_handle: Option<&tauri::AppHandle>,
        config: &Config,
    ) -> IncResult<()> {
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
            *self.healthcare.age_ranges.get_mut(key).ok_or_else(|| {
                Error::Danger(format!("Could not find age range '{}' in age ranges.", key))
            })? = json!(int64 + 1); // Increment counter of age ranges for state

            let die = per.day_pass(
                day,
                &mut self.healthcare,
                &date,
                &mut self.businesses,
                &mut self.purchases,
                &mut self.total_possible_purchases,
                &self.rules,
                &mut food_coverage,
                &mut unemployed_food_coverage,
            )?;

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
        self.finance_data.average_monthly_income =
            (total_monthly_income / self.people.len() as i64) as i32;

        for id in death_queue.iter() {
            let per = self.people.get_mut(id).ok_or(Error::DangerUnexpected)?;
            death_ages_total += per.age;

            let healthcare_group = get_healthcare_group(per.age, &mut self.healthcare);
            if let Some(_) = per.days_left_in_hospital {
                healthcare_group.current_capacity += 1;
            }

            let per_cpy = per.clone();
            self.resign_if_employed(per_cpy);
            self.people.remove(id);
        }

        if !death_queue.is_empty() {
            self.healthcare.life_expectancy = death_ages_total / death_queue.len() as i32;
        }

        self.deaths_in_last_month.push(death_queue.len());

        for _ in 0..new_birth_count {
            let infant =
                Person::new_infant(config, self.tax_rate, &self.rules.tax_rule, date.clone())?;
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

        self.average_welfare = (total_welfare as f32 / self.people.len() as f32) as i32;

        self.average_welfare_unemployed =
            (total_welfare_unemployed as f32 / self.unemployed_count as f32) as i32;

        self.expected_balance = (self.government_balance
            - (self.business_budget + self.welfare_budget + self.healthcare.budget))
            + (self.finance_data.expected_business_income
                + self.finance_data.expected_person_income);

        // Update graph data
        self.population_graph_data.push(self.people.len() as i64);
        self.births_graph_data.push(new_birth_count as i64);
        self.deaths_graph_data.push(death_queue.len() as i64);
        self.life_expectancy_graph_data.push(self.healthcare.life_expectancy as i64);
        self.hospital_usage_capacity_graph_data.push(self.healthcare.get_current_capacity() as i64);
        self.average_welfare_graph_data.push(self.average_welfare as i64);
        self.average_unemployed_welfare_graph_data.push(self.average_welfare_unemployed as i64);
        self.government_balance_graph_data.push(self.government_balance);
        self.government_balance_prediction_graph_data.push(self.expected_balance);
        self.average_monthly_income_graph_data.push(self.finance_data.average_monthly_income as i64);
        self.government_losses_graph_data.push(0);
        self.business_count_graph_data.push(self.businesses.len() as i64);
        self.average_employees_graph_data.push(self.business_data.average_employees as i64);
        self.business_average_monthly_income_graph_data.push(self.business_data.average_monthly_income);
        self.unemployed_count_graph_data.push(self.unemployed_count as i64);

        self.spare_budget = self.get_spare_budget();
        self.emit_daily_events(app_handle);

        Ok(())
    }

    pub fn emit_daily_events(&self, app_handle: Option<&AppHandle>) {
        if app_handle.is_none() {
            return;
        }

        let app_handle = app_handle.unwrap();

        update_app(
            App::Finance,
            json!({
                "government_balance": self.government_balance,
                "spare_budget": self.spare_budget,
                "used_hospital_capacity": self.healthcare.get_current_capacity(),
                "average_welfare": self.average_welfare,
                "average_welfare_unemployed": self.average_welfare_unemployed,
                "used_business_budget": (self.rules.business_funding_rule.fund * self.rules.business_funding_rule.business_count as i64),
                "used_welfare_budget": ((self.rules.cover_food_rule.people_count * 4) + (self.rules.cover_food_unemployed_rule.people_count * 4)) as i64,
                "spare_hospital_capacity": (self.healthcare.total_capacity - (self.healthcare.childcare.total_capacity + self.healthcare.adultcare.total_capacity + self.healthcare.eldercare.total_capacity)),
                "expected_balance": self.expected_balance,
            }),
            app_handle,
            AppUpdateType::Day,
        );

        update_app(
            App::Healthcare,
            json!({
                "population": self.people.len() as i32,
                "used_capacity": self.healthcare.get_current_capacity(),
                "total_capacity": self.healthcare.total_capacity,
                "age_ranges": self.healthcare.age_ranges,
                "child_care": self.healthcare.childcare,
                "adult_care": self.healthcare.adultcare,
                "elder_care": self.healthcare.eldercare,
                "births_per_month": self.healthcare.births_per_month,
                "deaths_per_month": self.healthcare.deaths_per_month,
                "population_graph_data": get_daily_data(&self.population_graph_data),
                "births_graph_data": get_daily_data(&self.births_graph_data),
                "deaths_graph_data": get_daily_data(&self.deaths_graph_data),
                "life_expectancy_graph_data": get_daily_data(&self.life_expectancy_graph_data),
                "hospital_usage_capacity_graph_data": get_daily_data(&self.hospital_usage_capacity_graph_data),
            }),
            app_handle,
            AppUpdateType::Day,
        );

        update_app(
            App::Welfare,
            json!({
                "average_welfare": self.average_welfare,
                "average_unemployed_welfare": self.average_welfare_unemployed,
                "welfare_budget": self.welfare_budget,
                "unemployed_count": self.unemployed_count,
                "average_welfare_graph_data": get_daily_data(&self.average_welfare_graph_data),
                "average_unemployed_welfare_graph_data": get_daily_data(&self.average_unemployed_welfare_graph_data),
            }),
            app_handle,
            AppUpdateType::Day,
        );

        update_app(
            App::Business,
            json!({
                "business_count": self.businesses.len() as i32,
                "business_budget": self.business_budget,
            }),
            app_handle,
            AppUpdateType::Day,
        );

        // app_handle.emit_all("debug_payload",  json! ({
        //     "Population": self.people.len(),
        //     "Average Welfare": self.average_welfare,
        //     "Government Balance": self.government_balance,
        //     "Monthly Births": self.healthcare.births_per_month,
        //     "Monthly Deaths": self.healthcare.deaths_per_month,
        //     "Unemployed Count": self.unemployed_count,
        // })).unwrap();
    }

    pub fn month_pass(&mut self, app_handle: &AppHandle, config: &Config) -> IncResult<()> {
        self.finance_data.expected_person_income = 0;

        for person in self.people.values_mut() {
            person.business_this_month = None;

            let tax_rate = Person::get_tax_rate(&self.rules.tax_rule, self.tax_rate, person.salary);
            person.calculate_demand(person.salary, None, tax_rate)?;

            match person.job {
                Job::BusinessOwner(bid) | Job::Employee(bid) => {
                    let business = self.businesses.get_mut(&bid);
                    if let Some(business) = business {
                        let tax_payment = (person.salary as f32 / 12.) * tax_rate;
                        self.finance_data.expected_person_income += tax_payment as i64;

                        person.pay_tax(
                            &mut self.government_balance,
                            (person.salary as f32 / 12.) * tax_rate,
                        );
                        person.business_pay(business, business.employee_salary as f64 / 12.);

                        if person.age >= 65 && chance_one_in(60) {
                            // Retired
                            person.set_salary(15800); // TODO: vary pension salary
                            person.job = Job::Retired;

                            let emp_idx = business.employees.iter().position(|&id| id == person.id);
                            if let Some(idx) = emp_idx {
                                business.employees.remove(idx);
                            }
                        }
                    } else {
                        person.job = Job::Unemployed;
                        person.set_salary(generate_unemployed_salary());
                    }

                    // business.pay_owner(person);
                }

                _ => (),
            };

            for i in 0..person.debts.len() {
                // TODO: Add functionality based on spending behaviour
                if !Debt::required_to_pay(person) {
                    break;
                }

                let debt = &mut person.debts[i];

                if debt.debt_type == DebtType::Education
                    && person.age < (18 + person.years_in_higher_education)
                {
                    continue;
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

        // 0.5% or above unemployed - generate new businesses
        let portion = self.people.len() as f32 * 0.005;
        let required_new_businesses = (self.unemployed_count as f32 / portion) as usize;

        if required_new_businesses != 0 {
            let owners: Vec<_> = self
                .people
                .values_mut()
                .filter(|p| p.balance >= 15000. && p.age >= 18)
                .take(required_new_businesses)
                .collect();
            for bus_owner in owners {
                let mut business = Business::default();
                let start_balance = bus_owner.balance * 0.45; // TODO: improve me

                business.generate_midgame(ProductType::Leisure, config, start_balance as f64); // TODO: support multiple product types

                business.owner_id = bus_owner.id;

                // TODO: resign from old job

                // let owner_cpy = bus_owner.clone();

                // self.resign_if_employed(owner_cpy);

                // bus_owner.set_salary(0);
                // bus_owner.job = Job::BusinessOwner(business.id);

                self.businesses.insert(business.id, business);
            }
        }

        for business in self.businesses.values_mut() {
            Business::check_funding(
                &self.rules.business_funding_rule,
                business,
                funded_businesses,
            );
            business.last_month_income = business.balance - business.last_month_balance;

            // TODO: make me more varied
            if business.balance <= 0. {
                bus_removal_queue.push(business.id);
            }

            let tax_rate = Business::get_tax_rate(
                &self.rules.business_tax_rule,
                business.last_month_income,
                self.business_tax_rate,
            );

            if business.last_month_income > 0. {
                let tax_cost = business.last_month_income * tax_rate as f64;
                self.finance_data.expected_business_income += tax_cost as i64;
            }

            business.pay_tax(
                &mut self.government_balance,
                business.last_month_income * tax_rate as f64,
            );
            let reinvesment_budget =
                business.balance * as_decimal_percent!(business.marketing_cost_percentage) as f64;

            if reinvesment_budget > 0. {
                total_reinvestment_budget += reinvesment_budget;
                reinvestment_budgets.push((business.id, reinvesment_budget));
            }

            total_business_income += business.last_month_income as u128;
            total_employees += business.employees.len() as u64;
        }

        self.business_data.average_monthly_income =
            (total_business_income / self.businesses.len() as u128) as i64;
        self.business_data.average_employees =
            (total_employees / self.businesses.len() as u64) as i32;

        let mut remaining_market_percentage: f32 = 100.;
        let mut cost_per_percent = 0.;

        let purchase_rate = self.purchases as f32 / self.total_possible_purchases as f32;

        let mut demand: f32 = 0.;
        for person in self.people.values() {
            // TODO: do this for all product types
            demand += person.demand[&ProductType::Leisure];
        }

        let people_cpy = &mut self.people.clone();
        let unemployed_people: &mut Vec<&mut Person> = &mut people_cpy
            .values_mut()
            .filter(|p| p.job == Job::Unemployed && p.age >= 18)
            .collect();

        for (i, (bid, budget)) in reinvestment_budgets.iter().enumerate() {
            let maximum_percentage = (budget / total_reinvestment_budget) * 100.;

            if i == 0 {
                cost_per_percent = (budget / maximum_percentage) as f32;
            }

            let mut assigned_percent = (budget / cost_per_percent as f64) as f32;

            if (remaining_market_percentage - assigned_percent) < 0. {
                assigned_percent = remaining_market_percentage;
            }

            let business = self.businesses.get_mut(bid).ok_or_else(|| {
                Error::Danger("Could not get business from reinvestment budgets list.".to_string())
            })?;

            business.get_new_market(
                assigned_percent,
                cost_per_percent,
                &mut self.people,
                unemployed_people,
                demand,
                purchase_rate,
            )?;
            business.last_month_balance = business.balance;

            remaining_market_percentage -= assigned_percent;
        }

        self.rules.business_funding_rule.budget_cost =
            self.rules.business_funding_rule.fund * (self.businesses.len() as i64);

        let mut unemployed_count = 0;

        // Sync people to unemployed people
        for per_cpy in unemployed_people {
            if per_cpy.job != Job::Unemployed {
                let per = self.people.get_mut(&per_cpy.id).unwrap();
                per.job = per_cpy.job.clone();
                per.set_salary(per_cpy.salary);
            } else {
                unemployed_count += 1;
            }
        }

        for id in bus_removal_queue {
            self.businesses.remove(&id);
        }

        let percentage = (unemployed_count as f32 / self.people.len() as f32) * 100.;

        if percentage >= 4. {
            app_handle.emit_all("unemployed_high", json!({ "unemployed_count": unemployed_count, "percent": percentage, "severity": "mild" })).unwrap();
        } else if percentage >= 1. {
            app_handle.emit_all("unemployed_high", json!({ "unemployed_count": unemployed_count, "percent": percentage, "severity": "high" })).unwrap();
        }

        let losses = self.welfare_owed
            + (*funded_businesses as i64 * self.rules.business_funding_rule.fund)
            + self.healthcare.budget;

        self.government_balance -= losses;

        // Update graph data

        self.average_monthly_income_graph_data.push(self.finance_data.average_monthly_income as i64);
        self.government_losses_graph_data.push(losses);
        self.business_count_graph_data.push(self.businesses.len() as i64);
        self.average_employees_graph_data.push(self.business_data.average_employees as i64);
        self.business_average_monthly_income_graph_data.push(self.business_data.average_monthly_income);
        self.unemployed_count_graph_data.push(self.unemployed_count as i64);
        
        update_app(
            App::Finance,
            json!({
                "average_monthly_income": self.finance_data.average_monthly_income,
                "expected_person_income": self.finance_data.expected_person_income,
                "expected_business_income": self.finance_data.expected_business_income,
                "welfare_budget": self.welfare_budget,
                "business_budget": self.business_budget,
                "healthcare_budget": self.healthcare.budget,
                "expected_balance": self.expected_balance,
                "government_balance_graph_data": get_monthly_data(&self.government_balance_graph_data, false),
                "government_balance_prediction_graph_data": get_monthly_data(&self.government_balance_prediction_graph_data, false),
                "average_monthly_income_graph_data": get_monthly_data(&self.average_monthly_income_graph_data, false),
                "government_losses_graph_data": get_monthly_data(&self.government_losses_graph_data, false),
            }),
            app_handle,
            AppUpdateType::Month,
        );

        update_app(
            App::Healthcare,
            json!({
                "life_expectancy": self.healthcare.life_expectancy,
            }),
            app_handle,
            AppUpdateType::Month,
        );

        update_app(
            App::Welfare,
            json!({
                "unemployed_count_graph_data": get_monthly_data(&self.unemployed_count_graph_data, false),
            }),
            app_handle,
            AppUpdateType::Month,
        );

        update_app(
            App::Business,
            json!({
                "average_employees": self.business_data.average_employees,
                "average_monthly_income": self.business_data.average_monthly_income,
                "business_count_graph_data": get_monthly_data(&self.business_count_graph_data, false),
                "average_employees_graph_data": get_monthly_data(&self.average_employees_graph_data, false),
                "average_monthly_income_graph_data": get_monthly_data(&self.average_monthly_income_graph_data, false),
            }),
            app_handle,
            AppUpdateType::Month,
        );

        self.healthcare.month_unhospitalised_count = 0;
        self.total_possible_purchases = 0;
        self.purchases = 0;

        Ok(())
    }
}
