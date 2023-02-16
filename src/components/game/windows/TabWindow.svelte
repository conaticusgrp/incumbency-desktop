<script lang="ts">
  
  import { createEventDispatcher, SvelteComponent } from "svelte";
  import Window, { type CriticalWindowData, type Pos, type Size } from "./Window.svelte";
  import { TAB_LIST_ENTRY_MARGIN, TAB_LIST_MIN_WIDTH, TAB_LIST_WIDTH } from "../../../scripts/desktopConstants";
  
  export let title: string;
  export let pos: Pos;
  export let size: Size;
  export let windowData: CriticalWindowData;
  export let tabComponent: typeof SvelteComponent;
  export let tabData: any[] = [];
  
  export let currentTabIndex: number = 0;
  
  let dispatcher = createEventDispatcher();

  const selectTab = (e: CustomEvent): void => {
    const i = e.detail.index;
    if (i < 0 || i >= tabData.length) return;
    currentTabIndex = i;
  }

</script>

<Window
  {title}
  {pos}
  {size}
  {windowData}
  on:criticalWindowEvent={(e) => dispatcher("criticalWindowEvent", e.detail)}
  on:windowEvent={(e) => dispatcher("windowEvent", e.detail)}
>
  <main
    style="
      --tab-list-width: {TAB_LIST_WIDTH};
      --tab-list-min-width: {TAB_LIST_MIN_WIDTH};
      --tab-list-entry-margin: {TAB_LIST_ENTRY_MARGIN};
    "
  >

    <section>

      <div class="tab-list">

        {#each tabData as data, i}

        <svelte:component
          this={tabComponent}
          index={i}
          {data}
          on:selectTab={selectTab}
        />

        {/each}

        {#if false}
        <p></p>
        {/if}
    
      </div>

    </section>

    <section>

    </section>

  </main>
</Window>

<style>

  main {
    display: flex;
    flex-direction: row;
    width: 100%;
    height: 100%;
  }

  main > section:first-of-type {
    width: var(--tab-list-width);
    min-width: var(--tab-list-min-width);
    height: 100%;
    border-right: 1px solid var(--color-accent);
  }
  
  .tab-list {
    display: flex;
    flex-direction: column;
    width: 100%;
    height: 100%;
    overflow-y: scroll;
  }

 .tab-list::-webkit-scrollbar {
    display: none;
  }

  .tab-list > * {
    margin: var(--tab-list-entry-margin);
    /* box-sizing: border-box; */
  }

</style>