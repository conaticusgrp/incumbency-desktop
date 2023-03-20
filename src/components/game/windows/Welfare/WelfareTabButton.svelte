<script lang="ts">
    
  import { createEventDispatcher } from "svelte";
  import { TAB_LIST_ENTRY_MARGIN } from "../../../../scripts/desktopConstants";

  export let index: number;
  export let selected: boolean = false;
  export let data: string;

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
