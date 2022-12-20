use std::collections::HashMap;
use maplit::hashmap;

use crate::{new_game::GameStateSafe, config::{load_config, Config}, entities::{person::{Person, EducationLevel::{*, self}}, business::{Business, ProductType}}, util::percentage_based_output_int};

pub fn generate_education_level(config: &Config) -> EducationLevel {
    percentage_based_output_int::<EducationLevel>(hashmap! {
        NoFormalEducation => (config.no_education_chance),
        HighSchoolDiploma => (config.high_school_diploma_chance),
        College => (config.college_chance),
        AssociateDegree => (config.associate_degree_chance),
        Bachelors => (config.bachelors_chance),
        AdvancedDegree => (config.advanced_degree_chance),
    })
}

pub fn generate_game(state_mux: &GameStateSafe) {
    let mut state = state_mux.lock().unwrap();
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
        let finished = business.generate(&config, ProductType::LEISURE, product_demand[&ProductType::LEISURE], &mut remaning_market_percentage);

        if finished {
            break;
        }
    }
}