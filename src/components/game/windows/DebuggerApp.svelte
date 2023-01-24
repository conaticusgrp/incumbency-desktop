<script lang="ts">
  
  import { createEventDispatcher } from "svelte";
  import { listen } from "@tauri-apps/api/event";
  import Window from "./Window.svelte"
  import DebugValueDisplay from "../debug/DebugValueDisplay.svelte";

  let dispatcher = createEventDispatcher();

  let debugData: object | undefined;

  listen('debug_payload', (e) => {
    if (debugData) {
      debugData = { ...debugData, ...e.payload as object }
      return;
    }

    debugData =  e.payload as object;
  });

</script>

<Window
  title="DEBUG"
  pos={{ x: 100, y: 50 }}
  size={{ width: 800, height: 600 }}
  on:criticalWindowEvent={(e) => dispatcher('criticalWindowEvent', e.detail)}
>
  <main>
  
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
  
  div {
    height: 100%;
    overflow-x: scroll;
    overflow-y: scroll;
  }

  div::-webkit-scrollbar,
  div::-webkit-scrollbar-corner {
    background-color: black;
  }
  
  div::-webkit-scrollbar-thumb {
    background-color: var(--color-secondary);
  }

</style>