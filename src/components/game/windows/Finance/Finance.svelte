<script lang="ts">

  import { createEventDispatcher } from "svelte";
  import AppWindow from "../TabWindow.svelte";
  import type { CriticalWindowData } from "../Window.svelte";
  
  import Overview from "./Overview.svelte";
  import Budgets from "./Budgets.svelte";
  import Rules from "./Rules.svelte";

  export let windowData: CriticalWindowData;

</script>

<AppWindow
  title="Finance"
  {windowData}
  tabs={{
    Overview: Overview,
    Budgets: Budgets,
    Rules: Rules,
  }}
  dispatcher={createEventDispatcher()}
/>