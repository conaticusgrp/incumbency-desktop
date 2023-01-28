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
            if Debt::required_to_pay(self) {
                total += (self.salary / 12) as f32 * debt.minimum_monthly_payoff;
            }
        }

        total
    }
}

#[derive(Clone)]
pub struct Debt {
    pub owed: f32,
    pub minimum_monthly_payoff: f32, // Amount required to be paid per month
    pub debt_type: DebtType,
}

#[derive(Clone, PartialEq)]
pub enum DebtType {
    Education,
}

impl Debt {
    // TODO: add more types of debt
    pub fn generate(person: &mut Person, salary: i32) -> Vec<Self> {
        let mut debts: Vec<Self> = Vec::new();
        let mut rng = rand::thread_rng();

        person.years_in_higher_education = rng.gen_range(1..4);

        let mut owed: f32 = match person.education_level {
            NoFormalEducation | HighSchoolDiploma => 0,
            College | AssociateDegree => rng.gen_range(10000..12500) * person.years_in_higher_education,
            Bachelors => rng.gen_range(12000..15000) * person.years_in_higher_education,
            AdvancedDegree => rng.gen_range(30000..34000) * person.years_in_higher_education,
        } as f32;

        let education_finished_age = 18 + person.years_in_higher_education; // Age at which the individual finishes education
        let salary_percentage = rng.gen_range(11..35);

        if Debt::required_to_pay(person) && person.age >= education_finished_age {
           owed -= (person.age - education_finished_age) as f32 * salary_percentage as f32 * salary as f32;
        }

        owed -= match person.spending_behaviour {
            SpendingBehaviour::One => Debt::get_education_prepaid_amount(3, 50..150),
            SpendingBehaviour::Two => Debt::get_education_prepaid_amount(15, 100..300),
            SpendingBehaviour::Three => Debt::get_education_prepaid_amount(30, 400..1200),
            SpendingBehaviour::Four => Debt::get_education_prepaid_amount(72, 800..3500),
        };

        if owed < 0. {
            return debts;
        }

        debts.push(Self {
            owed,
            minimum_monthly_payoff: salary_percentage as f32,
            debt_type: DebtType::Education,
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

    pub fn required_to_pay(person: &Person) -> bool {
        person.age < 18 || person.salary as f32 >= US_DEBT_REPAYMENT_THRESHOLD
    }
}