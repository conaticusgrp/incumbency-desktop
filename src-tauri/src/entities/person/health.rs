use super::{debt::Debt, person::Person};
use crate::{
    common::util::{get_healthcare_group, percentage_based_output_int},
    game::structs::{GameStateRules, HealthcareState},
};
use maplit::hashmap;
use rand::Rng;

impl Person {
    pub fn add_health(&mut self, amount: i32) {
        if self.health_percentage + amount > self.maximum_health {
            self.health_percentage = self.maximum_health;
            return;
        }

        self.health_percentage += amount;
    }

    /// Get percentage chance of death based on current health percentage
    pub fn get_death_chance(&self) -> i32 {
        match self.health_percentage {
            h if h <= 3 => 45,
            h if h <= 5 => 15,
            h if h <= 10 => 8,
            h if h <= 20 => 4,
            h if h <= 30 => 1,
            h if h <= 40 => 0,
            _ => 100, // if health percentage is 3 or below, death is iminent
        }
    }

    /// Change an additional chance of death based on hospital capacity - the predetermined death chance must be multiplyed by this multiplyer
    fn multiplyer_based_on_capacity(&self, capacity_percentage: f32) -> f32 {
        match capacity_percentage {
            c if c <= 0. => 5.,
            c if c <= 5. => 2.5,
            c if c <= 10. => 1.8,
            c if c <= 25. => 1.5,
            c if c >= 30. => 1.2,
            _ => 1.,
        }
    }

    pub fn eligible_for_healthcare(&self, rules: &GameStateRules) -> bool {
        if rules.deny_age_rule.enabled && self.age > rules.deny_age_rule.maximum_age {
            return false;
        }

        if rules.deny_health_percentage_rule.enabled
            && self.health_percentage > rules.deny_health_percentage_rule.maximum_percentage
        {
            return false;
        }

        true
    }

    pub fn remove_health(
        &mut self,
        amount: i32,
        healthcare: &mut HealthcareState,
        rules: &GameStateRules,
    ) {
        self.health_percentage -= amount;
        if self.health_percentage <= 2 {
            self.die(0);
            return;
        }

        let percentage_below_hospitalisation =
            -(self.health_percentage - self.hospitalisation_percentage);
        if percentage_below_hospitalisation < 0 {
            return;
        }

        let mut death_chance = self.get_death_chance();
        let predetermined_health_factor = if self.hospitalisation_percentage < 35 {
            (percentage_below_hospitalisation / 2) + (self.hospitalisation_percentage / 10)
        } else {
            0
        };

        let healthcare_group = get_healthcare_group(self.age, healthcare);

        death_chance += predetermined_health_factor; // death chance is higher based on age and capacity for new patients

        let mut group_capacity_percentage = (healthcare_group.current_capacity as f32
            / healthcare_group.total_capacity as f32)
            * 100.;
        if group_capacity_percentage.is_nan() {
            group_capacity_percentage = 0.;
        }

        death_chance = (death_chance as f32
            * self.multiplyer_based_on_capacity(group_capacity_percentage))
            as i32;

        if death_chance > 100 {
            death_chance = 100
        }

        if healthcare_group.current_capacity == 0 || !self.eligible_for_healthcare(rules) {
            healthcare.month_unhospitalised_count += 1;
            self.die_based_on_chance(death_chance * 3, 0); // will die for the proceeding day - TODO: die on the current day instead, if possible
            return;
        }

        healthcare_group.current_capacity -= 1;
        self.hospitalize(percentage_below_hospitalisation, death_chance, amount);
    }

    pub fn hospitalize(
        &mut self,
        percentage_below_hospitalisation: i32,
        death_chance: i32,
        initial_health_loss: i32,
    ) {
        self.hospitalisation_count += 1;
        let mut rng = rand::thread_rng();

        let hospital_days = death_chance;
        self.days_left_in_hospital = Some(hospital_days);
        self.hospitalised_age = self.age;

        let increase_percent = rng.gen_range(0..=1) == 1;

        let hospitalisation_percent_increase = match percentage_below_hospitalisation {
            p if p <= 8 => i32::from(increase_percent),
            p if p <= 15 => 3,
            p if p <= 25 => 5,
            _ => 8,
        };

        self.hospitalisation_percentage += hospitalisation_percent_increase;

        self.die_based_on_chance(death_chance, hospital_days); // i swear my code gets worse every time i do this

        self.health_percentage = self.hospitalisation_percentage + (initial_health_loss / 2);
        if self.health_percentage > self.maximum_health {
            self.health_percentage = self.maximum_health
        }
    }

    fn die_based_on_chance(&mut self, chance: i32, days_until_death: i32) {
        let die = percentage_based_output_int(hashmap! {
            true => chance,
            false => 100 - chance,
        });

        if die {
            self.die(days_until_death); // massive L, the person will automatically die the following day
        }
    }

    pub fn die(&mut self, days_until_death: i32) {
        self.days_until_death = Some(days_until_death);
        // TODO: inheritance of money
    }

    pub fn generate_health(&mut self) {
        let (
            health_range,
            hospitalisation_percentage_range,
            hospitalisation_count_range,
            maximum_health_range,
        ) = match self.age {
            a if a <= 20 => (75..95, 8..15, 0..3, 97..100),
            a if a <= 35 => (65..85, 12..20, 1..6, 85..97),
            a if a <= 55 => (55..80, 15..25, 1..12, 72..88),
            a if a <= 75 => (30..55, 35..50, 5..25, 45..60),
            _ => (20..30, 45..70, 5..25, 35..50),
        };

        let mut rng = rand::thread_rng();

        self.health_percentage = rng.gen_range(health_range);
        self.hospitalisation_percentage = rng.gen_range(hospitalisation_percentage_range);
        self.hospitalisation_count = rng.gen_range(hospitalisation_count_range);
        self.maximum_health = rng.gen_range(maximum_health_range);
    }

    pub fn grow_up(&mut self) {
        self.age += 1;

        let education_finished_age = 18 + self.years_in_higher_education;
        if self.age == education_finished_age {}

        if self.age == 18 {
            self.debts = Debt::generate(self, self.salary);
            self.generate_daily_food_spending();
        }

        if self.age % 2 == 0 {
            self.hospitalisation_percentage += 1;
        }
    }

    // This should be executed everyday
    pub fn replenish_health(&mut self) {
        // Age determines the chance that the individual will regenerate their health
        let replenish_chance = 30
            * match self.age {
                a if a >= 75 => 3,
                a if a >= 60 => 2,
                _ => 1,
            };

        let replenish = rand::thread_rng().gen_range(0..=replenish_chance) == replenish_chance;

        if replenish {
            self.add_health(1);
        }
    }
}
