<script lang="ts" context="module">
    export interface DataItem {
        title: string;
        data: any;
        pinned: boolean;
    }
</script>

<script lang="ts">
    import type { FinanceData } from "../Finance.svelte";
    import OverviewData from "../../templates/OverviewData.svelte";

    export let data: FinanceData;

    let dataArray: DataItem[] = [];

    $: dataArray = [
      {
        title: "Government Balance",
        data: `$${data.government_balance}`,
        pinned: false,
      },
      {
        title: "Average Monthly Income",
        data: `$${data.average_monthly_income}`,
        pinned: false,
      },
      {
        title: "Expected Tax Income",
        data: `$${data.expected_person_income}`,
        pinned: false,
      },
      {
        title: "Expected Business Tax Income",
        data: `$${data.expected_business_income}`,
        pinned: false,
      },
      {
        title: "Total Expected Income",
        data: `$${data.expected_person_income + data.expected_business_income}`,
        pinned: false,
      },
      {
        title: "Spare Budget",
        data: `$${data?.spare_budget}`,
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