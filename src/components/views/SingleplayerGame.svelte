<script lang="ts">

  import { listen } from "@tauri-apps/api/event";
  import { invoke } from "@tauri-apps/api/tauri";
  import Desktop from "../game/desktop/Desktop.svelte"
  import Loading from "../game/desktop/Loading.svelte";

  let gameGenerated = false;
  
  listen('game_generated', () => {
    gameGenerated = true;
  });

  invoke("create_game");
  
</script>

<main>

  {#if !gameGenerated}

  <Loading />

  {/if}
  
  <Desktop />

</main>

<style>

  main {
    position: relative;
    width: 100%;
    height: 100%;
  }

</style>
