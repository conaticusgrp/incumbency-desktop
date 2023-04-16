<script setup lang="ts">
import { ref } from 'vue';
import { GameState, useGameStore } from 'src/store/game';

const saves = ref<string[]>([]);
const appStore = useGameStore();

const clickSave = () => {
  appStore.goto(GameState.Singleplayer);
}

const gotoMainMenu = () => {
  appStore.goto(GameState.MainMenu);
}

</script>

<template>
  <main>
    <div class="center" style="height: 100%; flex-direction: column;">
      <h2>Load Game</h2>
      <ul class="horizontal_scroll">
        <!-- change this to open game view -->
        <div v-for="save, i in saves" :key="i" class="save_card" @mousedown="clickSave">
          {{ save }}
        </div>
      </ul>

    </div>
    <button class="button_back" @click="gotoMainMenu">Back</button>
  </main>
</template>

<style scoped>
::-webkit-scrollbar {
  background: #1a1a1a30;
  border-radius: 2rem;
}

::-webkit-scrollbar-thumb {
  background: #1a1a1a;
  border-radius: 2rem;
}

main {
  position: relative;
  width: 100%;
  height: 100%;
}

.horizontal_scroll {
  height: 50%;
  width: 100%;
  display: flex;
  align-items: center;

  overflow: auto;
  white-space: nowrap;
}

.save_card {
  flex-grow: 0;
  flex-shrink: 0;
  background-color: #1a1a1a;
  border-radius: var(--app-border-radius);

  margin: 0.1rem 1rem;
  width: 20rem;
  height: 25rem;

  /* in case we add cover images */
  mask: linear-gradient(to bottom, rgba(0, 0, 0, 1) 0%, rgba(0, 0, 0, 0) 99%);
}

.save_card:hover {
  filter: brightness(0.8);
}

.button_back {
  position: absolute;
  bottom: 0.5rem;
  left: 50%;
  transform: translate(-50%, 0);
}
</style>
