use maplit::hashmap;
use rand::Rng;

use crate::{common::config::Config, common::util::{percentage_based_output_int, float_range}, game::{generation::{generate_education_level, get_expected_salary_range}}, as_decimal_percent, percentage_of};
use super::person::{EducationLevel::{*, self}, Person, Job};

#[derive(Default, Clone, PartialEq, Eq, Hash)]
pub enum ProductType {
    #[default]
    LEISURE,

    // These will be implemented later:
    // FURNITURE
    // HOUSES
}

#[derive(Default, Clone)]
pub struct Business {
    pub balance: f32,

    pub minimum_education_level: EducationLevel,
    pub expected_marketing_reach: i32, // Amount of population that the marketing will reach (roughly)
    pub product_price: i32,
    pub production_cost_per_product: f32,
    pub marketing_cost_percentage: i32,
    pub product_type: ProductType,
    
    pub employee_salary: i32,
    pub default_income_per_employee: i32, // Default profit that is made from an employee salary, not taking into account the employee's welfare

    pub expected_income: i32, // Expected income for the current month
    pub last_month_balance: f32, // Used to calculate the income for this month
}

impl Business {
    /// Generates a business based on demand
    pub fn generate(&mut self, config: &Config, product_type: ProductType, product_demand: f32, remaining_market_percentage: &mut f32, people: &mut Vec<Person>, idx: usize, tax_rate: f32) -> bool {
        let mut rng = rand::thread_rng();

        self.product_type = product_type;
        self.minimum_education_level = generate_education_level(&config);
        self.marketing_cost_percentage = rng.gen_range(5..12);
        self.product_price = rng.gen_range(2..100); // TODO: determine this price more accurately?
        self.production_cost_per_product = self.product_price as f32 * float_range(0.15, 0.25, 3);

        let (sufficient_businesses, marketing_reach_percentage) = self.generate_marketing_reach(remaining_market_percentage);
        if sufficient_businesses { return sufficient_businesses }

        self.assign_to_people(marketing_reach_percentage, people, idx);

        self.expected_income = (product_demand * marketing_reach_percentage) as i32;

        // TODO: make this more varied & accurate, influence it by external factors
        let production_cost = self.get_production_cost();
        let marketing_cost = as_decimal_percent!(self.marketing_cost_percentage) * self.expected_income as f32;
        // This can only be a maximum of 67%, leaving roughly 30% capacity for employees, the minimum (with tax no lower than 20%) is 40%
        let mut loss_percentage = percentage_of!(marketing_cost + production_cost; / self.expected_income) + (tax_rate * 100.) as i32;

        self.employee_salary = self.generate_employee_salary(config, loss_percentage);
        let employee_count = self.generate_employee_count();

        self.default_income_per_employee = percentage_of!(self.expected_income; / employee_count);
        loss_percentage += percentage_of!(employee_count * (self.employee_salary / 12); / self.expected_income);

        // TODO: balance the amount of people who get employment
        self.assign_employees(people, employee_count, idx);
        self.set_starting_balance(loss_percentage);

        false
    }

    fn set_starting_balance(&mut self, loss_percentage: i32) {
        let expected_income_fl = self.expected_income as f32;
        let expected_profits = (expected_income_fl - (self.expected_income as f32 * as_decimal_percent!(loss_percentage))) as i32;

        self.balance = expected_profits as f32 * float_range(0.15, 3., 3); // A range of 0% - 300% of the expected profit is the business balance
        self.balance -= expected_income_fl * as_decimal_percent!(loss_percentage);
        self.last_month_balance = self.balance;
    }

    fn generate_employee_count(&self) -> i32 {
        let budget_allocation = (self.expected_income as f32 * float_range(0.15, 0.3, 3)) as i32;
        budget_allocation / (self.employee_salary / 12)
    }

    fn generate_employee_salary(&self, config: &Config, loss_percentage: i32) -> i32 {
        let expected_salary_range = get_expected_salary_range(&config, &self.minimum_education_level);

        let mid_of_range = (expected_salary_range.start + expected_salary_range.end) / 2; // middle of expected salary range
        let lower_mid_of_range = expected_salary_range.start + ((expected_salary_range.end - mid_of_range) / 2); // lower middle of expected salary range

        // Generate a more narrowed down range based on the randomly generated marketing cost and product cost
        // The employee salary will be lower if the marketing and product cost is cheap, this is to compensate and keep the economy balanced
        // The purpose of lowering the employee salary is that there is less work output from employees, so there is a higher staff turnover
        let employee_salary_range = match loss_percentage {
            loss if loss >= 60 => mid_of_range..expected_salary_range.end,
            loss if loss >= 45 => lower_mid_of_range..mid_of_range,
            _ => expected_salary_range.start..lower_mid_of_range,
        };

        rand::thread_rng().gen_range(employee_salary_range)
    }

    pub fn get_production_cost(&self) -> f32 {
        (self.expected_income / self.product_price) as f32 * self.production_cost_per_product
    }

    fn generate_marketing_reach(&self, remaining_market_percentage: &mut f32) -> (bool, f32) {
        let marketing_reach_percentage = match self.minimum_education_level {
            NoFormalEducation => self.random_marketing_percentage_multiplyer(0.3, 0.5),
            HighSchoolDiploma => self.random_marketing_percentage_multiplyer(0.5, 0.9),
            College => self.random_marketing_percentage_multiplyer(0.6, 1.1),
            AssociateDegree => self.random_marketing_percentage_multiplyer(0.8, 1.4),
            Bachelors => self.random_marketing_percentage_multiplyer(1., 2.1),
            AdvancedDegree => self.random_marketing_percentage_multiplyer(0.5, 4.),
        } as f32;

        if (*remaining_market_percentage - marketing_reach_percentage) < 0. {
            return (true, marketing_reach_percentage);
        }

        *remaining_market_percentage -= marketing_reach_percentage;
        (false, marketing_reach_percentage)
    }

    pub fn assign_to_people(&self, market_percentage: f32, people: &mut Vec<Person>, idx: usize) {
        let mut rng = rand::thread_rng();
        let reach = (market_percentage * people.len() as f32) as i32;

        // People who have not yet picked a business to buy from
        let unassigned_people: Vec<&mut Person> = people.iter_mut().filter(|p| p.business_this_month == 0).collect(); // TODO: optimise this
        let mut count = 0;

        for person in unassigned_people {
            if count == reach { break }

            person.business_this_month = idx;
            let wants = person.wants[&self.product_type];
            let purchase_capacity = wants as i32 / self.product_price;

            for _ in 0..purchase_capacity {
                let day = rng.gen_range(1..=30);
                *person.purchase_days.entry(day).or_insert(1) += 1;
            }

            count += 1;
        }
    }

    fn assign_employees(&self, people: &mut Vec<Person>, employee_count: i32, idx: usize) {
        let minimum_education_level = self.minimum_education_level.clone();
        let unemployed_people: Vec<&mut Person> = people.iter_mut().filter(|p| {
            p.job == Job::Unemployed && p.education_level == minimum_education_level
        }).collect(); // TODO: optimise this

        let mut count = 0;
        for person in unemployed_people {
            if count == employee_count { break }

            person.job = Job::Employee(idx);
            person.salary = self.employee_salary;

            count += 1;
        }
    }

    /// Multiplies the percentage target audience for the market based on educated odds 
    pub fn random_marketing_percentage_multiplyer(&self, min: f32, max: f32) -> f32 {
        // 1 - smallest, 3 - largest
        let tier = percentage_based_output_int(hashmap! {
            1 => 75,
            2 => 20,
            3 => 5,
        });

        let mut rng = rand::thread_rng();
        let increase_multiplyer = match tier {
            8 => rng.gen_range(2..5) as f32, // Increase start and end by a random range of 150%-320%
            3 => rng.gen_range(5..10) as f32,
            _ => 1.,
        };

        float_range(min * increase_multiplyer, max * increase_multiplyer, 2)
    }

    /// This function assigns the business to a new market with a new market percentage. This runs monthly.
    pub fn get_new_market(&mut self, market_percentage: f32, cost_per_percent: f32, people: &mut Vec<Person>, demand: f32, idx: usize) {
        self.assign_to_people(market_percentage, people, idx);

        self.expected_income = (demand * market_percentage) as i32;
        self.balance -= self.get_production_cost();
        self.balance -= market_percentage * cost_per_percent;
    }

    /// This is run on a monthly basis
    pub fn pay_owner(&mut self, owner: &mut Person) {
        let month_profits = self.balance - self.last_month_balance; // The profit percentage earned in the current month

        // TODO: Vary this on spending behaviour
        let owner_expected_income = month_profits / 2.;
        if owner_expected_income < (self.employee_salary as f32 / 12.) {
            owner.business_pay(self, self.employee_salary as f32);
            return;
        }

        owner.business_pay(self, owner_expected_income);
    }
}