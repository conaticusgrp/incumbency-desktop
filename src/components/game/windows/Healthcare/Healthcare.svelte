<script lang="ts" context="module">
  export interface HealthcareData {
     age_ranges: {
      "0-18": number;
      "19-29": number;
      "30-44": number;
      "45-60": number;
      "61-84": number;
      "85+": number;
     },
     births_per_month: number;
     deaths_per_month: number;
     life_expectancy: number;
     population: number;
     used_capacity: number;
     total_capacity: number;
  }
</script>

<script lang="ts">
  
  import { createEventDispatcher } from "svelte";
  import TabWindow from "../TabWindow.svelte";
  import type { CriticalWindowData } from "../Window.svelte";
  
  import HealthcareTabButton from "./HealthcareTabButton.svelte";
  import Overview from "./Overview.svelte";
  
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
  ]}
  on:criticalWindowEvent={(e) => dispatcher('criticalWindowEvent', e.detail)}
/>
