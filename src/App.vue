<script setup lang="ts">
import {
    Chart as ChartJS,
    Title,
    Tooltip,
    Legend,
    LineElement,
    LinearScale,
    PointElement,
    CategoryScale,
} from "chart.js";
import { onMounted } from "vue";

ChartJS.register(
    Title,
    Tooltip,
    Legend,
    LineElement,
    LinearScale,
    PointElement,
    CategoryScale
);


onMounted(() => {
    document.addEventListener("contextmenu", (event) => event.preventDefault());
});
</script>

<template>
    <div>
        <RouterView></RouterView>
    </div>
</template>

<style scoped>
main {
    width: 100%;
    height: 100%;
}
</style>
