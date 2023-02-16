<script lang="ts">

  import { handleAppWindowEvents } from "../../../../scripts/windowEvent";
  import Window, { type CriticalWindowData } from "../Window.svelte";
  import Tabs, { type TabsObj } from "./Tabs.svelte";
  
  export let title: string;
  export let pos;
  export let size;
  export let windowData: CriticalWindowData;
  export let eventDispatcher: any;
  export let tabs: TabsObj;
  
  let appData: any;

</script>

<Window
  {title}
  {pos}
  {size}
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