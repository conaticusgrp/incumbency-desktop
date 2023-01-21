<script lang="ts">

  import Desktop from "../game/desktop/Desktop.svelte"
  import { invoke } from "@tauri-apps/api/tauri";

  invoke("create_game");

</script>

<main>
  <Desktop />

</main>

<style>

  main {
    position: relative;
    width: 100%;
    height: 100%;
  }

</style>
