<script lang="ts">
  import { createEventDispatcher } from "svelte";
  import Overview from "../Finance/Overview.svelte";
  import AppWindow from "../utility/AppWindow.svelte";
  import type { CriticalWindowData } from "../Window.svelte";

  export let windowData: CriticalWindowData;
</script>

<AppWindow name="Healthcare" windowData={windowData} tabs={{
  Overview: Overview,
}} dispatcher={createEventDispatcher()} />