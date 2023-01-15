<script lang="ts">
  
  import { createEventDispatcher } from "svelte";
  import { listen } from "@tauri-apps/api/event";
  import Window from "./Window.svelte"
  import DebugValueDisplay from "../debug/DebugValueDisplay.svelte";

  export let iconPath: string | undefined = undefined;

  let dispatcher = createEventDispatcher();

  let debugData: object | undefined;

  listen('debug_payload', (e) => {
    debugData = e.payload as object;
    console.log(debugData);
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
  <main>

    <h2>Tracked data:</h2>
  
    <div>

      {#if debugData != undefined}

      <DebugValueDisplay key={undefined} value={debugData} />

      {/if}

    </div>

  </main>

</Window>

<style>

  main {
    display: flex;
    flex-direction: column;
    width: 100%;
    height: 100%;
    text-align: left;
  }

  h2 {
    height: 1em;
    margin: 0.5em;
  }

  div {
    height: calc(100% - 2em);
    overflow-x: scroll;
    overflow-y: scroll;
  }

</style>