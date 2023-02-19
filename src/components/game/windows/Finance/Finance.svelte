<script lang="ts">

  import { createEventDispatcher } from "svelte";
  import AppWindow from "../TabWindow.svelte";
  import type { CriticalWindowData } from "../Window.svelte";
  
  import FinanceTabButton from "./FinanceTabButton.svelte";
  import Overview from "./Overview.svelte";
  import Budgets from "./Budgets.svelte";
  import Rules from "./Rules.svelte";

  export let windowData: CriticalWindowData;

  let dispatcher = createEventDispatcher();
  let data: any = {};

</script>

<AppWindow
  title="Finance"
  pos={{ x: 100, y: 50 }}
  size={{ width: 800, height: 600 }}
  {windowData}
  tabButtonComponent={FinanceTabButton}
  tabButtonData={["Overview", "Budgets", "Rules"]}
  tabs={[
    { c: Overview, data },
    { c: Budgets, data },
    { c: Rules, data }
  ]}
  on:criticalWindowEvent={(e) => dispatcher('criticalWindowEvent', e.detail)}
/>
