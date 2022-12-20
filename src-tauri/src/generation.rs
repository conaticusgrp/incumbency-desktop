use std::collections::HashMap;
use crate::{new_game::GameStateSafe, config::{load_config, Config}, entities::person::{Person, EducationLevel::{*, self}}, util::percentage_based_output_float};

pub fn generate_education_level(config: &Config) -> EducationLevel {
    let mut map = HashMap::new();

    map.insert(NoFormalEducation, config.no_education_chance);
    map.insert(HighSchoolDiploma, config.high_school_diploma_chance);
    map.insert(College, config.college_chance);
    map.insert(AssociateDegree, config.associate_degree_chance);
    map.insert(Bachelors, config.bachelors_chance);
    map.insert(AdvancedDegree, config.advanced_degree_chance);

    percentage_based_output_float::<EducationLevel>(map, 2)
}

pub fn generate_game(state_mux: &GameStateSafe) {
    let mut state = state_mux.lock().unwrap();
    let config = load_config();

    for _ in 0..config.starting_population {
        let mut person = Person::default();
        person.generate(&config);

        state.people.push(person);
    }
}