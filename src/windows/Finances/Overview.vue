<script setup lang="ts">
import GraphCard from "../../components/cards/GraphCard.vue";
import { useFinanceStore } from "../../store/graphs";
import { computed, ref } from "vue";

const graphStore = useFinanceStore();
const data = graphStore.graphData;

const governmentBalance = computed<CardGraphData<number>>(() => ({
    type: data.government_balance_graph_data.type_id,
    title: "Government Balance",
    historical: {
        actual: data.government_balance_graph_data,
        predicted: data.government_balance_prediction_graph_data,
    },
}));
const unemploymentRate = computed<CardGraphData<number>>(() => ({
    type: data.average_monthly_income_graph_data.type_id,
    title: "Average Monthly Income",
    historical: {
        actual: data.average_monthly_income_graph_data,
    },
}));
const governmentLosses = computed<CardGraphData<number>>(() => ({
    type: data.government_losses_graph_data.type_id,
    title: "Government Losses",
    historical: {
        actual: data.government_losses_graph_data,
    },
}));
</script>

<template>
    <GraphCard
        :type="governmentBalance.type"
        :title="governmentBalance.title"
        :history="governmentBalance.historical"
    ></GraphCard>
    <GraphCard
        :type="unemploymentRate.type"
        :title="unemploymentRate.title"
        :history="unemploymentRate.historical"
    ></GraphCard>
    <GraphCard
        :type="governmentLosses.type"
        :title="governmentLosses.title"
        :history="governmentLosses.historical"
    ></GraphCard>
</template>

<style scoped>
main {
    width: 100%;
    height: 100%;
    overflow-y: scroll;
}

main::-webkit-scrollbar {
    display: none;
}
</style>
