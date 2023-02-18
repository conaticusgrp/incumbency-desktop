<script lang="ts">

  import { createEventDispatcher } from "svelte";
  import TabWindow from "../TabWindow.svelte";
  import type { CriticalWindowData } from "../Window.svelte";

  import WelfareTabButton from "./WelfareTabButton.svelte";
  import Overview from "./Overview.svelte";

  export let windowData: CriticalWindowData;

  let dispatcher = createEventDispatcher();

</script>

<TabWindow
  title="Welfare"
  pos={{ x: 100, y: 50 }}
  size={{ width: 800, height: 600 }}
  {windowData}
  tabButtonComponent={WelfareTabButton}
  tabButtonData={["Overview"]}
  tabs={[
    { c: Overview, data: null }
  ]}
  on:criticalWindowEvent={(e) => dispatcher('criticalWindowEvent', e.detail)}
/>
