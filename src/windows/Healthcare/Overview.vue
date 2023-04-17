<script setup lang="ts">
import OverviewCard from "src/components/cards/OverviewCard.vue";
import { useHealthcareStore } from "src/store/graphs";

const graphStore = useHealthcareStore();
const data = graphStore.graphData;
</script>

<template>
    <div v-for="(value, key) in data">
        <OverviewCard :key-name="key" :value="value" />
    </div>
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
