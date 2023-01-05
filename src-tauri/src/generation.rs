use std::{collections::{HashMap}, ops::Range};
use maplit::hashmap;

use crate::{game::GameStateSafe, config::{load_config, Config}, entities::{person::{Person, EducationLevel::{*, self}}, business::{Business, ProductType}}, util::percentage_based_output_int};

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

pub async fn generate_game(state_mux: &GameStateSafe) {
    let mut state = state_mux.lock().await;
    let config = load_config();

    let mut product_demand: HashMap<ProductType, f32> = HashMap::new();
    product_demand.insert(ProductType::LEISURE, 0.);

    for _ in 0..config.starting_population {
        let mut person = Person::default();
        person.generate(&config, &mut product_demand);

        state.people.push(person);
    }

    let mut remaning_market_percentage: f32 = 100.;

    loop {
        let mut business = Business::default();
        let idx = state.businesses.len();

        let finished = business.generate(&config, ProductType::LEISURE, product_demand[&ProductType::LEISURE], &mut remaning_market_percentage, &mut state.people, idx);

        state.businesses.push(business);

        if finished {
            break;
        }
    }
}