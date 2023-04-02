<!-- Don't forget to check if all the import statements are correct! -->
<script lang="ts">
    export let data: any;
</script>

<!-- This component is a template for tabs -->
<!-- How to add a new tab: -->
<!-- 1. Copy-paste this file and change its name -->
<!-- 2. Pass the tab component as a prop to the tab window -->

<main>
    {#if data != null}
        <!-- Insert HTML here -->
    {/if}
</main>

<style>
    main {
        width: 100%;
        height: 100%;
    }
</style>
