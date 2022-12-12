use std::{fs};
use rand::Rng;
use serde::Deserialize;
use std::collections::HashMap;

const CONFIG_PATH: &str = "./game_config.toml";

#[derive(Deserialize)]
pub struct Config {
    pub starting_population: String,
} 

pub fn load_config() -> Config {
    let config_contents = fs::read_to_string(CONFIG_PATH).unwrap();
    toml::from_str(config_contents.as_str()).unwrap()
}

pub fn set_demical_count(number: f32, decimal_count: u32) -> f32 {
    let power = 10i32.pow(decimal_count) as f32;
    (number * power).round() / power
}

fn generate_percentage_float(decimal_count: u32) -> f32 {
    let mut rng = rand::thread_rng();
    set_demical_count(rng.gen::<f32>() * 100., decimal_count)
}

fn generate_percentage() -> i32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(0..100)
}

// Takes in an input of percentages and then an output is generated based on the chances provided
pub fn percentage_based_output_int<ValueType>(chances: HashMap<ValueType, i32>) -> Result<ValueType, String> {
    let mut remaining_percentage = 100;
    let percentage = generate_percentage();
    let mut ret_value: Option<ValueType> = None;

    for (value, chance) in chances {
        remaining_percentage -= chance;
        if percentage >= remaining_percentage {
            ret_value = Some(value);
        }
    }

    if remaining_percentage != 0 {
        return Err("Percentage output could not be calculated because the chances do not add up to exactly 100%.".to_string());
    }

    match ret_value {
        Some(v) => Ok(v),
        None => Err("Invalid input provided. Percentages must add up to exactly 100%.".to_string())
    }
}

pub fn percentage_based_output_float<ValueType>(chances: HashMap<ValueType, f32>, decimal_count: u32) -> Result<ValueType, String> {
    let mut remaining_percentage = 100.;
    let percentage = generate_percentage_float(decimal_count);
    let mut ret_value: Option<ValueType> = None;

    for (value, chance) in chances {
        remaining_percentage -= chance;
        if percentage >= remaining_percentage {
            ret_value = Some(value);
        }
    }

    if remaining_percentage != 0. {
        return Err("Percentage output could not be calculated because the chances do not add up to exactly 100.".to_string());
    }

    match ret_value {
        Some(v) => Ok(v),
        None => Err("Invalid input provided. Percentages must add up to exactly 100%.".to_string())
    }
}