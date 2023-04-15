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

export const useFinanceStore = defineStore("graphs", {
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
        setGraphData(data: FinanceData) {
            this.data = data;
        },
    },
});

export const useBusinessStore = defineStore("graphs", {
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
        setGraphData(data: BusinessData) {
            this.data = data;
        },
    },
});
