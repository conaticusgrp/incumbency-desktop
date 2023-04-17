<script setup lang="ts">
import { useHealthcareStore } from "src/store/graphs";
import { ref } from "vue";
import GraphCard from "/src/components/cards/GraphCard.vue";

const graphStore = useHealthcareStore();
const data = graphStore.graphData;

const population = ref<CardGraphData<number>>({
    type: data.population_graph_data.type_id,
    title: "Population",
    historical: {
        actual: data.population_graph_data,
    },
});

const birthRate = ref<CardGraphData<number>>({
    type: data.births_graph_data.type_id,
    title: "Birth Rate",
    historical: {
        actual: data.births_graph_data,
    },
});

const deathRate = ref<CardGraphData<number>>({
    type: data.deaths_graph_data.type_id,
    title: "Death Rate",
    historical: {
        actual: data.deaths_graph_data,
    },
});

const lifeExpectancy = ref<CardGraphData<number>>({
    type: data.life_expectancy_graph_data.type_id,
    title: "Life Expectancy",
    historical: {
        actual: data.life_expectancy_graph_data,
    },
});

const hospitalCapacity = ref<CardGraphData<number>>({
    type: data.hospital_usage_capacity_graph_data.type_id,
    title: "Hospital Usage",
    historical: {
        actual: data.hospital_usage_capacity_graph_data,
    },
});

graphStore.$subscribe((_, state) => {
    population.value = {
        ...population.value,
        historical: {
            actual: state.data.population_graph_data,
        },
    };

    birthRate.value = {
        ...birthRate.value,
        historical: {
            actual: state.data.births_graph_data,
        },
    };

    deathRate.value = {
        ...deathRate.value,
        historical: {
            actual: state.data.deaths_graph_data,
        },
    };

    lifeExpectancy.value = {
        ...lifeExpectancy.value,
        historical: {
            actual: state.data.life_expectancy_graph_data,
        },
    };

    hospitalCapacity.value = {
        ...hospitalCapacity.value,
        historical: {
            actual: state.data.hospital_usage_capacity_graph_data,
        },
    };
});
</script>
<template>
    <GraphCard
        :type="population.type"
        :title="population.title"
        :history="population.historical"
    />
    <GraphCard
        :type="birthRate.type"
        :title="birthRate.title"
        :history="birthRate.historical"
    />
    <GraphCard
        :type="deathRate.type"
        :title="deathRate.title"
        :history="deathRate.historical"
    />
    <GraphCard
        :type="lifeExpectancy.type"
        :title="lifeExpectancy.title"
        :history="lifeExpectancy.historical"
    />
    <GraphCard
        :type="hospitalCapacity.type"
        :title="hospitalCapacity.title"
        :history="hospitalCapacity.historical"
    />
</template>

<!-- This style is used many times across overview tabs and idk how to make it global -->
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
