<script lang="ts">
    import Button from "../../../../ui/Button.svelte";
    import AmendModalTemplate from "../../templates/AmendModalTemplate.svelte";

    export let title: string;
    export let currentValue: any;
    export let assignValueFn: (newValue: any) => any;
    export let data: { [key: string]: any } = {};
    export let appendValueStart: string = "";
    export let appendValueEnd: string = "";
    export let hasAmendButton = true;

    let showAmendModal = false;
</script>

<main>
    <AmendModalTemplate appendValueStart={appendValueStart} appendValueEnd={appendValueEnd} bind:shown={showAmendModal} title={`Enter new value for "${title}"`} value={currentValue} updateValueFn={assignValueFn} />

    <div class="container">
        <div class="left">
            <h4>{title}</h4>
            <h2>{appendValueStart}{currentValue}{appendValueEnd}</h2>
            {#if hasAmendButton}
                <Button onClick={() => showAmendModal = true} width="200px">Amend</Button>
            {/if}
        </div>
    </div>
</main>

<style>
    .container {
        width: 80%;
        display: flex;
        margin: 30px auto;
        padding: 8px;
        height: 150px;
        border: solid 1px var(--color-accent);
    }
    
    .container .left {
        display: flex;
        flex-direction: column;
        justify-content: space-around;
    }

    h4 {
        font-size: 20px;
        color: grey;
    }

    h2 {
        font-size: 30px;
        font-weight: bold;
    }
</style>