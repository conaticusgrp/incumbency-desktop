<script lang="ts" context="module">
    export interface DataItem {
        title: string;
        dataKey: keyof FinanceData;
        data: any;
        pinned?: boolean;
        prefix: string;
    }
</script>

<script lang="ts">
    import type { FinanceData } from "../Finance.svelte";
    import OverviewData from "../../templates/OverviewData.svelte";

    export let data: FinanceData;

    let dataArray: DataItem[] = [
        {
            title: "Government Balance",
            prefix: "$",
            dataKey: "government_balance",
            data: null,
            pinned: false,
        },
        {
            title: "Average Monthly Income",
            prefix: "$",
            dataKey: "average_monthly_income",
            data: null,
            pinned: false,
        },
        {
            title: "Expected Balance",
            dataKey: "expected_balance",
            prefix: "$",
            data: null,
            pinned: false,
        },
        {
            title: "Total Expected Income",
            dataKey: "expected_person_income",
            prefix: "$",
            data: null,
            pinned: false,
        },
        {
            title: "Expected Tax Income",
            dataKey: "expected_person_income",
            prefix: "$",
            data: null,
            pinned: false,
        },
        {
            title: "Expected Business Tax Income",
            dataKey: "expected_business_income",
            prefix: "$",
            data: null,
            pinned: false,
        },
        {
            title: "Spare Budget",
            dataKey: "spare_budget",
            prefix: "$",
            data: null,
            pinned: false,
        },
    ];

    $: dataArray.forEach((d, i) => {
        if (d.title === "Total Expected Income") {
            dataArray[i].data = `${d.prefix}${
                data.expected_person_income + data.expected_business_income
            }`;
            return;
        }

        dataArray[i].data = `${d.prefix ? d.prefix : ""}${data[d.dataKey]}`;
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
        overflow-y: scroll;
    }

    main::-webkit-scrollbar {
        display: none;
    }
</style>
