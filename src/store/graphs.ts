import { defineStore } from 'pinia';

const initGraphData: GraphData<number> = {
  type_id: 0,
  one_week: [],
  one_month: [],
  three_months: [],
  six_months: [],
  one_year: [],
  three_years: [],
}

export const useGraphStore = defineStore('graphs', {
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
    }
  }
});

