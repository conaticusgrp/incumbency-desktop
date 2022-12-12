use std::fs;

use serde::Deserialize;

const CONFIG_PATH: &str = "./game_config.toml";

#[derive(Deserialize)]
pub struct Config {
    pub starting_population: String,
} 

pub fn load_config() -> Config {
    let config_contents = fs::read_to_string(CONFIG_PATH).unwrap();
    toml::from_str(config_contents.as_str()).unwrap()
}