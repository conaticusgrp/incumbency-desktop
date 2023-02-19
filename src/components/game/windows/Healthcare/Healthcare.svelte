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
  tabButtonData={["Overview"]}
  tabs={[
    { c: Overview, data: null }
  ]}
  on:criticalWindowEvent={(e) => dispatcher('criticalWindowEvent', e.detail)}
/>
