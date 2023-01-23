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
}