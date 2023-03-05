<script lang="ts" context="module">
    interface CareCapacity {
        budget: number;
        current_capacity: number;
        total_capacity: number;
    }

    export interface HealthcareData {
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
    }
</script>

<script lang="ts">
    import { createEventDispatcher } from "svelte";
    import TabWindow from "../TabWindow.svelte";
    import type { CriticalWindowData } from "../Window.svelte";
    import Capacity from "./Capacity/Capacity.svelte";

    import HealthcareTabButton from "./HealthcareTabButton.svelte";
    import Overview from "./Overview/Overview.svelte";
    import Rules from "./Rules/Rules.svelte";

    export let windowData: CriticalWindowData;

    let dispatcher = createEventDispatcher();
</script>

<TabWindow
    title="Healthcare"
    pos={{ x: 100, y: 50 }}
    size={{ width: 800, height: 600 }}
    {windowData}
    tabButtonComponent={HealthcareTabButton}
    tabButtonData={["Overview", "Capacity", "Rules"]}
    tabs={[
        { c: Overview, data: null },
        { c: Capacity, data: null },
        { c: Rules, data: null },
    ]}
    on:criticalWindowEvent={(e) => dispatcher("criticalWindowEvent", e.detail)}
/>
