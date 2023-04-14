<script setup lang="ts">
import { listen } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/tauri";
import { onMounted, ref } from "vue";
import LoadingGame from "./LoadingGame.vue";
import Desktop from "./singleplayer/Desktop.vue";
import Loading from "./singleplayer/Loading.vue";

const gameGenerated = ref(false);
const unlisten = await listen("game_generated", () => {
  gameGenerated.value = true;
  unlisten();
});

// The invoke call is in onMount because the backend could potentially instantly create the game,
// which will lead to other frontend events ignored
onMounted(async () => {
  await invoke('create_game')
});
</script>

<template>
  <main>
    <LoadingGame v-if="!gameGenerated" />
    <Desktop />
  </main>
</template>

<style scoped>
main {
  position: relative;
  width: 100%;
  height: 100%;
}
</style>
