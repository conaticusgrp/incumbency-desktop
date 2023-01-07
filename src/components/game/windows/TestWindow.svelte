<script lang="ts">
  
  import { createEventDispatcher } from "svelte";
  import Window from "./Window.svelte"

  export let iconPath: string | undefined = undefined;

  let dispatcher = createEventDispatcher();

</script>

<Window
  title="TestWindow"
  {iconPath}
  pos={{ x: 100, y: 50 }}
  size={{ width: 800, height: 600 }}
  on:windowClose={() => dispatcher('windowClose')}
  on:windowMinimize={() => dispatcher('windowMinimize')}
>
  <p>Test window</p>
  <!-- Write the HTML for the window here -->
</Window>

<style>

</style>
