use std::fs;

use serde::Deserialize;

const CONFIG_PATH: &str = "./game_config.toml";

#[derive(Deserialize)]
pub struct Config {
    pub starting_population: i32,

    pub no_education_chance: f32,
    pub high_school_diploma_chance: f32,
    pub college_chance: f32,
    pub associate_degree_chance: f32,
    pub bachelors_chance: f32,
    pub advanced_degree_chance: f32,
}

pub fn load_config() -> Config {
    let config_contents = fs::read_to_string(CONFIG_PATH).unwrap();
    toml::from_str(config_contents.as_str()).unwrap()
}