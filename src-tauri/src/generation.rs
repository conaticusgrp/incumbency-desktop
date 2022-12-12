use crate::{new_game::GameStateSafe, util::load_config};

pub fn generate_game(state_mux: &GameStateSafe) {
    let mut state = state_mux.lock().unwrap();
    let config = load_config();
}