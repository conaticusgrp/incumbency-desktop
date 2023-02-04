<script lang="ts">
  
  import { createEventDispatcher } from "svelte";
  import Window, { type CriticalWindowData } from "./Window.svelte"

  export let windowData: CriticalWindowData;

  let dispatcher = createEventDispatcher();

</script>

<Window
  title="Welfare"
  pos={{ x: 100, y: 50 }}
  size={{ width: 800, height: 600 }}
  {windowData}
  on:criticalWindowEvent={(e) => dispatcher('criticalWindowEvent', e.detail)}
>
  
  <p>Welfare</p>

</Window>

<style>

</style>
