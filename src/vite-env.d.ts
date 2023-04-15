/// <reference types="vite/client" />

declare module "*.vue" {
    import type { DefineComponent } from "vue";
    const component: DefineComponent<{}, {}, any>;
    export default component;
}

type CardData = {
    title: string;
    data: number;
    prefix: string;
    dataKey: string;
};

type CardGraphData<T> = {
    type: GraphData<T>["type_id"];
    title: string;
    historical: {
        actual: GraphData<T>;
        predicted?: GraphData<T>;
    };
};

type ProjectedGraphData = {
    labels: string[];
    actual: number[];
    predicted?: number[];
};

type EmailData = {
    title: string;
    content: string;
    sender: string;
    severity: Severity;
};

type GraphData<T> = {
    // 0 = Daily, 1 = Monthly
    type_id: 0 | 1;
    one_week: T[];
    one_month: T[];
    three_months: T[];
    six_months: T[];
    one_year: T[];
    three_years: T[];
};

type GraphDataWithLabel<T> = {
    label: string;
} & GraphData<T>;

type FinanceData = {
    average_monthly_income: number;
    business_budget: number;
    business_tax_rate: number;
    expected_business_income: number;
    expected_person_income: number;
    government_balance: number;
    healthcare_budget: number;
    welfare_budget: number;
    spare_budget: number;
    tax_rate: number;
    average_welfare: number;
    average_welfare_unemployed: number;
    rules: {
        business_tax: {
            enabled: boolean;
            minimum_monthly_income: number;
            tax_rate: number;
        };
        tax: { enabled: boolean; minimum_salary: number; tax_rate: number };
    };
    used_hospital_capacity: number;
    total_hospital_capacity: number;
    spare_hospital_capacity: number;
    used_welfare_budget: number;
    used_business_budget: number;
    expected_balance: number;
    government_balance_graph_data: GraphData<number>;
    government_balance_prediction_graph_data: GraphData<number>;
    average_monthly_income_graph_data: GraphData<number>;
    government_losses_graph_data: GraphData<number>;
};

type BusinessData = {
    average_employees: number;
    average_monthly_income: number;
    business_count: number;
    rules: {
        funding: {
            enabled: boolean;
            business_count: number;
            fund: number;
            maximum_income: number;
            budget_cost: number;
        };
    };
    business_budget: number;
    business_count_graph_data: MonthlyGraphData;
    average_employees_graph_data: MonthlyGraphData;
    average_monthly_income_graph_data: MonthlyGraphData;
};

type CareCapacity = {
    budget: number;
    current_capacity: number;
    total_capacity: number;
};

type HealthcareData = {
    age_ranges: {
        "0-18": number;
        "19-29": number;
        "30-44": number;
        "45-60": number;
        "61-84": number;
        "85+": number;
    };
    births_per_month: number;
    deaths_per_month: number;
    life_expectancy: number;
    population: number;
    used_capacity: number;
    total_capacity: number;
    rules: {
        deny_past_age: {
            enabled: boolean;
            maximum_age: number;
        };
        deny_past_health: {
            enabled: boolean;
            maximum_percentage: number;
        };
    };
    child_care: CareCapacity;
    adult_care: CareCapacity;
    elder_care: CareCapacity;
    population_graph_data: DailyGraphData;
    births_graph_data: DailyGraphData;
    deaths_graph_data: DailyGraphData;
    life_expectancy_graph_data: DailyGraphData;
    hospital_usage_capacity_graph_data: DailyGraphData;
};

type Filter =
    | "one_week"
    | "one_month"
    | "three_months"
    | "six_months"
    | "one_year"
    | "three_years";

type Pos = {
    x: number;
    y: number;
};

type Size = {
    width: number;
    height: number;
    maximized?: boolean;
};

type CriticalWindowData = {
    opened: boolean;
    focused: boolean;
    index: number;
};

// Rules from backend
type TaxRule = {
    enabled: boolean;
    minimum_salary: number;
    tax_rate: number;
};

type BusinessTaxRule = {
    enabled: boolean;
    minimum_monthly_income: number;
    tax_rate: number;
};

type BusinessFundingRule = {
    enabled: boolean;
    fund: number;
    maximum_income: number;
    business_count: number;
    budget_cost: number;
};

type DenyAgeRule = {
    enabled: boolean;
    maximum_age: number;
};

type DenyHealthPercentageRule = {
    enabled: boolean;
    maximum_percentage: number;
};

type CoverFoodRule = {
    enabled: boolean;
    people_count: number;
    maximum_salary: number;
    budget_cost: number;
};

type CoverFoodUnemployedRule = {
    enabled: boolean;
    people_count: number;
    budget_cost: number;
};

type GameStateRules = {
    tax_rule: TaxRule;
    business_tax_rule: BusinessTaxRule;
    business_funding_rule: BusinessFundingRule;
    deny_age_rule: DenyAgeRule;
    deny_health_percentage_rule: DenyHealthPercentageRule;
    cover_food_rule: CoverFoodRule;
    cover_food_unemployed_rule: CoverFoodUnemployedRule;
};

// Events from backend
type BusinessUpdatePayload = {
    average_employees: number;
    average_monthly_income: number;
    business_count_graph_data: GraphData<number>;
    average_employees_graph_data: GraphData<number>;
    average_monthly_income_graph_data: GraphData<number>;
};

type WelfareUpdatePayload = {
    unemployed_count_graph_data: GraphData<number>;
};

type HealthcarePayload = {
    life_expectancy: number;
};

type FinancePayload = {
    average_monthly_income: number;
    expected_person_income: number;
    expected_business_income: number;
    welfare_budget: number;
    business_budget: number;
    healthcare_budget: number;
    expected_balance: number;
    government_balance_graph_data: GraphData<number>;
    government_balance_prediction_graph_data: GraphData<number>;
    average_monthly_income_graph_data: GraphData<number>;
    government_losses_graph_data: GraphData<number>;
};

type UpdateAppEvent<T> = {
    app_id: number;
    update_type: string;
    data: T;
};

type UpdateAppPayloads =
    | BusinessUpdatePayload
    | WelfareUpdatePayload
    | HealthcarePayload
    | FinancePayload;
type UpdateAppEventTypes = UpdateAppEvent<AppUpdatePayloads>;

type FinanceAppOpenedPayload = {
    government_balance: number;
    average_monthly_income: number;
    expected_person_income: number;
    expected_business_income: number;
    used_hospital_capacity: number;
    total_hospital_capacity: number;
    spare_hospital_capacity: number;
    business_tax_rate: number;
    tax_rate: number;
    healthcare_budget: number;
    used_welfare_budget: number;
    welfare_budget: number;
    used_business_budget: number;
    business_budget: number;
    spare_budget: number;
    average_welfare: number;
    average_unemployed_welfare: number;
    expected_balance: number;
    rules: {
        tax: TaxRule;
        business_tax: BusinessTaxRule;
    };
    // NOTE(dylhack): Monthly data
    government_balance_graph_data: GraphData;
    government_balance_prediction_graph_data: GraphData;
    average_monthly_income_graph_data: GraphData;
    government_losses_graph_data: GraphData;
};

type HealthcareAppOpenedPayload = {
    population: number;
    births_per_month: number;
    deaths_per_months: number;
    life_expectancy: number;
    used_capacity: number;
    total_capacity: number;
    // TODO(dylhack): type hint this
    age_ranges: {};
    child_care: HealthcareGroup;
    adult_care: HealthcareGroup;
    elder_care: HealthcareGroup;
    // NOTE(dylhack): Daily data
    population_graph_data: GraphData;
    births_graph_data: GraphData;
    deaths_graph_data: GraphData;
    life_expectancy_graph_data: GraphData;
    hospital_usage_capacity_graph_data: GraphData;
};

type BusinessAppOpenedPayload = {
    business_count: number;
    average_employees: number;
    average_monthly_income: number;
    // NOTE(dylhack): Monthly data
    business_count_graph_data: GraphData;
    average_employees_graph_data: GraphData;
    average_monthly_income_graph_data: GraphData;
};

type OpenEvents =
    | FinanceAppOpenedPayload
    | HealthcareAppOpenedPayload
    | BusinessAppOpenedPayload;

enum Rules {
    Tax,
    BusinessTax,
    BusinessFunding,
    DenyAge,
    DenyHealthPercentage,
    CoverFood,
    CoverFoodUnemployed,
}
