<script setup lang="ts">
  import { GameState, useGameStore } from "src/store/game";
  import { invoke } from "@tauri-apps/api/tauri";
  import { ref } from "vue";

  const appStore = useGameStore();
  const name = ref("");
  const saveAlreadyExists = ref(false);

  const checkAlreadyExists = async () => {
    saveAlreadyExists.value = await invoke<boolean>("check_save_exists", { name });
  }

  const createGame = async () => {
    if (name.value.length === 0 || saveAlreadyExists) return;
    await invoke("create_game", { name });
  } 
</script>

<template>
  <main>
    <h2>New Game</h2>
    <p v-if="saveAlreadyExists" style="color: red">Save with name '{name}' already exists.</p>

    <div class="save_name">
      <label for="name">Enter save name: </label>
      <input type="text" required bind:value={name} min="1" max="30" @keyup={checkAlreadyExists} />
    </div>

    <div>
      <button @click={appStore.goto(GameState.MainMenu)}>Back</button>
      <button @click={createGame} type="submit">Create!</button>
    </div>

  </main>
</template>

<style scoped>

  main {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    
    position: absolute;
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);
    width: 100%;
    height: 100%;
  }

</style>
