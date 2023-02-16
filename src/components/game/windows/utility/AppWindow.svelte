<script lang="ts">
  import { createEventDispatcher } from "svelte";
  import { handleAppWindowEvents } from "../../../../scripts/windowEvent";
  import Window, { type CriticalWindowData } from "../Window.svelte";
  import Tabs, { type TabsObj } from "./Tabs.svelte";
  
  export let name: string;
  export let windowData: CriticalWindowData;
  export let tabs: TabsObj;
  export let dispatcher: any;
  
  let appData: any;
</script>

<Window
  title={name}  
  pos={{ x: 100, y: 50 }}
  size={{ width: 800, height: 600 }}
  {windowData}
  on:criticalWindowEvent={(e) => dispatcher("criticalWindowEvent", e.detail)}
  on:windowEvent={(e) => appData = handleAppWindowEvents(e, appData)}
>
  <main>
    <Tabs
      tabs={tabs}
      appData={appData}
    />
  </main>
</Window>

<style>
  main {
    width: 100%;
    height: 100%;
  }
</style>