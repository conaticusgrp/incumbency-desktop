<script lang="ts">

  import { AppState, appState } from "../../stores/appState";


</script>

<main>

  <p>load game</p>
  <button on:click={() => appState.set(AppState.MAIN_MENU)}>Back</button>

</main>

<style>

  main {
    position: relative;
    width: 100%;
    height: 100%;
  }

</style>
