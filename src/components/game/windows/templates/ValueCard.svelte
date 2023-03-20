<script lang="ts">
    import Button from "../../../ui/Button.svelte";
    import AmendModalTemplate from "./AmendModalTemplate.svelte";

    export let title: string;
    export let currentValue: any;
    export let assignValueFn: (newValue: any) => any;
    export let data: { [key: string]: any } = {};
    export let appendValueStart: string = "";
    export let appendValueEnd: string = "";
    export let hasAmendButton = true;

    let showAmendModal = false;
</script>

<main style="--container-height: {hasAmendButton ? '120px' : '60px'}">
    <AmendModalTemplate
        {appendValueStart}
        {appendValueEnd}
        bind:shown={showAmendModal}
        title={`Enter new value for "${title}"`}
        value={currentValue}
        updateValueFn={assignValueFn}
    />

    <div class="container">
        <div class="left">
            <h4>{title}</h4>
            <h2>{appendValueStart}{currentValue}{appendValueEnd}</h2>
            {#if hasAmendButton}
                <Button onClick={() => (showAmendModal = true)} width="200px"
                    >Amend</Button
                >
            {/if}
        </div>

        <div class="right">
            <div class="keys">
                {#each Object.keys(data) as key}
                    <p>{key}:&nbsp;</p>
                {/each}
            </div>

            <div class="values">
                {#each Object.values(data) as value}
                    <p>{value}</p>
                {/each}
            </div>
        </div>
    </div>
</main>

<style>
    .container {
        width: 95%;
        display: flex;
        margin: 0 auto;
        margin-bottom: 20px;
        padding: 10px;
        height: var(--container-height);
        border: solid 1px var(--color-accent);
    }

    .container .left {
        display: flex;
        flex-direction: column;
        justify-content: space-between;
        margin-right: 20px;
        width: 200px;
    }

    .container .right {
        display: flex;
        text-align: left;
    }

    .container .right .keys {
        margin-right: 20px;
    }

    .container .right p {
        font-size: 15px;
        font-weight: normal;
        margin-bottom: 3px;
    }

    .container .right .keys p {
        color: grey;
    }

    h4 {
        font-size: 20px;
        color: grey;
        font-weight: 500;
    }

    h2 {
        font-size: 30px;
        font-weight: bold;
    }
</style>
