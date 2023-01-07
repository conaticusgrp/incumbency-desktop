<script lang="ts">

  import { appWindow } from "@tauri-apps/api/window";
  import { appState } from "../../stores/appState";
  
  let fullscreen = false; // TODO: Make this true by default when settings are implemented
  let volume = 0.5;
  
  
  // Adjust volume
  $: if (volume >= 0 && volume <= 1) {
    // Adjust volume
    console.log(`Volume: ${volume}`);
  }

  const toggleFullscreen = async () => {
    const newVal = ! await appWindow.isFullscreen();
    appWindow.setFullscreen(newVal);
    fullscreen = newVal;
  };

</script>

<main>

  <h2>Settings</h2>
  
  <label for="fullscreen">Fullscreen</label>
  <input bind:checked={fullscreen} on:change={toggleFullscreen} type="checkbox" name="Fullscreen" id="fullscreen">
  <label for="fullscreen">Volume</label>
  <input bind:value={volume} type="range" name="Volume" min="0" max="1" step="0.01">

  <button on:click={() => appState.set('MainMenu')}>Back</button>
  <button on:click={() => appState.set('Credits')}>Credits</button>

</main>

<style>

</style>