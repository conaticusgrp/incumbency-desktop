<script lang="ts">
    
  import { createEventDispatcher } from "svelte";
  import { TAB_LIST_ENTRY_MARGIN } from "../../../../scripts/desktopConstants";
  import type { EmailData } from "./Email.svelte"

  export let index: number;
  export let selected: boolean = false;
  export let data: EmailData;

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

  <h2>{data.sender?.username ?? "Unknown sender"}</h2>
  <h3>{data.title}</h3>
  <p>{data.content}</p>

</button>

<style>

  button {
    height: 6em;
    padding: 0.5em;
    color: var(--color-highlight);
    background-color: var(--color-bg);
    border: 1px solid var(--color-accent);
    text-align: left;
    text-overflow: ellipsis;
    cursor: pointer;
  }

  button[data-selected="true"] {
    background-color: var(--color-accent);
    color: var(--color-bg);
    font-weight: bold;
  }

  h2 {
    font-size: 14px;
    font-weight: bold;
  }

  h3 {
    font-size: 12px;
    font-weight: 500;
  }
  
  p {
    font-size: 12px;
    font-weight: 400;
    opacity: 0.35;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

</style>
