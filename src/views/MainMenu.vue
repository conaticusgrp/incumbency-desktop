<script setup lang="ts">
  import { AppState, useAppStore } from "../store/app";
  import Logo from "../assets/logo.svg";

  const appStore = useAppStore();
  const goto = (state: AppState) => () => appStore.setState(state);
</script>

<template>
  <main>
    <img :src=Logo alt="Game logo">
    <!-- TODO(dylhack): use routes instead -->
    <button @click=goto(AppState.NewGameMenu)>New Game</button>
    <button @click=goto(AppState.LoadGameMenu)>Load Game</button>
    <button @click=goto(AppState.SettingsMenu)>Settings</button>
  </main>
</template>

<style scoped>
  main {
    /* position: relative; */
    width: 100%;
    height: 100%;
  }
  
  img {
    margin: 0 auto;
    display: block;
    width: 100%;
    max-width: 650px;
    max-height: 500px;
    margin-top: 4rem;
  }

  /* Note: The "disabled" class should probably be moved to app.css since it could be used in the other parts for the app. */
  .disabled {
    position: relative;
  }
  .disabled:after {
    content: '';
    position: absolute;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    background-color: rgba(0, 0, 0, 0.5);
  }
</style>
