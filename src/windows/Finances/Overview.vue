<script setup lang="ts">
import GraphCard from "../../components/cards/GraphCard.vue";
import { useFinanceStore } from "../../store/graphs";
import { computed, ref } from "vue";

const graphStore = useFinanceStore();
const data = ref<FinanceData>(graphStore.$state.data);

const governmentBalance = computed<CardGraphData<number>>(() => ({
    type: data.value.government_balance_graph_data.type_id,
    title: "Government Balance",
    historical: {
        actual: data.value.government_balance_graph_data,
        predicted: data.value.government_balance_prediction_graph_data,
    },
}));
const unemploymentRate = computed<CardGraphData<number>>(() => ({
    type: data.value.average_monthly_income_graph_data.type_id,
    title: "Average Monthly Income",
    historical: {
        actual: data.value.average_monthly_income_graph_data,
    },
}));
const governmentLosses = computed<CardGraphData<number>>(() => ({
    type: data.value.government_losses_graph_data.type_id,
    title: "Government Losses",
    historical: {
        actual: data.value.government_losses_graph_data,
    },
}));

// expected_business_income: number;
// expected_person_income: number;
// expected_balance: number;

// average_welfare: number;
// average_welfare_unemployed: number;

// used_welfare_budget: number;
// used_business_budget: number;

graphStore.$subscribe((_, val) => (data.value = val.data));
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
