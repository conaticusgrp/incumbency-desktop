<script lang="ts">
  
  import { createEventDispatcher } from "svelte";
  import type { CriticalWindowData } from "../../../scripts/criticalWindowData";
  import Window from "./Window.svelte"

  export let windowData: CriticalWindowData;

  let dispatcher = createEventDispatcher();

  let formatter = Intl.NumberFormat('en', { notation: 'compact' });

  let gdp = 1_500_000_000;;
  let population = 69_200_000;
  let taxRate = 20;
  let transport = 5_000_000;

</script>

<Window
  title="Budget Panel"
  pos={{ x: 100, y: 50 }}
  size={{ width: 800, height: 600 }}
  {windowData}
  on:criticalWindowEvent={(e) => dispatcher('criticalWindowEvent', e.detail)}
>
  <main>

    <h2>GDP</h2>
    <p>{formatter.format(gdp)}</p>

    <h2>Population</h2>
    <p>{formatter.format(population)}</p>

    <h2>Tax rate</h2>
    <p>{taxRate.toFixed(1)}%</p>

    <h2>Transport</h2>
    <p>{formatter.format(transport)}/yr</p>

  </main>
</Window>

<style>

  main {
    width: 100%;
    height: 100%;
  }

</style>
