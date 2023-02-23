use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use crate::{entities::{business::Business, person::person::Person}, common::util::{Date, SlotArray}};

use super::events::App;

#[derive(Default, Serialize, Deserialize)]
pub struct TaxRule {
    pub enabled: bool, 
    pub minimum_salary: i32,
    pub tax_rate: f32,
}

#[derive(Default, Serialize, Deserialize)]
pub struct BusinessTaxRule {
    pub enabled: bool,
    pub minimum_monthly_income: f64,
    pub tax_rate: f32,
}

#[derive(Default, Serialize, Deserialize)]
pub struct BusinessFundingRule {
  pub enabled: bool,
  pub fund: i64,
  pub maximum_income: i64,
  pub business_count: i32,
}

#[derive(Default, Serialize, Deserialize)]
pub struct DenyAgeRule {
  pub enabled: bool,
  pub maximum_age: i32,
}

#[derive(Default, Serialize, Deserialize)]
pub struct DenyHealthPercentageRule {
  pub enabled: bool,
  pub maximum_percentage: i32,
}

#[derive(Default, Serialize, Deserialize)]
pub struct CoverFoodRule {
  pub enabled: bool,
  pub people_count: i32,
  pub maximum_salary: i32,
  pub budget_cost: i64,
}

#[derive(Default, Serialize, Deserialize)]
pub struct CoverFoodUnemployedRule {
  pub enabled: bool,
  pub people_count: i32,
  pub budget_cost: i64,
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

#[derive(Default)]
pub struct FinanceData {
  pub average_monthly_income: i32,
  pub expected_person_income: i64,
  pub expected_business_income: i64,
}

#[derive(Default)]
pub struct BusinessData {
  pub average_employees: i32,
  pub average_monthly_income: i64,
}

pub struct GameState {
  pub tax_rate: f32,
  pub business_tax_rate: f32,
  pub businesses: HashMap<Uuid, Business>,
  pub people: HashMap<Uuid, Person>,
  pub date: Date,

  pub government_balance: i64, // This is expected to be quite large
  
  pub total_possible_purchases: u32,
  pub purchases: u32,

  pub rules: GameStateRules,
  pub open_apps: HashMap<App, bool>,

  pub births_in_last_month: SlotArray<i32>,
  pub deaths_in_last_month: SlotArray<usize>,
  pub healthcare: HealthcareState,

  pub finance_data: FinanceData,

  pub welfare_budget: i64,
  pub welfare_owed: i64,

  pub business_budget: i64,
  pub business_owed: i64,

  pub spare_budget: i64,

  pub average_welfare: f32,
  pub average_welfare_unemployed: f32,

  pub business_data: BusinessData,

  pub unemployed_count: i32,
}

#[derive(Default, Clone, Copy, Serialize, Deserialize)]
pub struct HealthcareGroup {
  pub budget: i64,
  pub current_capacity: i32,
  pub total_capacity: i32,
}

#[derive(Default)]
pub struct HealthcareState {
  pub cost_per_hospital_capacity: f32, // This is the cost per person capacity in a hospital for the government, each month
  pub month_unhospitalised_count: i32, // Number of patient that could not go to hospital because of the full capacity

  pub budget: i64,
  pub total_capacity: i32,

  pub childcare: HealthcareGroup,
  pub adultcare: HealthcareGroup,
  pub eldercare: HealthcareGroup,

  pub life_expectancy: i32,
  pub age_ranges: serde_json::Value,

  pub births_per_month: i32,
  pub deaths_per_month: i32,
}

impl HealthcareState {
  pub fn get_current_capacity(&self) -> i32 {
    self.total_capacity - (self.childcare.current_capacity - self.adultcare.current_capacity - self.eldercare.current_capacity)
  }
}