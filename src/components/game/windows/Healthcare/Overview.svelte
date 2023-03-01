<script lang="ts" context="module">
    export interface DataItem {
        title: string;
        data: any;
        pinned: boolean;
    }
</script>

<script lang="ts">
    import OverviewData from "../templates/OverviewData.svelte";
    import type { HealthcareData } from "./Healthcare.svelte";

    export let data: HealthcareData;

    let dataArray: DataItem[] = [];

    $: dataArray = [
      {
        title: "Population",
        data: data.population,
        pinned: false,
      },
      {
        title: "Births per month",
        data: data.births_per_month,
        pinned: false,
      },
      {
        title: "Deaths per month",
        data: data.deaths_per_month,
        pinned: false,
      },
      {
        title: "Life Expectancy",
        data: data.life_expectancy,
        pinned: false,
      },
      {
        title: "Capacity",
        data: `${data.used_capacity}/${data.total_capacity} (${Math.floor((data.used_capacity / data.total_capacity) * 100)}%)`,
        pinned: false,
      },
    ];
</script>

<main>
    {#each dataArray as item (item.title)}
      <OverviewData bind:dataArray={dataArray} title={item.title} data={item.data} />
    {/each}
</main>
  
<style>
  main {
    width: 100%;
    height: 100%;
  }
</style>