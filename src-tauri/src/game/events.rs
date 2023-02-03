use serde::{Serialize, Deserialize};
use serde_json::json;
use tauri::{State, AppHandle, Manager};

use crate::entities::{person::person::Person, business::Business};

use super::{state_manager::GameStateSafe, structs::{GameState, TaxRule}};

#[derive(PartialEq, Eq, Hash)]
pub enum App {
    Finance = 2,
    Healthcare = 3,
    Welfare = 4,
    Business = 5,
}

#[derive(Serialize, Deserialize)]
pub struct FinanceAppOpenedPayload {
    pub government_balance: i64,
    pub average_monthly_income: i32,
    pub expected_person_income: i64,
    pub expected_business_income: i64,
    pub used_hospital_capacity: i32,
    pub total_hospital_capacity: i32,
    pub business_tax_rate: i32,
    pub tax_rate: i32,
    pub healthcare_budget: i64,
    pub welfare_budget: i64,
    pub business_budget: i64,
    pub spare_budget: i64,
    pub rules: serde_json::Value,
}

pub fn get_app_from_id(app_id: u8) -> Option<App> {
    match app_id {
        a if a == App::Finance as u8 => Some(App::Finance),
        a if a == App::Healthcare as u8 => Some(App::Healthcare),
        a if a == App::Welfare as u8 => Some(App::Welfare),
        a if a == App::Business as u8 => Some(App::Business),
        _ => None,
    }
}

#[tauri::command]
pub fn app_open(state_mux: State<'_, GameStateSafe>, app_id: u8) -> String {
    let mut state = state_mux.lock().unwrap();

    let app = match get_app_from_id(app_id) {
        Some(a) => a,
        None => return String::new(),
    };

    *state.open_apps.entry(app).or_insert(true) = true;

    match app {
        App::Finance => {
            let payload = FinanceAppOpenedPayload {
                government_balance: state.government_balance,
                average_monthly_income: state.finance_data.average_monthly_income,
                expected_person_income: state.finance_data.expected_person_income,
                expected_business_income: state.finance_data.expected_business_income,
                used_hospital_capacity: state.healthcare.get_current_capacity(),
                total_hospital_capacity: state.healthcare.total_capacity,
                business_tax_rate: (state.business_tax_rate * 100.) as i32,
                tax_rate: (state.tax_rate * 100.) as i32,
                healthcare_budget: state.healthcare.budget,
                welfare_budget: state.welfare_budget,
                business_budget: state.business_budget,
                spare_budget: state.spare_budget,
                rules: json!({
                    "tax": { "enabled": state.rules.tax_rule.enabled, "data": {
                        "minimum_salary": state.rules.tax_rule.minimum_salary,
                        "tax_rate": (state.rules.tax_rule.tax_rate * 100.) as i32
                    }},

                    "business_tax": { "enabled": state.rules.tax_rule.enabled, "data": {
                        "minimum_monthly_income": state.rules.business_tax_rule.minimum_monthly_income,
                        "tax_rate": (state.rules.business_tax_rule.tax_rate * 100.) as i32,
                    }},
                }),
            };

            serde_json::to_string(&payload).unwrap()
        },
        _ => String::new(),
    }
}

#[tauri::command]
pub fn app_close(state_mux: State<'_, GameStateSafe>, app_id: u8) {
    let mut state = state_mux.lock().unwrap();

    let app = match get_app_from_id(app_id) {
        Some(a) => a,
        None => return,
    }; 
    *state.open_apps.entry(app).or_insert(false) = false;
}

fn set_rule(state: &mut GameState, id: i32, enabled: bool) {
    match id {
        0 => {
            state.rules.tax_rule.enabled = enabled;
        },
        1 => {
            state.rules.business_tax_rule.enabled = enabled;
        },
        2 => {
            state.rules.business_funding_rule.enabled = enabled;
        },
        3 => {
            state.rules.deny_age_rule.enabled = enabled;
        },
        4 => {
            state.rules.deny_health_percentage_rule.enabled = enabled;
        },
        5 => {
            state.rules.cover_food_rule.enabled = enabled;
        },
        6 => {
            state.rules.cover_food_unemployed_rule.enabled = enabled;
        }
    };

}

#[tauri::command]
pub fn enable_rule(state_mux: State<'_, GameStateSafe>, rule_id: i32) {
    let mut state = state_mux.lock().unwrap();
    set_rule(&mut state, rule_id, true);
}

#[tauri::command]
pub fn disable_rule(state_mux: State<'_, GameStateSafe>, rule_id: i32) {
    let mut state = state_mux.lock().unwrap();
    set_rule(&mut state, rule_id, false);
}

#[tauri::command]
pub fn update_rule(state_mux: State<'_, GameStateSafe>, rule_id: i32, data: serde_json::Value) {
    let mut state = state_mux.lock().unwrap();
    match rule_id {
        0 => {
            state.rules.tax_rule.minimum_salary = (data.get("minimum_salary").unwrap().as_i64().unwrap()) as i32;
            state.rules.tax_rule.tax_rate = (data.get("tax_rate").unwrap().as_f64().unwrap() / 100.) as f32;
        },
        1 => {
            state.rules.business_tax_rule.minimum_monthly_income = data.get("minimum_monthly_income").unwrap().as_f64().unwrap();
            state.rules.business_tax_rule.tax_rate = (data.get("tax_rate").unwrap().as_f64().unwrap() / 100.) as f32;
        },
        2 => {
            state.rules.business_funding_rule.fund = data.get("fund").unwrap().as_f64().unwrap();
            state.rules.business_funding_rule.maximum_income = data.get("maximum_income").unwrap().as_f64().unwrap();
        },
        3 => {
            state.rules.deny_age_rule.maximum_age = (data.get("maximum_age").unwrap().as_i64().unwrap()) as i32;
        },
        4 => {
            state.rules.deny_health_percentage_rule.maximum_percentage = (data.get("maximum_percentage").unwrap().as_i64().unwrap()) as i32;
        },
        5 => {
            state.rules.cover_food_rule.people_count = (data.get("people_count").unwrap().as_i64().unwrap()) as i32;
            state.rules.cover_food_rule.maximum_salary = (data.get("maximum_salary").unwrap().as_i64().unwrap()) as i32;
        },
        6 => {
            state.rules.cover_food_unemployed_rule.people_count = (data.get("people_count").unwrap().as_i64().unwrap()) as i32;
        }
    }
}

pub fn update_app(app: App, payload: serde_json::Value, app_handle: &AppHandle) {
    let app_id = app as u8;
    app_handle.emit_all("update_app", json!({ "app_id": app_id, "data": payload })).unwrap();
}

#[tauri::command]
pub fn update_tax_rate(state_mux: State<'_, GameStateSafe>, tax_rate: i32) -> i64 {
    let mut state = state_mux.lock().unwrap();

    state.tax_rate = tax_rate as f32 / 100.;
    
    let mut total_income: i64 = 0;

    for per in state.people.values() {
        let tax_rate = Person::get_tax_rate(&state.rules.tax_rule, state.tax_rate, per.salary);
        total_income += ((per.salary as f32 / 12.) * tax_rate) as i64;
    }

    state.finance_data.expected_person_income = total_income / state.people.len() as i64;
    state.finance_data.expected_person_income
}

#[tauri::command]
pub fn update_business_tax_rate(state_mux: State<'_, GameStateSafe>, tax_rate: i32) -> i64 {
    let mut state = state_mux.lock().unwrap();

    state.business_tax_rate = tax_rate as f32 / 100.;

    let mut total_income: i64 = 0;

    for bus in state.businesses.values() {
        let tax_rate = Business::get_tax_rate(&state.rules.business_tax_rule, bus.last_month_income, state.business_tax_rate);
        total_income += (bus.last_month_income * tax_rate as f64) as i64;
    }

    state.finance_data.expected_business_income = total_income / state.businesses.len() as i64;
    state.finance_data.expected_business_income
}

#[tauri::command]
pub fn update_healthcare_budget(state_mux: State<'_, GameStateSafe>, new_budget: i64) -> serde_json::Value {
    let mut state = state_mux.lock().unwrap();

    let old_budget = state.healthcare.budget;

    state.healthcare.budget = new_budget;
    let spare_budget = state.get_spare_budget();

    if spare_budget < 0 {
        state.healthcare.budget = old_budget;
        return json!({
            "error": "Cannot afford this budget",
        });
    }

    state.spare_budget = spare_budget;
    state.healthcare.total_capacity = (new_budget as i64 / state.healthcare.cost_per_hospital_capacity as i64) as i32;

    json!({
        "used_hospital_capacity": state.healthcare.get_current_capacity(),
        "total_hospital_capacity": state.healthcare.total_capacity,
    })
}

#[tauri::command]
pub fn update_welfare_budget(state_mux: State<'_, GameStateSafe>, new_budget: i64) -> serde_json::Value {
    let mut state = state_mux.lock().unwrap();

    let old_budget = state.welfare_budget;

    state.welfare_budget = new_budget;
    let spare_budget = state.get_spare_budget();

    if spare_budget < 0 {
        state.welfare_budget = old_budget;
        return json!({
            "error": "Cannot afford this budget",
        })
    }

    state.spare_budget = spare_budget;

    json!({})
}

#[tauri::command]
pub fn update_business_budget(state_mux: State<'_, GameStateSafe>, new_budget: i64) -> serde_json::Value {
    let mut state = state_mux.lock().unwrap();

    let old_budget = state.business_budget;

    state.business_budget = new_budget;
    let spare_budget = state.get_spare_budget();

    if spare_budget < 0 {
        state.business_budget = old_budget;
        return json!({
            "error": "Cannot afford this budget",
        })
    }

    state.spare_budget = spare_budget;

    json!({})
}