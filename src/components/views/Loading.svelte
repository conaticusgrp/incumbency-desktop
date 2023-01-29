<script lang="ts">

  import { listen } from "@tauri-apps/api/event";
  import { appState } from "../../stores/appState";


  listen('game_generated', () => {
    appState.set('Loading');
  });

</script>

<main>

    <p>Loading...</p>

</main>

<style>

</style>
