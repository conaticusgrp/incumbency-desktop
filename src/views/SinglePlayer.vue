<script setup lang="ts">
import { listen } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/tauri";
import { onMounted, ref } from "vue";
import { useGameStore } from "../store/game";
import Desktop from "./singleplayer/Desktop.vue";
import Loading from "./singleplayer/Loading.vue";

const gameStore = useGameStore();
const gameGenerated = ref(gameStore.generated);

// The invoke call is in onMount because the backend could potentially instantly create the game,
// which will lead to other frontend events ignored
onMounted(async () => {
  const unlisten = await listen("game_generated", () => {
    gameStore.setIsReady();
    gameGenerated.value = true;
    unlisten();
  });

  await invoke('create_game')
  unlisten();
});
</script>

<template>
  <main>
    <Loading v-if="!gameGenerated" />
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
