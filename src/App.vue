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
import { ref, onMounted, DefineComponent, ComponentOptionsMixin, ExtractPropTypes } from "vue";
import { GameState, useGameStore } from "./store/game";
import LoadingGameVue from "./views/LoadingGame.vue";
import MainMenu from "./views/MainMenu.vue";
import SinglePlayer from "./views/SinglePlayer.vue";
import SettingsMenu from "./views/SettingsMenu.vue";

ChartJS.register(
    Title,
    Tooltip,
    Legend,
    LineElement,
    LinearScale,
    PointElement,
    CategoryScale
);

// TODO(dylhack): replace with "vue-router" (For some reason it doesn't during runtime.)
type Component = DefineComponent<{}, {}, {}, {}, {}, ComponentOptionsMixin, ComponentOptionsMixin, {}, string, {}, Readonly<ExtractPropTypes<{}>>, {}>
const appStore = useGameStore();
const component = ref<Component | null>(null);
const routes = new Map([
    [GameState.LoadGameMenu, LoadingGameVue],
    [GameState.MainMenu, MainMenu],
    [GameState.Singleplayer, SinglePlayer],
    [GameState.SettingsMenu, SettingsMenu],
]);

appStore.$subscribe((_, value) => {
    const { state } = value;
    const route = routes.get(state);
    if (!route) {
        console.error(`Impossible condition - no such app state: ${state}`);
        return;
    }
    component.value = route;
});

onMounted(() => {
    document.addEventListener("contextmenu", (event) => event.preventDefault());
});
</script>

<template>
    <div>
        <component :is="component" />
    </div>
</template>

<style scoped>
main {
    width: 100%;
    height: 100%;
}
</style>
