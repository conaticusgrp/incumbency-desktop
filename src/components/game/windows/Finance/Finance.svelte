<script lang="ts" context="module">
    export interface FinanceData {
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
    }
</script>

<script lang="ts">
    import { createEventDispatcher } from "svelte";
    import type { CriticalWindowData } from "../Window.svelte";

    import FinanceTabButton from "./FinanceTabButton.svelte";
    import Overview from "./Overview/Overview.svelte";
    import Budgets from "./Budgets/Budgets.svelte";
    import Rules from "./Rules/Rules.svelte";
    import TabWindow from "../TabWindow.svelte";

    export let windowData: CriticalWindowData;

    let dispatcher = createEventDispatcher();
</script>

<TabWindow
    title="Finance"
    pos={{ x: 100, y: 50 }}
    size={{ width: 800, height: 600 }}
    {windowData}
    tabButtonComponent={FinanceTabButton}
    tabButtonData={["Overview", "Budgets", "Rules"]}
    tabs={[
        { c: Overview, data: {} },
        { c: Budgets, data: {} },
        { c: Rules, data: {} },
    ]}
    on:criticalWindowEvent={(e) => dispatcher("criticalWindowEvent", e.detail)}
/>
