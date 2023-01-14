use maplit::hashmap;
use rand::Rng;
use crate::{common::util::percentage_based_output_int};
use super::person::Person;

impl Person {
    pub fn add_health(&mut self, amount: i32) {
        self.health_percentage += amount;
    }

    /// Get percentage chance of death based on current health percentage
    pub fn get_death_chance(&self) -> i32 {
        match self.health_percentage {
            h if h >= 40 => 0,
            h if h >= 30 => 5,
            h if h >= 20 => 20,
            h if h >= 10 => 30,
            h if h >= 2 => 45,
            _ => 100, // if health percentage is 2 or below, death is iminent
        }
    }

    /// Change an additional chance of death based on hospital capacity - the predetermined death chance must be multiplyed by this multiplyer
    fn multiplyer_based_on_capacity(&self, capacity: i32) -> f32 {
        match capacity {
            0 => 4.,
            c if c <= 3 => 2.,
            c if c <= 7 => 1.5,
            c if c <= 15 => 1.2,
            _ => 1.,
        }
    }

    /// Either a positive or negative amount can be entered here.
    pub fn remove_health(&mut self, amount: i32, hospital_capacity: &mut i32, month_unhospitalised_count: &mut i32) {
        self.health_percentage -= amount;
        if self.health_percentage <= 2 {
            self.die(0);
            return;
        }

        let percentage_below_hospitalisation = -(self.health_percentage - self.hospitalisation_count);
        if percentage_below_hospitalisation >= 0 {

            let death_chance = self.get_death_chance();
            let predetermined_health_factor = if self.hospitalisation_percentage > 20 { percentage_below_hospitalisation } else { 0 };
            
            let mut chance_of_death = death_chance + predetermined_health_factor; // death chance is higher based on age and capacity for new patients
            chance_of_death = (chance_of_death as f32 * self.multiplyer_based_on_capacity(*hospital_capacity)) as i32;

            if chance_of_death > 100 { chance_of_death = 100 }

            if *hospital_capacity == 0 {
                *month_unhospitalised_count += 1;
                self.die_based_on_chance(chance_of_death, 0); // will die for the proceeding day - TODO: die on the current day instead, if possible
                return;
            }

            *hospital_capacity -= 1;
            self.hospitalize(percentage_below_hospitalisation, death_chance);
        }
    }

    pub fn hospitalize(&mut self, percentage_below_hospitalisation: i32, death_chance: i32) {
        self.hospitalisation_count += 1;

        let mut rng = rand::thread_rng();
        let increase_percent = rng.gen_range(0..=1) == 1;

        let hospitalisation_percent_increase = match percentage_below_hospitalisation {
            p if p <= 8 => if increase_percent { 1 } else { 0 },
            p if p <= 15 => 3,
            p if p <= 25 => 5,
            _ => 8,
        };

        self.hospitalisation_percentage += hospitalisation_percent_increase;

        self.days_left_in_hospital = Some(death_chance / 2);
        self.die_based_on_chance(death_chance, rng.gen_range(0..=death_chance / 2));
    }

    /// If chance is met, kill the current person
    fn die_based_on_chance(&mut self, chance: i32, days_until_death: i32) {
            let die = percentage_based_output_int(hashmap! {
                true => chance,
                false => 100 - chance,
            });

            if die {
                self.die(days_until_death); // massive L, the person will automatically die the following day
                return;
            }
    }

    pub fn die(&mut self, days_until_death: i32) {
        self.days_until_death = Some(days_until_death);
        // TODO: inherit family
    }

    pub fn generate_health(&mut self) {
        let (health_range, hospitalisation_percentage_range, hospitalisation_count_range) = match self.age {
            a if a <= 20 => (75..95, 8..15, 0..3),
            a if a <= 35 => (65..85, 12..20, 1..6),
            a if a <= 55 => (55..80, 15..25, 1..12),
            a if a <= 75 => (30..55, 20..40, 5..25),
            _ => (20..30, 25..50, 5..25) // TODO: Allow people to live older
        };

        let mut rng = rand::thread_rng(); 

        self.health_percentage = rng.gen_range(health_range);
        self.hospitalisation_percentage = rng.gen_range(hospitalisation_percentage_range);
        self.hospitalisation_count = rng.gen_range(hospitalisation_count_range);
    }

    pub fn grow_up(&mut self) {
        self.age += 1;
        if self.age % 2 == 0 {
            self.hospitalisation_percentage += 1;
        }
    }

    // This should be executed everyday
    pub fn replenish_health(&mut self) {
        let replenish = rand::thread_rng().gen_range(1..=30) == 30;

        if replenish {
            self.health_percentage += 1;
        }
    }
}