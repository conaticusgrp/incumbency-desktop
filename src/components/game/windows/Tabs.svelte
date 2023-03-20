<script context="module" lang="ts">

    export interface TabsObj {
        [key: string]: typeof SvelteComponent;
    }

</script>

<script lang="ts">

  import type { SvelteComponent } from "svelte";
  
  export let tabs: TabsObj;
  export let appData: any;

  let currentComponent = Object.values(tabs)[0];
  
</script>

<main>

  {#each Object.entries(tabs) as [name, Component]}
    <button on:click={() => (currentComponent = Component)}>{name}</button>
  {/each}

  <svelte:component data={appData} this={currentComponent} />
  
</main>
