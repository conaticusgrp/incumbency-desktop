<script setup lang="ts">
import { ref } from "vue";

const props = defineProps({
    title: { type: String, required: true },
    data: { type: String, required: true },
});
</script>

<template>
    <div class="container">
        <div style="display: flex">
            <h1>{{ title.toUpperCase() }}</h1>
        </div>
        <h3>{{ data }}</h3>
    </div>
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
