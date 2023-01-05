use std::{ops::Range, collections::HashMap};
use maplit::hashmap;
use rand::{Rng};
use serde::Serialize;
use crate::{common::util::{float_range, percentage_based_output_int, generate_percentage}, common::config::Config, game::generation::{generate_education_level, get_expected_salary_range}};
use EducationLevel::*;

use super::business::ProductType;

const US_DEBT_REPAYMENT_THRESHOLD: f32 = 32_000.; // Minimum salary required to start paying debts

#[derive(Default, Clone, Serialize)]
pub struct Person {
    pub education_level: EducationLevel,
    pub years_in_higher_education: i32, // Amount of years the individual spent in college or university (TODO: use this)
    pub job: Job,
    pub debts: Vec<Debt>,

    pub age: i32,
    pub balance: f32,

    pub expected_salary_range: Range<i32>, // Range of the expected salary for the person based on their education level
    pub salary: i32,

    pub spending_behaviour: SpendingBehaviour,
    pub daily_food_spending: i32,
    pub food_spending_streak: i32, // The amount of months the person has undergone the current food spending

    pub wants: HashMap<ProductType, f32>,
    pub business_this_month: usize,  // The business the individual will buy from this month, until marketing is re-evaluated
    pub purchase_days: HashMap<i32, i32>, // The days of the month that they will make a purchase - <day, quantity>
}

impl Person {
    pub fn generate(&mut self, config: &Config, product_demand: &mut HashMap<ProductType, f32>) {
        self.education_level = generate_education_level(&config);
        self.expected_salary_range = get_expected_salary_range(&config, &self.education_level);

        let expected_salary = ((self.expected_salary_range.start + self.expected_salary_range.end) / 2) as f32;

        self.spending_behaviour = self.generate_spending_behaviour();
        self.balance = self.generate_balance(expected_salary);
        self.age = self.generate_age();
        self.debts = Debt::generate(self, expected_salary);
        self.daily_food_spending = self.generate_daily_food_spending(expected_salary, None);

        self.generate_wants(expected_salary, product_demand);
    }

    fn behaviour_one(&self, salary: f32) {

    }

    fn behaviour_two(&self, salary: f32) {

    }

    fn behaviour_three(&self, salary: f32) {

    }

    fn behaviour_four(&self, salary: f32) {

    }

    fn generate_balance(&self, salary: f32) -> f32 {
        if salary > 0. {
            /*
                We calculate the average % of salary U.S citizens have in their bank account with ((average_salary * us_population) / us_gdp) * 100
                This evaluated to 107%, have added a 50% leeway which gives us a range between 53.5% and 214% of the individuals salary
            */
            return salary * float_range(0.535, 2.14, 3);
        }

        // TODO: make this depend on spending behaviour
        float_range(50., 1200., 3)
    }

    fn generate_spending_behaviour(&self) -> SpendingBehaviour {
        if matches!(self.job, Job::BusinessOwner(_)) {
            return percentage_based_output_int::<SpendingBehaviour>(hashmap! {
                SpendingBehaviour::One => 1,
                SpendingBehaviour::Two => 4,
                SpendingBehaviour::Three => 25,
                SpendingBehaviour::Four => 70,
            });
        }

        match self.education_level {
            NoFormalEducation => percentage_based_output_int::<SpendingBehaviour>(hashmap! {
                SpendingBehaviour::One => 75,
                SpendingBehaviour::Two => 20,
                SpendingBehaviour::Three => 4,
                SpendingBehaviour::Four => 1,
            }),

            HighSchoolDiploma => percentage_based_output_int::<SpendingBehaviour>(hashmap! {
                SpendingBehaviour::One => 20,
                SpendingBehaviour::Two => 70,
                SpendingBehaviour::Three => 9,
                SpendingBehaviour::Four => 1,
            }),

            College | AssociateDegree => percentage_based_output_int::<SpendingBehaviour>(hashmap! {
                SpendingBehaviour::One => 3,
                SpendingBehaviour::Two => 10,
                SpendingBehaviour::Three => 82,
                SpendingBehaviour::Four => 5,
            }),

            Bachelors | AdvancedDegree => percentage_based_output_int::<SpendingBehaviour>(hashmap! {
                SpendingBehaviour::One => 1,
                SpendingBehaviour::Two => 4,
                SpendingBehaviour::Three => 77,
                SpendingBehaviour::Four => 18,
            })
        }
    }

    fn generate_age(&self) -> i32 {
        let mut rng = rand::thread_rng();

        let age_category = percentage_based_output_int::<i32>(hashmap! {
            1 => 24,
            2 => 9,
            3 => 12,
            4 => 25,
            5 => 13,
            6 => 17,
        });

        match age_category {
            1 => rng.gen_range(0..=18),
            2 => rng.gen_range(19..=25),
            3 => rng.gen_range(26..=34),
            4 => rng.gen_range(35..=54),
            5 => rng.gen_range(55..=64),
            6 => rng.gen_range(65..=90),
            _ => unreachable!(),
        }
    }

    fn generate_wants(&mut self, salary: f32, product_demand: &mut HashMap<ProductType, f32>) {
        // TODO: generate based on more factors rather than just salary
        let mut rng = rand::thread_rng();

        // The percentage of balance that will be added to wants
        let (balance_percentage, salary_percentage) = match self.spending_behaviour {
            SpendingBehaviour::One => (float_range(0.4, 1., 2), rng.gen_range(1..5) as f32),
            SpendingBehaviour::Two => (float_range(0.08, 0.3, 3), rng.gen_range(1..=3) as f32),
            SpendingBehaviour::Three =>  (float_range(0.02, 0.25, 3), float_range(0.01, 0.05, 2)),
            SpendingBehaviour::Four => (float_range(0.005, 0.058, 4), 0.),
        };

        let mut total_wants = self.balance * (balance_percentage as f32 / 100.);
        
        total_wants += (salary / 12.) * (salary_percentage as f32 / 100.);

        self.wants.insert(ProductType::LEISURE, total_wants);
        *product_demand.get_mut(&ProductType::LEISURE).unwrap() += total_wants;
    }

    /// `chances` - The percentage chance of going over or under minimum food spending. \
    /// `chances` - `Option<(chance_under_minimum, chance_over_minimum)>`
    fn generate_daily_food_spending(&self, salary: f32, chances: Option<(i32, i32)>) -> i32 {
        let minimum = self.minimum_appropriate_daily_food_spending(salary);
        let mut rng = rand::thread_rng();

        // TODO: change these boundaries when lower values are implemented
        // TODO: make these boundaries more dynamic based on if the person can afford it

        let percentage = generate_percentage();
        let mut spending = if let Some((chance_under_min, chance_over_min)) = chances {
            match percentage {
                p if p >= 100 - chance_under_min => chance_under_min, 
                p if p >= 100 - chance_under_min - chance_over_min => chance_over_min,
                _ => minimum,
            }
        } else {
            rng.gen_range(minimum - 1..minimum + 1)
        };

        if spending > 4 { spending = 4 }
        else if spending < 1 { spending = 1 };

        spending
    }

    fn minimum_appropriate_daily_food_spending(&self, salary: f32) -> i32 { // TODO: make this more random, sometimes choose a slightly less appropriate spending
        if salary > 0. {
            // TODO: handle case where none are affordable
            for i in 1..4 {
                if self.can_afford((i * 30) as f32) {
                    return i;
                }
            }
        }

        0
    }

    pub fn can_afford(&self, price: f32) -> bool {
        let cut_balance: f32 = self.balance - (self.balance * 0.1);
        cut_balance - price > 0.
    }
}

#[derive(Default, Clone, PartialEq, Eq, Hash, Serialize)]
pub enum EducationLevel {
    #[default]
    NoFormalEducation,
    HighSchoolDiploma,
    College,
    AssociateDegree,
    Bachelors,
    AdvancedDegree
}

#[derive(Default, Clone, PartialEq, Serialize)]
pub enum Job {
    BusinessOwner(usize), // usize refers to index of the business in the game state
    Employee(usize),
    #[default]
    Unemployed,
}

#[derive(Clone, Serialize)]
pub struct Debt {
    pub owed: f32,
    pub minimum_monthly_payoff: f32, // Amount required to be paid per month
    pub required_to_pay: bool, // Whether the person is legally required to pay for this debt each month
}

impl Debt {
    // TODO: add more types of debt
    fn generate(person: &mut Person, salary: f32) -> Vec<Self> {
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

        if required_to_pay && person.age > education_finished_age {
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

#[derive(Default, Clone, PartialEq, Eq, Hash, Serialize)]
pub enum SpendingBehaviour {
    #[default]
    One,
    Two,
    Three,
    Four,
}