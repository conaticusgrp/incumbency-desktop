<script lang="ts" context="module">
    export interface WelfareData {
        average_welfare: number;
        average_unemployed_welfare: number;
        rules: {
            cover_food: {
                enabled: boolean;
                budget_cost: number;
                maximum_salary: number;
                people_count: number;
            };
            cover_food_unemployed: {
                enabled: boolean;
                budget_cost: number;
                people_count: number;
            };
        };
    }
</script>

<script lang="ts">
    import { createEventDispatcher } from "svelte";
    import TabWindow from "../TabWindow.svelte";
    import type { CriticalWindowData } from "../Window.svelte";

    import WelfareTabButton from "./WelfareTabButton.svelte";
    import Overview from "./Overview/Overview.svelte";
    import Rules from "./Rules/Rules.svelte";

    export let windowData: CriticalWindowData;

    let dispatcher = createEventDispatcher();
</script>

<TabWindow
    title="Welfare"
    pos={{ x: 100, y: 50 }}
    size={{ width: 800, height: 600 }}
    {windowData}
    tabButtonComponent={WelfareTabButton}
    tabButtonData={["Overview", "Rules"]}
    tabs={[
        { c: Overview, data: null },
        { c: Rules, data: null },
    ]}
    on:criticalWindowEvent={(e) => dispatcher("criticalWindowEvent", e.detail)}
/>
