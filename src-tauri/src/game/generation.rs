use std::{collections::{HashMap}, ops::Range};
use maplit::hashmap;
use rand::Rng;

use crate::{common::{config::{Config}, util::percentage_based_output_int}, entities::{person::person::{EducationLevel::{*, self}, Person, Job}, business::{ProductType, Business}}};

use super::state_manager::GameStateSafe;

pub fn generate_education_level(config: &Config) -> EducationLevel {
    percentage_based_output_int::<EducationLevel>(hashmap! {
        NoFormalEducation => (config.no_education.chance),
        HighSchoolDiploma => (config.high_school_diploma.chance),
        College => (config.college.chance),
        AssociateDegree => (config.associate_degree.chance),
        Bachelors => (config.bachelors.chance),
        AdvancedDegree => (config.advanced_degree.chance),
    })
}

pub fn get_expected_salary_range(config: &Config, education_level: &EducationLevel) -> Range<i32> {
    match education_level {
        NoFormalEducation => config.no_education.salary_range.min..config.no_education.salary_range.max,
        HighSchoolDiploma => config.high_school_diploma.salary_range.min..config.high_school_diploma.salary_range.max,
        College => config.college.salary_range.min..config.college.salary_range.max,
        AssociateDegree => config.associate_degree.salary_range.min..config.associate_degree.salary_range.max,
        Bachelors => config.bachelors.salary_range.min..config.bachelors.salary_range.max,
        AdvancedDegree => config.bachelors.salary_range.min..config.bachelors.salary_range.max,
    }
}

pub fn generate_game(state_mux: &GameStateSafe, config: &Config) {
    let mut state = state_mux.lock().unwrap();

    let mut product_demand: HashMap<ProductType, f32> = HashMap::new();
    product_demand.insert(ProductType::Leisure, 0.);

    for _ in 0..config.starting_population {
        let person = Person::new_generate(&config, &mut product_demand, state.tax_rate);
        state.people.insert(person.id, person);
    }

    let mut remaning_market_percentage: f32 = 100.;

    loop {
        let mut business = Business::default();
        
        let idx = state.businesses.len();

        let tax_rate = state.tax_rate;

        let sufficient_businesses = business.generate(&config, ProductType::Leisure, product_demand[&ProductType::Leisure], &mut remaning_market_percentage, &mut state.people, idx, tax_rate);
        let owner = Person { job: Job::BusinessOwner(business.id), age: rand::thread_rng().gen_range(20..70), ..Person::new_generate(&config, &mut product_demand, tax_rate) };
        business.owner_id = owner.id;

        state.people.insert(owner.id, owner);
        state.businesses.insert(business.id, business);

        if sufficient_businesses {
            break;
        }
    }

    // This of course cannot be calculated until after the businesses are generated
    for per in state.people.values_mut() {
        per.generate_daily_food_spending();
    }

    state.population_counter = state.people.len() as f64;
}

/// Runs 1 month of the game to prepare the economy and get all the required values
pub fn stabilize_game(state_mux: &GameStateSafe, config: &Config) {
    let mut state = state_mux.lock().unwrap();
    for day in 1..=30 {
        state.day_pass(day, None, config);
    }

    state.cost_per_hospital_capacity = state.healthcare_investment / state.month_unhospitalised_count as f64;

    let starting_investment = state.cost_per_hospital_capacity * state.month_unhospitalised_count as f64;
    state.set_healthcare_investment(starting_investment);

    let tax_rate = state.tax_rate;
    state.month_pass(tax_rate);
}