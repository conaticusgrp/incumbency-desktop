use std::{ops::Range, collections::HashMap};
use maplit::hashmap;
use rand::{Rng};
use tauri::http::header::LAST_MODIFIED;
use uuid::Uuid;
use crate::{common::{util::{float_range, percentage_based_output_int, Date, percentage_chance, chance_one_in, generate_unemployed_salary}, errors::{Error, IncResult}}, common::{config::Config, util::get_healthcare_group}, game::{generation::{generate_education_level, get_expected_salary_range}, structs::{TaxRule, HealthcareState, GameStateRules}}, entities::business::{ProductType, Business}, percentage_of, as_decimal_percent};
use EducationLevel::*;

use super::{debt::{Debt}, welfare::{WelfareMachine, WELFARE_IMPACT_FOUR, WELFARE_IMPACT_FIVE, WELFARE_IMPACT_THREE, WELFARE_IMPACT_TWO}};

#[derive(Default, Clone)]
pub struct Birthday {
    pub day: i32, // 1-30
    pub month: i32, // 1-12
}

impl Birthday {
    /// Generates a random birthday date
    pub fn generate() -> Self {
        let mut rng = rand::thread_rng();

        Self {
            day: rng.gen_range(1..=30),
            month: rng.gen_range(1..=12),
        }
    }

    pub fn from(date: &Date) -> Self {
        Self {
            day: date.day,
            month: date.month,
        }
    }
}

#[derive(Default, Clone, Eq, PartialEq)]
pub enum Gender {
    #[default]
    Male,
    Female,
}

#[derive(Default, Clone)]
pub struct Person {
    pub id: Uuid,

    pub education_level: EducationLevel,
    pub years_in_higher_education: i32, // Amount of years the individual spent in college or university (TODO: use this)
    pub job: Job,
    pub debts: Vec<Debt>,
    pub monthly_debt_cost: f32,
    pub years_in_unemployment: i32,

    pub age: i32,
    pub birthday: Birthday,
    pub balance: f32,

    pub expected_salary_range: Range<i32>, // Range of the expected salary for the person based on their education level
    pub salary: i32,

    pub saving_percentage_range: Range<i32>, // Range of how much the individual wishes to save of their balance - varied by spending behaviour
    pub spending_behaviour: SpendingBehaviour,
    pub daily_food_spending: i32,

    pub demand: HashMap<ProductType, f32>,
    pub business_this_month: Option<Uuid>,  // The business the individual will buy from this month, until marketing is re-evaluated
    pub purchase_days: HashMap<i32, i32>, // The days of the month that they will make a purchase - <day, quantity>

    pub health_percentage: i32, // The percentage of their health that they have remaining
    pub hospitalisation_percentage: i32, // The percentage of their health that will require them to be hospitalised
    pub hospitalisation_count: i32, // The amount of times the individual has been hospitalised
    pub days_until_death: Option<i32>, // If the person is predicted to die, use this as a counter
    pub days_left_in_hospital: Option<i32>, // Days left that the person is in hospitalisation
    pub maximum_health: i32,

    pub homeless: bool,

    pub gender: Gender,

    pub welfare_machine: WelfareMachine,
    pub welfare: i32,

    pub birth_date: Option<Date>, // Date the person will have a baby child
}

// Static methods
impl Person {
    /// Generates a randomly aged person based on statistics
    pub fn new_generate(config: &Config, product_demand: &mut HashMap<ProductType, f32>, tax_rate: f32, tax_rule: &TaxRule, date: Date) -> IncResult<Self> {
        let mut person = Self {
            id: Uuid::new_v4(),
            age: Self::generate_age(),
            birthday: Birthday::generate(),
            ..Self::default()
        };

        person.generate_gender(date);

        person.education_level = generate_education_level(config);
        person.expected_salary_range = get_expected_salary_range(config, &person.education_level);

        let mut expected_salary = ((person.expected_salary_range.start + person.expected_salary_range.end) / 2) as i32;
        if person.age >= 70 || (person.age >= 65 && chance_one_in(60)) {
            // Retired
            person.job = Job::Retired;
            person.set_salary(15800);
            expected_salary = 15800;
        }

        let tax_rate = Self::get_tax_rate(tax_rule, tax_rate, expected_salary);

        person.generate_spending_behaviour();
        person.generate_balance(expected_salary);

        if person.age >= 18 {
            person.generate_daily_food_spending();
            person.debts = Debt::generate(&mut person, expected_salary as i32);
            person.get_monthly_debt_cost();
        } else {
            person.daily_food_spending = 0;
        }

        person.calculate_demand(expected_salary, Some(product_demand), tax_rate)?;
        person.generate_health();

        Ok(person)
    }

    pub fn get_tax_rate(rule: &TaxRule, standard_tax_rate: f32, salary: i32) -> f32 {
        if rule.enabled && salary >= rule.minimum_salary {
            return rule.tax_rate;
        }

        standard_tax_rate
    }

    /// Adds a new baby to the population
    pub fn new_infant(config: &Config, tax_rate: f32, tax_rule: &TaxRule, date: Date) -> IncResult<Self> {
        let mut infant = Self {
            id: Uuid::new_v4(),
            birthday: Birthday::from(&date),
            health_percentage: 100,
            hospitalisation_percentage: 8,
            maximum_health: 100,
            ..Self::default()
        };

        infant.generate_gender(date);

        infant.education_level = generate_education_level(config);
        infant.expected_salary_range = get_expected_salary_range(config, &infant.education_level);

        let expected_salary = ((infant.expected_salary_range.start + infant.expected_salary_range.end) / 2) as i32;
        let tax_rate = Self::get_tax_rate(tax_rule, tax_rate, expected_salary);

        infant.generate_spending_behaviour();
        infant.calculate_demand(0, None, tax_rate)?;

        Ok(infant)
    }

    fn generate_age() -> i32 {
        // This is not entirely accurate in order to avoid massive decrease at start of game
        let mut rng = rand::thread_rng();

        percentage_based_output_int::<i32>(hashmap! {
            (rng.gen_range(0..=18)) => 33,
            (rng.gen_range(19..=25)) => 12,
            (rng.gen_range(26..=34)) => 16,
            (rng.gen_range(35..=54)) => 35,
            (rng.gen_range(55..=64)) => 2,
            (rng.gen_range(65..=90)) => 2,
        })
    }
}

// Dynamic methods
impl Person {
    fn generate_gender(&mut self, date: Date) {
        if percentage_chance(50.) {
            self.gender = Gender::Male;
        }

        let mut rng = rand::thread_rng();

        if date.is_generation_day() && self.age <= 35 && self.age >= 18 && percentage_chance(1.2) {
            let day = rng.gen_range(1..=30);
            let month = rng.gen_range(1..=12);

            self.birth_date = Some(Date::new(day, month, 0));
            return self.gender = Gender::Female;
        }

        if self.age <= 35 && percentage_chance(42.) { // 42% chance of having a newborn (based on real world statistic)
            let mut latest_birthday_year = date.year + (35 - self.age) - 1;
            if latest_birthday_year < 0 { latest_birthday_year = 0; }

            let mut earliest_birth_year = date.year;
            if self.age < 18 {
                earliest_birth_year += 18 - self.age;
            }

            let day = rng.gen_range(1..=30);
            let month = rng.gen_range(1..=12);
            let year = rng.gen_range(earliest_birth_year..=latest_birthday_year);

            self.birth_date = Some(Date::new(day, month, year));
        }

        self.gender = Gender::Female;
    }

    pub fn due_birth(&mut self, date: &Date, healthcare: &mut HealthcareState, rules: &GameStateRules) -> bool {
        if self.gender == Gender::Male || self.birth_date.is_none() {
            return false;
        }

        let birth_date = self.birth_date.clone().unwrap();
        if !date.is_eq(birth_date) {
            return false;
        }

        self.remove_health(rand::thread_rng().gen_range(20..40), healthcare, rules);

        !self.days_until_death.is_some() // Returns false if the person died during birth
    }

    fn generate_spending_behaviour(&mut self) {
        if matches!(self.job, Job::BusinessOwner(_)) {
            self.spending_behaviour = percentage_based_output_int::<SpendingBehaviour>(hashmap! {
                SpendingBehaviour::One => 1,
                SpendingBehaviour::Two => 4,
                SpendingBehaviour::Three => 25,
                SpendingBehaviour::Four => 70,
            });

            return;
        }

        self.spending_behaviour = match self.education_level {
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
        };

        self.saving_percentage_range = match self.spending_behaviour {
            SpendingBehaviour::One => 5..8,
            SpendingBehaviour::Two => 8..12,
            SpendingBehaviour::Three => 10..20,
            SpendingBehaviour::Four => 20..28,
        }
    }

    fn generate_balance(&mut self, salary: i32) {
        // TODO: Vary on spending behaviour

        if self.age >= 18 {
            if salary > 0 {
                /*
                    We calculate the average % of salary U.S citizens have in their bank account with ((average_salary * us_population) / us_gdp) * 100
                    This evaluated to 107%, have added a 50% leeway which gives us a range between 53.5% and 214% of the individuals salary
                */
                return self.balance = salary as f32 * float_range(0.535, 2.14, 3);
            }

            return self.balance = float_range(50., 1200., 1);
        }

        // Default child's balance
        self.balance = float_range(4., 90., 1);
    }

    pub fn calculate_demand(&mut self, salary: i32, product_demand: Option<&mut HashMap<ProductType, f32>>, tax_rate: f32) -> IncResult<()> {
        if salary == 0 {
            *self.demand.entry(ProductType::Leisure).or_insert(0.) = 0.;
            return Ok(());
        }

        let mut rng = rand::thread_rng();

        let balance_percentage = match self.spending_behaviour {
            SpendingBehaviour::One => rng.gen_range(25..50),
            SpendingBehaviour::Two => rng.gen_range(10..30),
            SpendingBehaviour::Three => rng.gen_range(2..8),
            SpendingBehaviour::Four => rng.gen_range(1..3),
        };

        let remaining_balance = self.balance - (4. * 30.) - self.monthly_debt_cost;
        let mut total_demand = remaining_balance * as_decimal_percent!(balance_percentage);
        total_demand -= (salary / 12) as f32 * tax_rate;
        if total_demand < 0. {
            total_demand = 0.; 
        }
        
        *self.demand.entry(ProductType::Leisure).or_insert(total_demand) += total_demand;

        if let Some(prod_dem) = product_demand {
            let product_demand = prod_dem.get_mut(&ProductType::Leisure).ok_or(Error::DangerUnexpected)?;
            *product_demand += total_demand; 
        }

        Ok(())
    }

    pub fn calculate_daily_food_spending(&self) -> i32 {
        let healthy_cost = self.monthly_debt_cost + (4 * 30) as f32;
        let survivable_cost = self.monthly_debt_cost + (3 * 30) as f32;
        let unhealthy_cost = self.monthly_debt_cost + (2 * 30) as f32;

        if self.can_afford_bare(healthy_cost) {
            return 4;
        } else if self.can_afford_bare(survivable_cost) {
            return 3;
        } else if self.can_afford_bare(unhealthy_cost) {
            let (action_one_chance, action_two_chance) = match self.spending_behaviour {
                SpendingBehaviour::One => (90, 10),
                SpendingBehaviour::Two => (55, 45),
                SpendingBehaviour::Three => (35, 65),
                SpendingBehaviour::Four => (10, 90),
            };

            let action = percentage_based_output_int(hashmap! {
                1 => action_one_chance,
                2 => action_two_chance,
            });

            if action == 1 {
                return 3;
            } else {
                return 2;
            }
        }

        1
    }

    /// This should be done every time the individual's salary changes, and every month.
    pub fn generate_daily_food_spending(&mut self) {
        // TODO: change me
        if let Job::BusinessOwner(_) = self.job { return self.daily_food_spending = 4 }

        if self.job == Job::Unemployed {
            self.set_salary(generate_unemployed_salary()); // TODO: make me more dynamic & move me
        }
        
        self.daily_food_spending = self.calculate_daily_food_spending()
    }

    pub fn can_afford(&self, price: f32) -> bool {
        let mut rng = rand::thread_rng();
        let saving_percent = rng.gen_range(self.saving_percentage_range.clone()) as f32 / 100.;

        let mut cut_balance: f32 = self.balance - (self.balance * saving_percent) - ((self.salary as f32 / 12.) * saving_percent);
        cut_balance -= self.monthly_debt_cost;
        cut_balance -= self.daily_food_spending as f32 * 30.;

        cut_balance - price > 0.
    }

    pub fn can_afford_bare(&self, price: f32) -> bool {
        self.balance - price > 0.
    }

    pub fn business_pay(&mut self, payer: &mut Business, amount: f64) {
        self.balance += amount as f32;
        payer.balance -= amount;
    }
    
    pub fn pay_tax(&mut self, government_balance: &mut i64, amount: f32) {
        if amount < 0. { return }
        self.balance -= amount;
        *government_balance += amount as i64;
    }

    pub fn check_birthday(&mut self, date: &Date) {
        if date.day == self.birthday.day && date.month == self.birthday.month {
            self.grow_up();
        }
    }

    pub fn day_pass(&mut self, day: i32, healthcare: &mut HealthcareState, date: &Date, businesses: &mut HashMap<Uuid, Business>, purchases: &mut u32, total_possible_purchases: &mut u32, rules: &GameStateRules, food_coverage: &mut i32, unemployed_food_coverage: &mut i32) -> IncResult<bool> {
        self.check_birthday(date);

        if let Some(ref mut days) = self.days_until_death {
            *days -= 1;
            if *days <= 0 {
                return Ok(true);
            }
        }

        let mut rng = rand::thread_rng();

        if !self.homeless && self.age >= 18 && self.job == Job::Unemployed {
            // TODO: affect this by other factors
            if chance_one_in(500 * 365) { // 1 in 500 chance each year 
                self.homeless = true;
            } 
        }

        if chance_one_in(7300) { // Average person has minor accident every 20 years
            self.remove_health(rng.gen_range(15..=25), healthcare, rules);
        }

        if self.homeless {
            self.balance += rng.gen_range(1..=2) as f32;
        }

        if rules.cover_food_rule.enabled && self.salary < rules.cover_food_rule.maximum_salary && *food_coverage <= rules.cover_food_rule.people_count {
            self.balance += 4.;
            *food_coverage += 1;
        } else if rules.cover_food_unemployed_rule.enabled && self.job == Job::Unemployed && *unemployed_food_coverage <= rules.cover_food_unemployed_rule.people_count {
            self.balance += 4.;
            *food_coverage += 1;
        }

        if self.age >= 18 && !matches!(self.job, Job::BusinessOwner(_)) {
            self.daily_food_spending = self.calculate_daily_food_spending();
            self.balance -= self.daily_food_spending as f32;

            let (health_loss_chance, welfare_loss) = match self.daily_food_spending { // Chance that the individual will lose 1% of their health
                1 => (50., WELFARE_IMPACT_FIVE),
                2 => (25., WELFARE_IMPACT_FOUR),
                3 => (0.9, WELFARE_IMPACT_THREE),
                4 => (0.6, 0),
                _ => unreachable!(),
            };

            self.welfare_machine.welfare_reset(day);
            self.welfare_machine.remove_welfare_if(welfare_loss, day, welfare_loss != 0);

            if percentage_chance(health_loss_chance) {
                self.remove_health(1, healthcare, rules);
            }
        }

        let mut in_hospital = false;

        if let Some(ref mut days) = self.days_left_in_hospital {
            in_hospital = true;

            *days -= 1;
            if *days <= 0 {
                self.days_left_in_hospital = None;
                let healthcare_group = get_healthcare_group(self.age, healthcare);
                healthcare_group.current_capacity += 1;
            }
        }

        self.welfare_machine.remove_welfare_if(WELFARE_IMPACT_THREE, day, in_hospital);
        self.replenish_health();

        let quantity_opt = self.purchase_days.remove(&day);
        let mut not_afford_wanted_item = false;

        if let Some(quantity) = quantity_opt {
            let business_this_month = &self.business_this_month.ok_or(Error::DangerUnexpected)?;
            let business = businesses.get_mut(business_this_month).ok_or(Error::Warning("Could not find business that was expected to purchase from.".to_string()))?;
            let item_cost = (business.product_price * quantity) as f32;
            *total_possible_purchases += quantity as u32;

            for _ in 0..quantity {
                if self.can_afford(item_cost) {
                    *purchases += 1;

                    self.balance -= item_cost;
                    let demand = self.demand.get_mut(&business.product_type).ok_or(Error::DangerUnexpected)?;
                    *demand -= item_cost;
                    if *demand < 0. { *demand = 0. }

                    business.balance += item_cost as f64;
                    self.welfare_machine.add_welfare_if(WELFARE_IMPACT_TWO, day, true);
                } else {
                    not_afford_wanted_item = true;
                    break;
                }
            }
        }

        let no_business_this_month = match self.business_this_month {
            Some(_) => false,
            None => true,
        };

        self.welfare_machine.remove_welfare_if(WELFARE_IMPACT_THREE, day, not_afford_wanted_item);
        self.welfare_machine.remove_welfare_if(WELFARE_IMPACT_FOUR, day, no_business_this_month);

        self.get_welfare();

        Ok(false)
    }

    pub fn get_welfare(&mut self) {
        let mut range_total = 0;
        let mut amount_total = 0;

        for day in self.welfare_machine.welfare_days.iter() {
            let mut minimum = day.minimum;

            if day.minimum < 0 {
                minimum = 0;
            }

            range_total += minimum + (day.maximum + -day.minimum);
            amount_total += day.amount + (-day.minimum);
        }

        if range_total == 0 {
            return self.welfare = 100;
        }

        self.welfare = percentage_of!(amount_total; / range_total);
    }

    pub fn set_salary(&mut self, salary: i32) {
        self.salary = salary;
        self.get_monthly_debt_cost();
    }
}

// MUST be ordered from lowest to highest
#[derive(Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EducationLevel {
    #[default]
    NoFormalEducation,
    HighSchoolDiploma,
    College,
    AssociateDegree,
    Bachelors,
    AdvancedDegree
}

#[derive(Default, Clone, PartialEq)]
pub enum Job {
    BusinessOwner(Uuid), // usize refers to index of the business in the game state
    Employee(Uuid),
    Retired,
    #[default]
    Unemployed,
}

#[derive(Default, Clone, PartialEq, Eq, Hash)]
pub enum SpendingBehaviour {
    #[default]
    One,
    Two,
    Three,
    Four,
}