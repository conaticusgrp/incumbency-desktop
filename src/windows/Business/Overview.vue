<script setup lang="ts">
import { useBusinessStore } from "src/store/graphs";
import { computed } from "vue";
import GraphCard from "/src/components/cards/GraphCard.vue";

const graphStore = useBusinessStore();
const data = graphStore.graphData;

const businessCount = computed<CardGraphData<number>>(() => {
    return {
        type: data.business_count_graph_data.type_id,
        title: "Business Count",
        historical: {
            actual: data.business_count_graph_data,
        },
    }
});

const averageEmployees = computed<CardGraphData<number>>(() => { 
    return {
        type: data.average_employees_graph_data.type_id,
        title: "Average Employees",
        historical: {
            actual: data.average_employees_graph_data,
        },
    }
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
