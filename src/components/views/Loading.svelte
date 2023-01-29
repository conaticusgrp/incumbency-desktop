<script lang="ts">

  import { listen } from "@tauri-apps/api/event";
  import { appState } from "../../stores/appState";


  listen('game_generated', () => {
    console.log("loaded");
    appState.set('Singleplayer');
  });

</script>

<main>

    <p>Loading...</p>

</main>

<style>

</style>
