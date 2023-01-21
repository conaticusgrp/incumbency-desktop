use std::ops::Range;

use crate::common::util::percentage_based_output_int;

use super::person::{Person, EducationLevel::*, SpendingBehaviour};
use maplit::hashmap;
use rand::{Rng};

const US_DEBT_REPAYMENT_THRESHOLD: f32 = 32_000.; // Minimum salary required to start paying debts


impl Person {
    pub fn get_monthly_debt_cost(&self) -> f32 {
        let mut total = 0.;

        for debt in self.debts.iter() {
            total += debt.minimum_monthly_payoff;
        }

        total
    }
}

#[derive(Clone)]
pub struct Debt {
    pub owed: f32,
    pub minimum_monthly_payoff: f32, // Amount required to be paid per month
    pub required_to_pay: bool,
}

impl Debt {
    // TODO: add more types of debt
    pub fn generate(person: &mut Person, salary: f32) -> Vec<Self> {
        let mut debts: Vec<Self> = Vec::new();

        if person.age < 18 {
            return debts;
        }

        let mut rng = rand::thread_rng();
        let salary_percentage: f32 = rng.gen_range(25..43) as f32 / 100.; // Percentage of salary that must be paid on debts

        person.years_in_higher_education = rng.gen_range(1..4);

        let mut owed: f32 = match person.education_level {
            NoFormalEducation | HighSchoolDiploma => 0,
            College | AssociateDegree => rng.gen_range(10000..12500) * person.years_in_higher_education,
            Bachelors => rng.gen_range(12000..15000) * person.years_in_higher_education,
            AdvancedDegree => rng.gen_range(30000..34000) * person.years_in_higher_education,
        } as f32;

        let required_to_pay = salary >= US_DEBT_REPAYMENT_THRESHOLD;
        let education_finished_age = 18 + person.years_in_higher_education; // Age at which the individual finishes education

        if required_to_pay && person.age >= education_finished_age {
           owed -= (person.age - education_finished_age) as f32 * salary_percentage * salary;
        }

        // TODO: make these values random
        owed -= match person.spending_behaviour {
            SpendingBehaviour::One => Debt::get_education_prepaid_amount(3, 50..150),
            SpendingBehaviour::Two => Debt::get_education_prepaid_amount(15, 100..300),
            SpendingBehaviour::Three => Debt::get_education_prepaid_amount(30, 400..1200),
            SpendingBehaviour::Four => Debt::get_education_prepaid_amount(72, 800..3500),
        } as f32;

        if owed < 0. {
            return debts;
        }

        debts.push(Self {
            owed,
            minimum_monthly_payoff: salary_percentage,
            required_to_pay
        });

        debts
    }

    /// Get amount that the student has already paid off, excluding what they are required to pay
    fn get_education_prepaid_amount(is_prepaid_chance_percentage: i32, prepaid_range: Range<i32>) -> f32 {
        let is_prepaid = percentage_based_output_int::<bool>(hashmap! {
            true => is_prepaid_chance_percentage,
            false => 100 - is_prepaid_chance_percentage,
        });

        if is_prepaid {
            return 0.;
        }

        rand::thread_rng().gen_range(prepaid_range) as f32
    }
}