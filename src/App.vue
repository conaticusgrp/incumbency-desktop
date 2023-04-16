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
import Desktop from "./views/singleplayer/Desktop.vue";
import { APPS } from "src/windows/index";
import { useAppStore } from "./store/apps";
import { listen } from "@tauri-apps/api/event";
import * as graphs from 'src/store/graphs';

const financeStore = graphs.useFinanceStore();
const businessStore = graphs.useBusinessStore();
const welfareStore = graphs.useWelfareStore();
const healthcareStore = graphs.useHealthcareStore();

enum App {
    Finance = 1,
    Healthcare = 2,
    Business = 3,
    Welfare = 4,
}

ChartJS.register(
    Title,
    Tooltip,
    Legend,
    LineElement,
    LinearScale,
    PointElement,
    CategoryScale
);

const appStore = useAppStore();

onMounted(() => {
    document.addEventListener("contextmenu", (event) => event.preventDefault());
    APPS.forEach((app) => appStore.registerApp(app));
});

listen<UpdateAppEventTypes>("update_app", (event) => {
    const data = event.payload.data;
    const app_id = event.payload.app_id;
    switch (app_id) {
        case App.Finance:
            financeStore.setGraphData(() => data);
            break;
        case App.Business:
            businessStore.setGraphData(() => data);
            break;
        case App.Welfare:
            welfareStore.setGraphData(() => data);
            break;
        case App.Healthcare:
            healthcareStore.setGraphData(() => data);
            break;
        default:
            console.error(`Unhandled app_id: ${app_id}`);
    }
});
</script>

<template>
    <!-- <div>
            <RouterView></RouterView>
        </div> -->
    <Desktop />
</template>

<style>
main {
    width: 100%;
    height: 100%;
}

@font-face {
    font-family: "Fira Code";
    src: url("./assets/Fira Code.ttf");
}

:root {
    /* variables */
    --color-bg: black;
    --color-accent: #6d9b14;
    --color-highlight: white;
    --color-shaded: rgba(255, 255, 255, 0.5);
    --color-critical: #ff5656;
    --app-border-radius: 0.5rem;

    /*  */

    font-family: "Fira Code";
    font-size: 14px;
    line-height: 24px;
    font-weight: 400;

    color: var(--color-highlight);
    background-color: var(--color-bg);

    font-synthesis: none;
    text-rendering: optimizeLegibility;
    -webkit-font-smoothing: antialiased;
    -moz-osx-font-smoothing: grayscale;
    -webkit-text-size-adjust: 100%;
    text-size-adjust: 100%;
}

* {
    margin: 0;
    padding: 0;

    -webkit-user-select: none;
    -ms-user-select: none;
    user-select: none;
}

body {
    margin: 0;
    display: flex;
    place-items: center;
    min-width: 320px;
    min-height: 100vh;
    overflow: hidden;
}

#app {
    /* max-width: 1280px; */
    margin: 0 auto;
    text-align: center;

    width: 100%;
    height: 100vh;
    /* overflow: hidden; */
}

h1 {
    font-size: 3.2em;
    line-height: 1.1;
}

button {
    border: none;
    border-radius: 0;
    background-color: transparent;
    color: var(--color-highlight);
    padding: 1em;
    cursor: pointer;
    font-family: "Fira Code";
}

button:disabled {
    cursor: default;
}
</style>
