<!-- This component is a template for windows -->
<!-- How to add a new app: -->
<!-- 1. Copy-paste this file and change its name -->
<!-- 2. Add the component to [apps] in the Dekstop component -->
<!-- Don't forget to check if all the import statements are correct! -->

<script lang="ts">
  
  import { createEventDispatcher } from "svelte";
  import Window, { type CriticalWindowData } from "../Window.svelte"

  export let windowData: CriticalWindowData;

  let dispatcher = createEventDispatcher();

</script>

<Window
  title="WindowTemplate"
  pos={{ x: 100, y: 50 }}
  size={{ width: 800, height: 600 }}
  {windowData}
  on:criticalWindowEvent={(e) => dispatcher('criticalWindowEvent', e.detail)}
>
  <p>Window template</p>
  <!-- Write the HTML for the window here -->
</Window>

<style>

</style>
