<script lang="ts" context="module">
    export interface DataItem {
        title: string;
        dataKey: keyof WelfareData;
        data: any;
        pinned?: boolean;
        suffix: string;
    }
</script>

<script lang="ts">
    import OverviewData from "../../templates/OverviewData.svelte";

    import type { WelfareData } from "../Welfare.svelte";

    export let data: WelfareData;

    let dataArray: DataItem[] = [
        {
            title: "Average Welfare",
            dataKey: "average_welfare",
            suffix: "%",
            data: null,
            pinned: false,
        },
        {
            title: "Average Unemployed Welfare",
            dataKey: "average_unemployed_welfare",
            suffix: "%",
            data: null,
            pinned: false,
        },
    ];

    $: dataArray.forEach((d, i) => {
        dataArray[i].data = `${data[d.dataKey]}${d.suffix}`;
    });
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
    }
</style>
