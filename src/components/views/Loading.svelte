<script lang="ts">

  import { listen } from "@tauri-apps/api/event";
  import { appState } from "../../stores/appState";
  import { invoke } from "@tauri-apps/api/tauri";

  invoke("create_game");

  listen('game_generated', () => {
    appState.set('Singleplayer');
  });

</script>

<main>

    <p>Loading...</p>

</main>

<style>

</style>
