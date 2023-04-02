<script lang="ts" context="module">
    export interface BusinessData {
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
    }
</script>

<script lang="ts">
    import { createEventDispatcher } from "svelte";
    import TabWindow from "../TabWindow.svelte";
    import type {
        CriticalWindowData,
        MonthlyGraphData,
    } from "../Window.svelte";

    import BusinessTabButton from "./BusinessTabButton.svelte";
    import Overview from "./Overview/Overview.svelte";
    import Rules from "./Rules/Rules.svelte";

    export let windowData: CriticalWindowData;

    let dispatcher = createEventDispatcher();
</script>

<TabWindow
    title="Business"
    pos={{ x: 100, y: 50 }}
    size={{ width: 800, height: 600 }}
    {windowData}
    tabButtonComponent={BusinessTabButton}
    tabButtonData={["Overview", "Rules"]}
    tabs={[
        { c: Overview, data: null },
        { c: Rules, data: null },
    ]}
    on:criticalWindowEvent={(e) => dispatcher("criticalWindowEvent", e.detail)}
/>
