use std::collections::HashMap;
use uuid::Uuid;
use crate::{entities::{business::Business, person::person::Person}, common::util::{Date, SlotArray}};

#[derive(Default)]
pub struct TaxRule {
    pub enabled: bool, 
    pub minimum_salary: i32,
    pub tax_rate: f32,
}

#[derive(Default)]
pub struct BusinessTaxRule {
    pub enabled: bool,
    pub minimum_monthly_income: f64,
    pub tax_rate: f32,
}

#[derive(Default)]
pub struct GameStateRules {
  pub tax_rule: TaxRule,
  pub busines_tax_rule: BusinessTaxRule,
}

pub struct GameState {
  pub tax_rate: f32,
  pub business_tax_rate: f32,
  pub businesses: HashMap<Uuid, Business>,
  pub people: HashMap<Uuid, Person>,
  pub date: Date,

  pub government_balance: i64, // This is expected to be quite large
  pub healthcare_investment: f64,

  pub hospital_total_capacity: i32,
  pub hospital_current_capacity: i32,
  pub cost_per_hospital_capacity: f64, // This is the cost per person capacity in a hospital for the government, each month
  pub month_unhospitalised_count: i32, // Number of patient that could not go to hospital because of the full capacity

  pub population_counter: f64,

  pub births_in_last_month: SlotArray<i32>,
  pub deaths_in_last_month: SlotArray<usize>,

  pub total_possible_purchases: u32,
  pub purchases: u32,

  pub rules: GameStateRules,
}