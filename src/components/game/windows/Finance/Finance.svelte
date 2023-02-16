<script lang="ts">
  import { createEventDispatcher } from "svelte";
  import AppWindow from "../utility/AppWindow.svelte";
  import type { CriticalWindowData } from "../Window.svelte";
  import Budgets from "./Budgets.svelte";
  import Overview from "./Overview.svelte";
  import Rules from "./Rules.svelte";

  export let windowData: CriticalWindowData;
</script>

<AppWindow name="Finance" windowData={windowData} tabs={{
  Overview: Overview,
  Budgets: Budgets,
  Rules: Rules,
}} dispatcher={createEventDispatcher()} />