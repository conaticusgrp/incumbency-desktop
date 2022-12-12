use std::ops::Range;
use maplit::hashmap;
use crate::util::percentage_based_output_float;
use EducationLevel::*;

pub struct Person {
    pub education_level: EducationLevel,
    pub job: Job,
    pub debts: Vec<Debt>,

    pub age: i8,
    pub balance: f32,
    pub expected_salary_range: Range<i32>, // Range of the expected salary for the person based on their education level

    pub spending_behaviour: SpendingBehaviour,
    pub daily_food_spending: i32,
    pub food_spending_streak: i32, // The amount of months the person has undergone the current food spending
}


// pub enum EducationLevel { // frequency  salary_range
//     #[default]
//     NoFormalEducation,    // 10.2%      $15,000-$30,000
//     HighSchoolDiploma,    // 28.52%     $30,000-$40,000
//     College,              // 16.12%     $50,000-$60,000
//     AssociateDegree,      // 10.18%     $40,000-$70,000
//     Bachelors,            // 29.48%     $60,000-$90,000
//     AdvancedDegree        // 5.5%       $100,000-$300,000

impl Person {
    pub fn generate(&mut self) {
        self.education_level = percentage_based_output_float(hashmap! {
            NoFormalEducation => 10.2,
            HighSchoolDiploma => 28.52,
            College => 16.12,
            AssociateDegree => 10.18,
            Bachelors => 29.48,
            AdvancedDegree => 5.5,
        }, 2).unwrap();
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
    pub monthly_payoff: i32, // Amount required to be paid per month
    pub required_to_pay: bool, // Whether the person is legally required to pay for this debt each month
}

#[derive(Default, Clone)]
pub enum SpendingBehaviour {
    #[default]
    ONE,
    TWO,
    THREE,
    FOUR,
}