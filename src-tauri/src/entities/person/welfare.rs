pub const WELFARE_IMPACT_ONE: i32 = 2;
pub const WELFARE_IMPACT_TWO: i32 = 5;
pub const WELFARE_IMPACT_THREE: i32 = 10;
pub const WELFARE_IMPACT_FOUR: i32 = 20;
pub const WELFARE_IMPACT_FIVE: i32 = 45;
pub const WELFARE_IMPACT_SIX: i32 = 70;
pub const WELFARE_IMPACT_SEVEN: i32 = 100;  

#[derive(Default, Clone)]
pub struct WelfareDay {
    pub maximum: i32, // Maximum possible welfare for that day
    pub minimum: i32,
    pub amount: i32, // Welfare gained that day
}

#[derive(Clone)]
pub struct WelfareMachine {
    pub welfare_days: Vec<WelfareDay>,
}

impl Default for WelfareMachine {
    fn default() -> Self {
        Self {
            welfare_days: vec![WelfareDay::default(); 30]
        }
    }
}

impl WelfareMachine {
    pub fn add_welfare_if(&mut self, amount: i32, day: i32, condition: bool) {
        let idx = (day - 1) as usize;

        if condition {
            self.welfare_days[idx].amount += amount;
        }

        self.welfare_days[idx].maximum += amount;
    }
    
    pub fn remove_welfare_if(&mut self, amount: i32, day: i32, condition: bool) {
        let idx = (day - 1) as usize;

        if condition {
            self.welfare_days[idx].amount -= amount;
        }

        self.welfare_days[idx].minimum -= amount
    }

    pub fn welfare_reset(&mut self, day: i32) {
        let idx = (day - 1) as usize;
        self.welfare_days[idx] = WelfareDay::default();
    }
}