<script lang="ts">
  
  import { createEventDispatcher } from "svelte";
  import Window from "./Window.svelte"

  let dispatcher = createEventDispatcher();

</script>

<Window
  parentComponent={parentComponent}
  title="TestWindow"
  iconPath="https://upload.wikimedia.org/wikipedia/commons/thumb/6/69/How_to_use_icon.svg/1200px-How_to_use_icon.svg.png"
  pos={{ x: 100, y: 50 }}
  size={{ width: 800, height: 600 }}
  on:windowClose={() => dispatcher('windowClose')}
>
  <p>Test window</p>
  <!-- Write the HTML for the window here -->
</Window>

<style>

</style>
