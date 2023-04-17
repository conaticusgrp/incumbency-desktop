<script setup lang="ts">
import { useBusinessStore } from "src/store/graphs";
import { computed, ref } from "vue";
import GraphCard from "/src/components/cards/GraphCard.vue";

const graphStore = useBusinessStore();
const data = graphStore.graphData;

const businessCount = ref<CardGraphData<number>>({
    type: data.business_count_graph_data.type_id,
    title: "Business Count",
    historical: {
        actual: data.business_count_graph_data,
    },
});

const averageEmployees = ref<CardGraphData<number>>({
    type: data.average_employees_graph_data.type_id,
    title: "Average Employees",
    historical: {
        actual: data.average_employees_graph_data,
    },
});

graphStore.$subscribe((_, state) => {
    businessCount.value = {
        ...businessCount.value,
        historical: {
            actual: state.data.business_count_graph_data,
        },
    };

    averageEmployees.value = {
        ...averageEmployees.value,
        historical: {
            actual: state.data.average_employees_graph_data,
        },
    };
});
</script>

<template>
    <GraphCard
        :type="businessCount.type"
        :title="businessCount.title"
        :history="businessCount.historical"
    />

    <GraphCard
        :type="averageEmployees.type"
        :title="averageEmployees.title"
        :history="averageEmployees.historical"
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
