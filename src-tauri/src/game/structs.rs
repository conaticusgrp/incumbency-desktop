use std::collections::HashMap;
use uuid::Uuid;
use crate::{entities::{business::Business, person::person::Person}, common::util::{Date, SlotArray}};

use super::events::App;

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
pub struct BusinessFundingRule {
  pub enabled: bool,
  pub fund: f64,
  pub maximum_income: f64,
}

#[derive(Default)]
pub struct DenyAgeRule {
  pub enabled: bool,
  pub maximum_age: i32,
}

#[derive(Default)]
pub struct DenyHealthPercentageRule {
  pub enabled: bool,
  pub maximum_percentage: i32,
}

#[derive(Default)]
pub struct CoverFoodRule {
  pub enabled: bool,
  pub people_count: i32,
  pub maxmimum_salary: f32,
}

#[derive(Default)]
pub struct CoverFoodUnemployedRule {
  pub enabled: bool,
  pub people_count: i32,
}

#[derive(Default)]
pub struct GameStateRules {
  pub tax_rule: TaxRule,
  pub business_tax_rule: BusinessTaxRule,
  pub business_funding_rule: BusinessFundingRule,
  pub deny_age_rule: DenyAgeRule,
  pub deny_health_percentage_rule: DenyHealthPercentageRule,
  pub cover_food_rule: CoverFoodRule,
  pub cover_food_unemployed_rule: CoverFoodUnemployedRule,
}

pub struct GameState {
  pub tax_rate: f32,
  pub business_tax_rate: f32,
  pub businesses: HashMap<Uuid, Business>,
  pub people: HashMap<Uuid, Person>,
  pub date: Date,

  pub government_balance: i64, // This is expected to be quite large
  pub population_counter: f64,

  pub births_in_last_month: SlotArray<i32>,
  pub deaths_in_last_month: SlotArray<usize>,

  pub total_possible_purchases: u32,
  pub purchases: u32,

  pub rules: GameStateRules,
  pub open_apps: HashMap<App, bool>,

  pub healthcare: HealthcareState,
}

#[derive(Default, Clone, Copy)]
pub struct HealthcareGroup {
  pub budget: f32,
  pub current_capacity: i32,
  pub total_capacity: i32,
}

#[derive(Default)]
pub struct HealthcareState {
  pub cost_per_hospital_capacity: f32, // This is the cost per person capacity in a hospital for the government, each month
  pub month_unhospitalised_count: i32, // Number of patient that could not go to hospital because of the full capacity

  pub budget: f32,
  pub total_capacity: i32,

  pub childcare: HealthcareGroup,
  pub adultcare: HealthcareGroup,
  pub eldercare: HealthcareGroup,
}

impl HealthcareState {
  pub fn get_current_capacity(&self) -> i32 {
    self.childcare.current_capacity + self.adultcare.current_capacity + self.eldercare.current_capacity
  }
}