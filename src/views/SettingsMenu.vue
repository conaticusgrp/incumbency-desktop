<script setup lang="ts">
import { appWindow } from "@tauri-apps/api/window";
import { ref } from "vue";
import { useRouter } from "vue-router";

const router = useRouter()

const volume = ref(0.5);
const isFullscreen = ref(await appWindow.isFullscreen());

const toggleFullscreen = async () => await appWindow.setFullscreen(!isFullscreen);
const gotoMainMenu = () => router.push({ name: 'main-menu' })
</script>

<template>
  <main>
    <h2>Settings</h2>

    <label for="fullscreen">Fullscreen</label>
    <input :checked="isFullscreen" @change=toggleFullscreen() type="checkbox" name="Fullscreen" id="fullscreen">
    <label for="fullscreen">Volume</label>
    <input v-bind:value=volume type="range" name="Volume" min="0" max="1" step="0.01">
    <button @click=gotoMainMenu()>Back</button>

  </main>
</template>

