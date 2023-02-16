<script lang="ts">
    
    import { createEventDispatcher } from "svelte";


    export let index: number;
    export let data: any;

    let dispatcher = createEventDispatcher();

    const select = (): void => {
        dispatcher('selectTab', { index });
    }

</script>

<button>

    {index}
    :
    {data}

</button>

<style>

    button {
        border: 1px solid var(--color-accent);
    }

</style>