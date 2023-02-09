use std::fs;
use serde::Deserialize;

use super::errors::IncResult;

const CONFIG_PATH: &str = "./game_config.toml";

#[derive(Deserialize)]
pub struct Config {
    pub starting_population: i32,

    pub no_education: EducationConfig,
    pub high_school_diploma: EducationConfig,
    pub college: EducationConfig,
    pub associate_degree: EducationConfig,
    pub bachelors: EducationConfig,
    pub advanced_degree: EducationConfig,
}

#[derive(Deserialize)]
pub struct EducationConfig {
    pub chance: i32,
    pub salary_range: ConfigRange,
}

#[derive(Deserialize)]
pub struct ConfigRange {
    pub min: i32,
    pub max: i32,
}

pub fn load_config() -> IncResult<Config> {
    let config_contents = fs::read_to_string(CONFIG_PATH)?;
    Ok(toml::from_str(config_contents.as_str())?)
}