<script lang="ts">

    export let key: string | undefined;
    export let value: any;
    export let indentation: number = 0;

    let dropdownShown = false;

    const toggleDropdown = (): void => {
      dropdownShown = !dropdownShown;
    }

</script>

<main style="margin-left: calc(10px * {indentation});">

  {#if key != undefined}

  <span class="key">
    <button on:click={toggleDropdown}>+</button>
    {key}
  </span>

  {/if}

  {#if key == undefined || dropdownShown}

  <span class="value">

    {#if value instanceof Array}

      {#each value as entry, i}
        <svelte:self key={i} value={entry} indentation={indentation + 1} />
      {/each}

    {:else if value instanceof Object}

      {#each Object.entries(value) as [newKey, entry]}
        <svelte:self key={newKey} value={entry} indentation={indentation + 1} />
      {/each}

    {:else}
      {value}
    {/if}

  </span>

  {/if}

</main>

<style>

    main {
        /* display: grid;
        grid-template-columns: 1fr 1fr;
        grid-template-rows: 1fr; */
        padding: 3px;
    }

    .key {
      color: grey;
    }

    .key > button {
      width: 1em;
      height: 1em;
      padding: 3px;
      border-radius: 0;
    }

</style>
