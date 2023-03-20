<!-- This component is a template for tab buttons -->
<!-- How to add a new button: -->
<!-- 1. Copy-paste this file and change its name -->
<!-- 2. Pass the tab button as a prop to the tab window -->
<!-- Don't forget to check if all the import statements are correct! -->

<script lang="ts">
    
  import { createEventDispatcher } from "svelte";
  import { TAB_LIST_ENTRY_MARGIN } from "../../../../scripts/desktopConstants";

  export let index: number;
  export let selected: boolean = false;
  export let data: any;

  let dispatcher = createEventDispatcher();

  const select = (): void => {
    dispatcher('selectTab', { index });
  }

</script>

<button
  style="margin: {TAB_LIST_ENTRY_MARGIN};"
  data-selected={selected}
  on:click={select}
>

  [{index}]
  :
  {data}

</button>

<style>

  button {
    border: 1px solid var(--color-accent);
  }

  button[data-selected="true"] {
    background-color: var(--color-accent);
    color: var(--color-bg);
    font-weight: bold;
  }

</style>
