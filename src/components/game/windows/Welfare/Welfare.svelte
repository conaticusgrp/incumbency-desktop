<script lang="ts">
  import { createEventDispatcher } from "svelte";
  import Overview from "../Finance/Overview.svelte";
  import AppWindow from "../TabWindow.svelte";
  import type { CriticalWindowData } from "../Window.svelte";

  export let windowData: CriticalWindowData;
</script>

<AppWindow name="Welfare" windowData={windowData} tabs={{
  Overview: Overview,
}} dispatcher={createEventDispatcher()} />
