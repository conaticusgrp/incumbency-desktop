<script lang="ts">
  
  export let data: any;

</script>

<main>

  {#if data != null}
    {#each Object.entries(data) as [name, val]}
      {#if typeof val !== "object"}
        {name}: {val} <br />
      {/if}
    {/each}
  {/if}

</main>

<style>
  
  main {
    width: 100%;
    height: 100%;
  }
  
</style>
