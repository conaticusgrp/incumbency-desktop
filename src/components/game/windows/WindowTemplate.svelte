<!-- This component is a template for windows -->
<!-- How to add a new app: -->
<!-- 1. Copy-paste this file and change its name -->
<!-- 2. Add the component to div.windows in desktop and add event listeners -->
<!-- 3. Make sure to add shortcut data -->

<script lang="ts">
  
  import { createEventDispatcher } from "svelte";
  import Window from "./Window.svelte"

  let dispatcher = createEventDispatcher();

</script>

<Window
  title="WindowTemplate"
  pos={{ x: 100, y: 50 }}
  size={{ width: 800, height: 600 }}
  on:windowClose={() => dispatcher('windowClose')}
  on:windowMinimize={() => dispatcher('windowMinimizeStateChange')}
>
  <p>Window template</p>
  <!-- Write the HTML for the window here -->
</Window>

<style>

</style>
