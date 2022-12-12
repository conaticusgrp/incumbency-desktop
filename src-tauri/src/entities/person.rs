pub struct Person {
    pub education_level: EducationLevel,
    pub job: Job,
    pub debts: Vec<Debt>,

    pub age: i8,
    pub balance: f32,

    pub spending_behaviour: SpendingBehaviour,
    pub daily_food_spending: i32,
    pub food_spending_streak: i32, // the amount of months the person has undergone the current food spending
}

#[derive(Default, Clone)]
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
    pub monthly_payoff: i32, // amount required to be paid per month
    pub required_to_pay: bool, // whether the person is legally required to pay for this debt each month
}

#[derive(Default, Clone)]
pub enum SpendingBehaviour {
    #[default]
    ONE,
    TWO,
    THREE,
    FOUR,
}