<!-- This component is a template for tab windows -->
<!-- How to add a new app: -->
<!-- 1. Copy-paste this file and change its name -->
<!-- 2. Add tab button and tab components  -->
<!-- 3. Add the component to [apps] in the Dekstop component -->
<!-- Don't forget to check if all the import statements are correct! -->

<script lang="ts">
  
  import { createEventDispatcher } from "svelte";
  import TabWindow from "../TabWindow.svelte";
  import type { CriticalWindowData } from "../Window.svelte"
    
  import TestTabButton from "./TabButtonTemplate.svelte";
  import TestTab1 from "./TabTemplate.svelte";
  
  export let windowData: CriticalWindowData;
  
  let dispatcher = createEventDispatcher();
  
</script>
  
<TabWindow
  title="TabWindowTemplate"
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
  