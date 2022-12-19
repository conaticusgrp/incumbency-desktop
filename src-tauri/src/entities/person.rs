use std::{ops::Range, collections::HashMap};
use maplit::hashmap;
use rand::{Rng};
use crate::{util::{percentage_based_output_float, float_range, percentage_based_output_int, generate_percentage, generate_percentage_float}, config::Config};
use EducationLevel::*;

use super::business::ProductType;

const US_DEBT_REPAYMENT_THRESHOLD: f32 = 32_000.; // Minimum salary required to start paying debts

#[derive(Default)]
pub struct Person {
    pub education_level: EducationLevel,
    pub years_in_higher_education: i32, // Amount of years the individual spent in college or university (TODO: use this)
    pub job: Job,
    pub debts: Vec<Debt>,

    pub age: i32,
    pub balance: f32,
    pub expected_salary_range: Range<i32>, // Range of the expected salary for the person based on their education level

    pub spending_behaviour: SpendingBehaviour,
    pub daily_food_spending: i32,
    pub food_spending_streak: i32, // The amount of months the person has undergone the current food spending

    pub wants: HashMap<ProductType, f32>,
}

impl Person {
    pub fn generate(&mut self, config: &Config) {
        self.job = Job::Unemployed;

        self.education_level = self.generate_education_level(&config);

        self.expected_salary_range = match self.education_level {
            NoFormalEducation => 15000..30000,
            HighSchoolDiploma => 30000..40000,
            College => 50000..60000,
            AssociateDegree => 40000..70000,
            Bachelors => 60000..90000,
            AdvancedDegree => 100000..300000,
        };

        let expected_salary = ((self.expected_salary_range.start + self.expected_salary_range.end) / 2) as f32;

        self.spending_behaviour = self.generate_spending_behaviour();
        self.balance = self.generate_balance(expected_salary);
        self.age = self.generate_age();
        self.debts = Debt::generate(self, expected_salary);
        self.daily_food_spending = self.generate_daily_food_spending(expected_salary, None);

        self.generate_wants(expected_salary);
    }

    fn generate_education_level(&self, config: &Config) -> EducationLevel {
        let nc = config.no_education_chance;
        let hsdc = config.high_school_diploma_chance;
        let cc = config.college_chance;
        let asc = config.associate_degree_chance;
        let bc = config.bachelors_chance;
        let addc = config.advanced_degree_chance;

        percentage_based_output_float::<EducationLevel>(hashmap! {
            NoFormalEducation => nc,
            HighSchoolDiploma => hsdc,
            College => cc,
            AssociateDegree => asc,
            Bachelors => bc,
            AdvancedDegree => addc,
        }, 2)
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
            NoFormalEducation | HighSchoolDiploma => percentage_based_output_int::<SpendingBehaviour>(hashmap! {
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

        let age_category = percentage_based_output_float::<i32>(hashmap! {
            1 => 23.5,
            2 => 8.5,
            3 => 12.3,
            4 => 25.7,
            5 => 13.1,
            6 => 16.9,
        }, 1);

        match age_category {
            1 => rng.gen_range(0..=18),
            2 => rng.gen_range(19..=25),
            3 => rng.gen_range(26..=34),
            4 => rng.gen_range(35..=54),
            5 => rng.gen_range(55..=64),
            6 => rng.gen_range(65..=90),
            _ => panic!("How did we get here.."),
        }
    }

    fn generate_wants(&mut self, salary: f32) {
        // TODO: generate based on more factors rather than just salary
        let mut rng = rand::thread_rng();

        // The percentage of balance that will be added to wants
        let balance_percentage = match self.spending_behaviour {
            SpendingBehaviour::One => rng.gen_range(5..15),
            SpendingBehaviour::Two => rng.gen_range(1..5),
            _ => 0,
        };

        let mut total_wants = self.balance * (balance_percentage as f32 / 100.);

        let salary_percenatge = rng.gen_range(5..25) as f32;
        total_wants += salary * (salary_percenatge / 100.);

        self.wants.insert(ProductType::LEISURE, total_wants);
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

    fn can_afford(&self, price: f32) -> bool {
        let cut_balance = self.balance * 0.1;
        cut_balance - price > 0.
    }
}

#[derive(Default, Clone, PartialEq, Eq, Hash)]
pub enum EducationLevel {
    #[default]
    NoFormalEducation,
    HighSchoolDiploma,
    College,
    AssociateDegree,
    Bachelors,
    AdvancedDegree
}

#[derive(Default, Clone)]
pub enum Job {
    BusinessOwner(usize), // usize refers to index of the business in the game state
    Employee(usize),
    #[default]
    Unemployed,
}

#[derive(Clone)]
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

#[derive(Default, Clone, PartialEq, Eq, Hash)]
pub enum SpendingBehaviour {
    #[default]
    One,
    Two,
    Three,
    Four,
}