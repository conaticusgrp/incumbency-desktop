<script lang="ts">
 import { appWindow } from "@tauri-apps/api/window";
 
 let fullscreen = true;

 const toggleFullscreen = async () => {
  appWindow.setFullscreen(! await appWindow.isFullscreen());
  fullscreen = await appWindow.isFullscreen();
 };
</script>

<main>
  <h1>Settings</h1>
  
  <label for="fullscreen">Fullscreen</label>
  <input bind:checked={fullscreen} on:change={toggleFullscreen} type="checkbox" name="Fullscreen" id="fullscreen">
</main>

<style>

</style>