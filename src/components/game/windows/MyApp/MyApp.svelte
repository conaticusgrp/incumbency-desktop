<!-- Don't forget to check if all the import statements are correct! -->
<script lang="ts">
    import { createEventDispatcher } from "svelte";
    import TabWindow from "../TabWindow.svelte";
    import type { CriticalWindowData } from "../Window.svelte";
    import MyTab from "./MyTab/MyTab.svelte";

    import TestTabButton from "./TabButton.svelte";

    export let windowData: CriticalWindowData;

    let dispatcher = createEventDispatcher();
</script>

<!-- This component is a template for tab windows -->
<!-- How to add a new app: -->
<!-- 1. Copy-paste this file and change its name -->
<!-- 2. Add tab button and tab components  -->
<!-- 3. Add the component to [apps] in the Dekstop component -->

<TabWindow
    title="TabWindowTemplate"
    pos={{ x: 100, y: 50 }}
    size={{ width: 800, height: 600 }}
    {windowData}
    tabButtonComponent={TestTabButton}
    tabButtonData={["The Tab"]}
    tabs={[{ c: MyTab, data: {} }]}
    on:criticalWindowEvent={(e) => dispatcher("criticalWindowEvent", e.detail)}
/>
