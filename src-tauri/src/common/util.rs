use rand::Rng;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, ops::{Index, IndexMut}};

use crate::{game::structs::{HealthcareGroup, HealthcareState}};

#[macro_export]
macro_rules! percentage_of {
    ($partial:expr; / $total:expr) => {
        (($partial as f32 / $total as f32) * 100.) as i32
    };
}

#[macro_export]
macro_rules! as_decimal_percent {
    ($x:expr) => {
        ($x as f32) / 100.
    };
}

pub fn set_decimal_count(number: f32, decimal_count: u32) -> f32 {
    let power = 10i32.pow(decimal_count) as f32;
    (number * power).round() / power
}

pub fn generate_percentage() -> i32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(0..100)
}

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

/// This isn't really a float percentage, it just rounds the float values to the nearest int value
pub fn percentage_based_output_float<ValueType>(chances: HashMap<ValueType, f32>) -> ValueType {
    let mut remaining_percentage = 100;
    let percentage = generate_percentage();
    let mut ret_value: Option<ValueType> = None;

    let mut sorted_chances: Vec<_> = chances.into_iter().collect();
    sorted_chances.sort_by(|a, b| a.1.total_cmp(&b.1));

    for (value, chance) in sorted_chances {
        let rounded_chance = chance.round() as i32;
        remaining_percentage -= rounded_chance;

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

#[derive(Clone, Serialize, Deserialize)]
pub struct Date {
    pub day: i32,
    pub month: i32,
    pub year: i32,
    pub on_new_month: bool,
}

impl Default for Date {
    fn default() -> Self {
        Self {
            day: 1,
            month: 1,
            year: 0,
            on_new_month: false,
        }
    }
}

impl Date {
    pub fn new(day: i32, month: i32, year: i32) -> Self {
        Self {
            day,
            month,
            year,
            on_new_month: false,
        }
    }

    pub fn is_eq(&self, date: Date) -> bool {
        if date.day == self.day && date.month == self.month && date.year == self.year {
            return true;
        }

        false
    }

    pub fn is_generation_day(&self) -> bool {
        self.day == 1 && self.month == 1 && self.year == 0
    }

    pub fn new_day(&mut self) {
        self.on_new_month = false;
        self.day += 1;
        if self.day == 31 {
            self.new_month();
            self.day = 1;
        }
    }

    pub fn new_month(&mut self) {
        self.month += 1;
        if self.month == 13 {
            self.year += 1;
            self.month = 1;
        }

        self.on_new_month = true;
    }

    pub fn get_date_string(&self) -> String {
        format!("{}/{}/{}", zerofy(self.day, 2), zerofy(self.month, 2), zerofy(self.year, 4))
    }
}

/// Adds a zero for every missing number of digits. \
/// For example, if the `expected_digits` is 3 and the number is 12, the string would evaluate to "012"
pub fn zerofy(val: i32, expected_digits: i32) -> String {
    let mut result = String::new();
    let missing_digits = expected_digits - length(val as u32);

    for _ in 0..missing_digits {
        result += "0";
    }

    result.push_str(val.to_string().as_str());
    result
}

pub fn length(n: u32) -> i32 {
    let mut power = 10;
    let mut count = 1;

    while n >= power {
        count += 1;
        if let Some(new_power) = power.checked_mul(10) {
            power = new_power;
        } else {
            break;
        }
    }

    count
}

pub fn percentage_chance(percent: f32) -> bool {
    let maximum = (100. / percent) as i32;
    rand::thread_rng().gen_range(0..=maximum) == maximum
}

pub fn chance_one_in(amount: i32) -> bool {
    rand::thread_rng().gen_range(0..=amount) == amount
}

#[derive(Default, Debug, Clone)]
pub struct SlotArray<T> {
    pub array: Vec<T>,
    current_idx: usize,
}

impl<T: Default + Clone> SlotArray<T> {
    pub fn new(size: usize) -> Self {
        Self {
            array: vec![T::default(); size],
            current_idx: 0,
        }
    }

    pub fn from(vector: Vec<T>) -> Self {
        Self {
            array: vector,
            current_idx: 0,
        }
    }

    pub fn push(&mut self, item: T) {
        self.array[self.current_idx] = item;

        self.current_idx += 1;
        if self.current_idx == self.array.capacity() {
            self.current_idx = 0;
        }
    }

    pub fn len(&self) -> usize { self.array.len() }
}

impl<T> Index<usize> for SlotArray<T> {
    type Output = T;
    fn index(&self, i: usize) -> &T {
        &self.array[i]
    }
}

impl<T> IndexMut<usize> for SlotArray<T> {
    fn index_mut(&mut self, i: usize) -> &mut T {
        &mut self.array[i]
    }
}

pub fn generate_unemployed_salary() -> i32 {
    rand::thread_rng().gen_range(300..=1100)    
}

pub fn get_healthcare_group(age: i32, healthcare: &mut HealthcareState) -> &mut HealthcareGroup {
    match age {
        age if age <= 18 => &mut healthcare.childcare,
        age if age <= 65 => &mut healthcare.adultcare,
        _ => &mut healthcare.eldercare,
    }
}