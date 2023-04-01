<script lang="ts" context="module">
    import type { GraphData } from "../../Window.svelte";

    export interface DataItem {
        title: string;
        dataKey: keyof FinanceData;
        data: any;
        pinned?: boolean;
        prefix: string;
        historical: {
            actual: GraphData,
            predicted?: GraphData,
        }
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
            historical: {
                actual: data.government_balance_graph_data,
                predicted: data.government_balance_prediction_graph_data
            }
        },
        {
            title: "Average Monthly Income",
            prefix: "$",
            dataKey: "average_monthly_income",
            data: null,
            pinned: false,
            historical: {
                actual: data.average_monthly_income_graph_data,
            }
        },
        // {
        //     title: "Total Expected Government Income",
        //     dataKey: "expected_person_income",
        //     prefix: "$",
        //     data: null,
        //     pinned: false,
        //     historical: {
        //         actual: data.expected_person_income
        //     }
        // },
        // {
        //     title: "Expected Citizen Tax Income",
        //     dataKey: "expected_person_income",
        //     prefix: "$",
        //     data: null,
        //     pinned: false,
        //     historical: {
        //         type: HistoricalIntervals.MONTH,
        //         data: data.government_balance_prediction_graph_data
        //     }
        // },
        // {
        //     title: "Expected Business Tax Income",
        //     dataKey: "expected_business_income",
        //     prefix: "$",
        //     data: null,
        //     pinned: false,
        //     historical: {
        //         type: HistoricalIntervals.MONTH,
        //         data: data.government_balance_prediction_graph_data
        //     }
        // },
        // {
        //     title: "Spare Budget",
        //     dataKey: "spare_budget",
        //     prefix: "$",
        //     data: null,
        //     pinned: false,
        //     historical: {
        //         type: HistoricalIntervals.MONTH,
        //         data: data.government_balance_prediction_graph_data
        //     }
        // },
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
        <OverviewData bind:dataArray title={item.title} data={item.data} history={item.historical} />
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
