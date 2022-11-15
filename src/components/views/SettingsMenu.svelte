<script lang="ts">

  import { appWindow } from "@tauri-apps/api/window";
  import { AppState, appState } from "../../stores/appState";
  
  let fullscreen = true;

  const toggleFullscreen = async () => {
    appWindow.setFullscreen(! await appWindow.isFullscreen());
    fullscreen = await appWindow.isFullscreen();
  };

</script>

<main>

  <h2>Settings</h2>
  
  <label for="fullscreen">Fullscreen</label>
  <input bind:checked={fullscreen} on:change={toggleFullscreen} type="checkbox" name="Fullscreen" id="fullscreen">

  <button on:click={() => appState.set(AppState.MAIN_MENU)}>Back</button>
  <button on:click={() => appState.set(AppState.CREDITS)}>Credits</button>

</main>

<style>

</style>