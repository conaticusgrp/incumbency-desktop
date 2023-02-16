<script lang="ts">
  
  import { createEventDispatcher } from "svelte";
  import TabWindow from "../windows/TabWindow.svelte";
  import type { CriticalWindowData } from "../windows/Window.svelte"
  import TestTab from "./TestTab.svelte";

  export let windowData: CriticalWindowData;

  let dispatcher = createEventDispatcher();

</script>

<TabWindow
  title="Test Tab App"
  pos={{ x: 100, y: 50 }}
  size={{ width: 800, height: 600 }}
  {windowData}
  tabComponent={TestTab}
  tabData={[
    1,
    2,
    3
  ]}
  on:criticalWindowEvent={(e) => dispatcher('criticalWindowEvent', e.detail)}
/>
