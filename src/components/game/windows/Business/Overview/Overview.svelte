<script lang="ts" context="module">
    export interface DataItem {
        title: string;
        dataKey: keyof BusinessData;
        data: any;
        pinned?: boolean;
        prefix?: string;
    }
</script>

<script lang="ts">
    import OverviewData from "../../templates/OverviewData.svelte";

    import type { BusinessData } from "../Business.svelte";

    export let data: BusinessData;

    let dataArray: DataItem[] = [
        {
            title: "Business Count",
            dataKey: "business_count",
            data: null,
            pinned: false,
        },
        {
            title: "Average Employees",
            dataKey: "average_employees",
            data: null,
            pinned: false,
        },
        {
            title: "Average Monthly Income",
            dataKey: "average_monthly_income",
            prefix: "$",
            data: null,
            pinned: false,
        },
    ];

</script>

<main>
    {#each dataArray as item (item.title)}
        <OverviewData title={item.title} history={item.data} />
    {/each}
</main>

<style>
    main {
        width: 100%;
        height: 100%;
    }
</style>
