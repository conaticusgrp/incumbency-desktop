<script lang="ts">
    import type { FinanceData } from "../Finance.svelte";
    export let data: FinanceData | undefined;
</script>

<main>
    {#if data != null}
        <!-- Stuff here -->
    {/if}
</main>
  
<style>
  main {
    width: 100%;
    height: 100%;
  }
</style>