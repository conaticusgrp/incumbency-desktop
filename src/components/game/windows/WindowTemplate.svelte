<!-- This component is a template for windows -->
<!-- How to add a new app: -->
<!-- 1. Copy-paste this file and change its name -->
<!-- 2. Add the component to [apps] in the Dekstop component -->

<script lang="ts">
  
  import { createEventDispatcher } from "svelte";
  import Window from "./Window.svelte"

  export let opened: boolean;
  export let focused: boolean;

  let dispatcher = createEventDispatcher();

</script>

<Window
  title="WindowTemplate"
  pos={{ x: 100, y: 50 }}
  size={{ width: 800, height: 600 }}
  {opened}
  {focused}
  on:criticalWindowEvent={(e) => dispatcher('criticalWindowEvent', e.detail)}
>
  <p>Window template</p>
  <!-- Write the HTML for the window here -->
</Window>

<style>

</style>
