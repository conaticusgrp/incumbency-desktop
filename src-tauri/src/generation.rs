use crate::{new_game::GameStateSafe, config::load_config, entities::person::Person};

pub fn generate_game(state_mux: &GameStateSafe) {
    let mut state = state_mux.lock().unwrap();
    let config = load_config();

    for _ in 0..config.starting_population {
        let mut person = Person::default();
        person.generate(&config);

        state.people.push(person);
    }
}