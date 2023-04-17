<script setup lang="ts">
import OverviewCard from "src/components/cards/OverviewCard.vue";
import { useWelfareStore } from "src/store/graphs";
import { ref } from "vue";
import GraphCard from "/src/components/cards/GraphCard.vue";

const graphStore = useWelfareStore();
const data = graphStore.graphData;

const unemployedCount = ref<CardGraphData<number>>({
    type: data.unemployed_count_graph_data.type_id,
    title: "Unemployed People Count",
    historical: {
        actual: data.unemployed_count_graph_data,
    },
});

const averageWelfare = ref<CardGraphData<number>>({
    type: data.average_welfare_graph_data.type_id,
    title: "Average People Welfare",
    historical: {
        actual: data.average_welfare_graph_data,
    },
});

const averageUnemployedWelfare = ref<CardGraphData<number>>({
    type: data.average_unemployed_welfare_graph_data.type_id,
    title: "Average Unemployed People Welfare",
    historical: {
        actual: data.average_unemployed_welfare_graph_data,
    },
});

graphStore.$subscribe((_, state) => {
    unemployedCount.value = {
        ...unemployedCount.value,
        historical: {
            actual: state.data.unemployed_count_graph_data,
        },
    };

    averageWelfare.value = {
        ...averageWelfare.value,
        historical: {
            actual: state.data.average_welfare_graph_data,
        },
    };

    averageUnemployedWelfare.value = {
        ...averageUnemployedWelfare.value,
        historical: {
            actual: state.data.average_unemployed_welfare_graph_data,
        },
    };
});
</script>

<template>
    <GraphCard
        :type="unemployedCount.type"
        :title="unemployedCount.title"
        :history="unemployedCount.historical"
    />

    <GraphCard
        :type="averageWelfare.type"
        :title="averageWelfare.title"
        :history="averageWelfare.historical"
    />

    <GraphCard
        :type="averageUnemployedWelfare.type"
        :title="averageUnemployedWelfare.title"
        :history="averageUnemployedWelfare.historical"
    />
</template>

<style scoped>
h1 {
    font-size: 18px;
    font-weight: bold;
}
h3 {
    text-align: left;
    margin-top: 20px;
    font-size: 30px;
}
.btn {
    margin-right: 20px;
}
.container {
    margin-left: 20px;
    margin-top: 20px;
    margin-bottom: 50px;
    border: solid 1px var(--color-accent);
    padding: 20px;
}
</style>
