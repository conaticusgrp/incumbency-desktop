import { defineStore } from "pinia";

const initGraphData: GraphData<number> = {
    type_id: 0,
    one_week: [],
    one_month: [],
    three_months: [],
    six_months: [],
    one_year: [],
    three_years: [],
};

export const useFinanceStore = defineStore("finance-store", {
    state: () => ({
        data: {
            average_monthly_income: -1,
            business_budget: -1,
            business_tax_rate: -1,
            expected_business_income: -1,
            expected_person_income: -1,
            government_balance: -1,
            healthcare_budget: -1,
            welfare_budget: -1,
            spare_budget: -1,
            tax_rate: -1,
            average_welfare: -1,
            average_welfare_unemployed: -1,
            rules: {
                business_tax: {
                    enabled: true,
                    minimum_monthly_income: -1,
                    tax_rate: -1,
                },
                tax: { enabled: true, minimum_salary: -1, tax_rate: -1 },
            },
            used_hospital_capacity: -1,
            total_hospital_capacity: -1,
            spare_hospital_capacity: -1,
            used_welfare_budget: -1,
            used_business_budget: -1,
            expected_balance: -1,
            government_balance_graph_data: initGraphData,
            government_balance_prediction_graph_data: initGraphData,
            average_monthly_income_graph_data: initGraphData,
            government_losses_graph_data: initGraphData,
        } as FinanceData,
    }),
    actions: {
        setGraphData(cb: (data: FinanceData) => FinanceData) {
            this.data = cb(this.data);
        },
    },
    getters: {
        graphData(state) {
            return state.data;
        },
    },
});

export const useBusinessStore = defineStore("business-store", {
    state: () => ({
        data: {
            average_employees: -1,
            average_monthly_income: -1,
            business_count: -1,
            rules: {
                funding: {
                    enabled: true,
                    business_count: -1,
                    fund: -1,
                    maximum_income: -1,
                    budget_cost: -1,
                },
            },
            business_budget: -1,
            business_count_graph_data: initGraphData,
            average_employees_graph_data: initGraphData,
            average_monthly_income_graph_data: initGraphData,
        } as BusinessData,
    }),
    actions: {
        setGraphData(cb: (data: BusinessData) => BusinessData) {
            this.data = cb(this.data);
        },
    },
    getters: {
        graphData(state) {
            return state.data;
        }
    }
});

const defaultHealthcareCapacity: CareCapacity = {
    budget: -1,
    current_capacity: -1,
    total_capacity: -1,
};

export const useHealthcareStore = defineStore("healthcare-store", {
    state: () => ({
        data: {
            age_ranges: {
                "0-18": -1,
                "19-29": -1,
                "30-44": -1,
                "45-60": -1,
                "61-84": -1,
                "85+": -1,
            },
            births_per_month: -1,
            deaths_per_month: -1,
            life_expectancy: -1,
            population: -1, // why tf do i have to type these out, i cry ;(
            used_capacity: -1,
            total_capacity: -1,
            rules: {
                deny_past_age: {
                    enabled: true,
                    maximum_age: -1,
                },
                deny_past_health: {
                    enabled: true,
                    maximum_percentage: -1,
                },
            },
            child_care: defaultHealthcareCapacity,
            adult_care: defaultHealthcareCapacity,
            elder_care: defaultHealthcareCapacity,
            population_graph_data: initGraphData,
            births_graph_data: initGraphData, // beepboop(conaticus): idk if we want this for both daily and monthly data
            deaths_graph_data: initGraphData,
            life_expectancy_graph_data: initGraphData,
            hospital_usage_capacity_graph_data: initGraphData,
        } as HealthcareData,
    }),
    actions: {
        setGraphData(cb: (data: HealthcareData) => HealthcareData) {
            this.data = cb(this.data);
        },
    },
    getters: {
        graphData(state) {
            return state.data;
        }
    },
});

export const useWelfareStore = defineStore("welfare-store", {
    state: () => ({
        data: {
            average_welfare: -1,
            average_unemployed_welfare: -1,
            welfare_budget: -1,
            unemployed_count: -1,
            rules: {
                cover_food: {
                    enabled: true,

                    budget_cost: -1,
                    maximum_salary: -1,
                    people_count: -1,
                },
                cover_food_unemployed: {
                    enabled: true,
                    budget_cost: -1,
                    people_count: -1,
                },
            },
            unemployed_count_graph_data: initGraphData,
            average_welfare_graph_data: initGraphData,
            average_unemployed_welfare_graph_data: initGraphData,
        },
    }),
    actions: {
        setGraphData(cb: (data: WelfareData) => WelfareData) {
            this.data = cb(this.data);
        },
    },
    getters: {
        graphData(state) {
            return state.data;
        }
    }
});
