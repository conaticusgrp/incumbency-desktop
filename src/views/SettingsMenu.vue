<script setup lang="ts">
  import { appWindow } from "@tauri-apps/api/window";
  import { AppState, useAppStore } from "../store/app";
  import { ref } from "vue";

  const appStore = useAppStore();
  const volume = ref(0.5);
  const checkBox = ref<HTMLInputElement | null>(null);
  
  const isFullscreen = () => {
    if (checkBox.value) {
      return checkBox.value.checked;
    }
    return false;
  };
  const toggleFullscreen = async () => {
    appWindow.setFullscreen(isFullscreen());
  };
  const gotoMainMenu = () => appStore.setState(AppState.MainMenu);

</script>

<template>
  <main>
    <h2>Settings</h2>
    
    <label for="fullscreen">Fullscreen</label>
    <input v-model="checkBox" @change={toggleFullscreen} type="checkbox" name="Fullscreen" id="fullscreen">
    <label for="fullscreen">Volume</label>
    <input v-bind:value={volume} type="range" name="Volume" min="0" max="1" step="0.01">

    <button @click={gotoMainMenu}>Back</button>
  </main>
</template>

