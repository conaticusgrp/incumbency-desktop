<script lang="ts">
  
  import { createEventDispatcher } from "svelte";
  import TabWindow from "../windows/TabWindow.svelte";
  import type { CriticalWindowData } from "../windows/Window.svelte"
  
  import TestTabButton from "./TestTabButton.svelte";
  import TestTab1 from "./TestTab1.svelte";

  export let windowData: CriticalWindowData;

  let dispatcher = createEventDispatcher();

</script>

<TabWindow
  title="Test Tab App"
  pos={{ x: 100, y: 50 }}
  size={{ width: 800, height: 600 }}
  {windowData}
  tabButtonComponent={TestTabButton}
  tabButtonData={[
    1,
    2,
    3
  ]}
  tabs={[
    { c: TestTab1, data: "msg 1" },
    { c: TestTab1, data: "msg 2" },
    { c: TestTab1, data: "msg 3" },
  ]}
  on:criticalWindowEvent={(e) => dispatcher('criticalWindowEvent', e.detail)}
/>
