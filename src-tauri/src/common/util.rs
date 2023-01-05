use rand::Rng;
use std::collections::HashMap;

pub fn set_decimal_count(number: f32, decimal_count: u32) -> f32 {
    let power = 10i32.pow(decimal_count) as f32;
    (number * power).round() / power
}

pub fn generate_percentage() -> i32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(0..100)
}

// TODO: Use something other than a hashmap
/// Takes in an input of percentages and then an output is generated based on the chances provided
pub fn percentage_based_output_int<ValueType>(chances: HashMap<ValueType, i32>) -> ValueType {
    let mut remaining_percentage = 100;
    let percentage = generate_percentage();
    let mut ret_value: Option<ValueType> = None;

    let mut sorted_chances: Vec<_> = chances.into_iter().collect();
    sorted_chances.sort_by(|a, b| a.1.cmp(&b.1));

    for (value, chance) in sorted_chances {
        remaining_percentage -= chance;

        if ret_value.is_none() && percentage >= remaining_percentage {
            ret_value = Some(value);
        }
    }

    if remaining_percentage != 0 {
        panic!("Percentage output could not be calculated because the chances do not add up to exactly 100%.");
    }

    match ret_value {
        Some(v) => v,
        None => panic!("Invalid input provided. Percentages must add up to exactly 100%."),
    }
}

/// Random float range
pub fn float_range(min: f32, max: f32, decimal_count: u32) -> f32 {
    let mut rng = rand::thread_rng();
    set_decimal_count(rng.gen::<f32>() * (max - min) + min, decimal_count)
}