<script setup lang="ts">
import PredictedGraph from "./graph-card/PredictedGraph.vue";
import { conaticus } from "../../util/data";
import { ref, watch } from "vue";
import FilterSelect from "./graph-card/FilterSelect.vue";

const props = defineProps<{
    type: GraphData<number>["type_id"];
    history: CardGraphData<number>["historical"];
    title: string;
}>();
const filter = ref<Filter>(props.type === 0 ? "one_week" : "one_month");
const data = ref(
    conaticus(filter.value, props.history.actual, props.history.predicted)
);

watch(
    () => props.history,
    () => {
        data.value = conaticus(
            filter.value,
            props.history.actual,
            props.history.predicted
        );
    }
);
</script>

<template>
    <div class="container">
        <h2>{{ title }}</h2>
        <FilterSelect
            :type="type"
            :length="history.actual.three_years.length"
        ></FilterSelect>
        <!-- <RateAnnotation data={result.actual.data} /> -->
        <PredictedGraph :data="data" />
    </div>
</template>

<style scoped>
.container {
    margin-left: 20px;
    margin-top: 20px;
    margin-bottom: 50px;
    border: solid 1px var(--color-accent);
    padding: 20px;
}
</style>
