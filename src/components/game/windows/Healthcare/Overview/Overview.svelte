<script lang="ts" context="module">
    export interface DataItem {
        title: string;
        dataKey: keyof HealthcareData;
        data: any;
        pinned: boolean;
    }
</script>

<script lang="ts">
    import OverviewData from "../../templates/OverviewData.svelte";
    import type { HealthcareData } from "../Healthcare.svelte";

    export let data: HealthcareData;
    console.log(data);

    let dataArray: DataItem[] = [
        {
            title: "Population",
            dataKey: "population",
            data: null,
            pinned: false,
        },
        {
            title: "Births per month",
            dataKey: "births_per_month",
            data: null,
            pinned: false,
        },
        {
            title: "Deaths per month",
            dataKey: "deaths_per_month",
            data: null,
            pinned: false,
        },
        {
            title: "Life Expectancy",
            dataKey: "life_expectancy",
            data: null,
            pinned: false,
        },
        {
            title: "Capacity",
            dataKey: "used_capacity",
            data: null,
            pinned: false,
        },
    ];

    $: {
        dataArray.forEach((d, i) => {
            if (d.title === "Capacity") {
                dataArray[i].data = `${data.used_capacity}/${
                    data.total_capacity
                } (${Math.floor(
                    (data.used_capacity / data.total_capacity) * 100
                )}%)`;
                return;
            }

            dataArray[i].data = data[d.dataKey];
        });
    }
</script>

<main>
    {#each dataArray as item (item.title)}
        <OverviewData bind:dataArray title={item.title} data={item.data} />
    {/each}
</main>

<style>
    main {
        width: 100%;
        height: 100%;
        overflow-y: scroll;
    }

    main::-webkit-scrollbar {
        display: none;
    }
</style>
