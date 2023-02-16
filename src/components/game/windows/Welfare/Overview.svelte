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