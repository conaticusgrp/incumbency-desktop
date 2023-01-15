<script lang="ts">
  
  import { createEventDispatcher } from "svelte";
  import { listen } from "@tauri-apps/api/event";
  import Window from "./Window.svelte"

  export let iconPath: string | undefined = undefined;

  let dispatcher = createEventDispatcher();

  let debugData: object | undefined;

  listen('debug_payload', (e) => {
    debugData = e.payload as object;
  });

</script>

<Window
  title="DEBUG"
  {iconPath}
  pos={{ x: 100, y: 50 }}
  size={{ width: 800, height: 600 }}
  on:windowClose={() => dispatcher('windowClose')}
  on:windowMinimize={() => dispatcher('windowMinimizeStateChange')}
>
  <p>data object:</p>
  <p>{debugData}</p>
</Window>

<style>

</style>