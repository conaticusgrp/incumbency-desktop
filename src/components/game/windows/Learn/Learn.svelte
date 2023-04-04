<!-- Don't forget to check if all the import statements are correct! -->
<script lang="ts">
    import { createEventDispatcher } from "svelte";
    import TabWindow from "../TabWindow.svelte";
    import type { CriticalWindowData } from "../Window.svelte";

    import TabButton from "./TabButton.svelte";
    import LearnTab from "./LearnTab.svelte";

    export let windowData: CriticalWindowData;

    let dispatcher = createEventDispatcher();
</script>

<!-- This component is a template for tab windows -->
<!-- How to add a new app: -->
<!-- 1. Copy-paste this file and change its name -->
<!-- 2. Add tab button and tab components  -->
<!-- 3. Add the component to [apps] in the Dekstop component -->

<TabWindow
    title="Learn"
    pos={{ x: 100, y: 50 }}
    size={{ width: 800, height: 600 }}
    {windowData}
    tabButtonComponent={TabButton}
    tabButtonData={["Start"]}
    tabs={[
        { c: LearnTab, data: ["https://www.youtube.com/embed/dBxOYE2j55U"] },
    ]}
    on:criticalWindowEvent={(e) => dispatcher("criticalWindowEvent", e.detail)}
/>

<style>
    main {
        width: 100%;
        height: 100%;
    }
</style>
