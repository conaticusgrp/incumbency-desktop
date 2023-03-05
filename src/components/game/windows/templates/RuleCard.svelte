<script lang="ts" context="module">
    export enum Rules {
        Tax,
        BusinessTax,
        BusinessFunding,
        DenyAge,
        DenyHealthPercentage,
        CoverFood,
        CoverFoodUnemployed,
    }

    export interface RuleCardValue {
        startStr?: string;
        value: any;
        endStr?: string;
    }
</script>

<script lang="ts">
    import Button from "../../../ui/Button.svelte";
    import ToggleButton from "../../../ui/ToggleButton.svelte";
    import AmendRule from "./AmendRule.svelte";

    export let category: string;
    export let title: string;
    export let values: RuleCardValue[];
    export let data: any;
    export let enabled: boolean;
    export let onActivationToggle: (activated: boolean) => void;
    export let updateRuleFn: (data: any[]) => void;

    let showAmendModal = false;
</script>

<div style="opacity: {enabled ? 1 : 0.5}" class="container">
    {#if showAmendModal}
        <AmendRule
            {updateRuleFn}
            bind:shown={showAmendModal}
            updateValues={values}
        />
    {/if}

    <div class="left">
        <div>
            <h5>{category.toUpperCase()}</h5>
            <h4>{title}</h4>
        </div>

        <div>
            <p>
                {#each values as val}
                    {val.startStr || ""}<strong>{val.value}</strong
                    >{val.endStr || ""}
                {/each}
            </p>
        </div>

        <div>
            <ToggleButton
                style="margin-right: 10px"
                activeText="Deactivate"
                inactiveText="Activate"
                bind:isToggled={enabled}
                width="200px"
                onClick={(isToggled) => {
                    onActivationToggle(isToggled);
                }}
            />
            <Button
                bind:enabled
                onClick={() => (showAmendModal = true)}
                width="200px">Amend</Button
            >
        </div>
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

<style>
    .container {
        display: flex;
        border: solid 1px var(--color-accent);
        margin: 20px;
        padding: 10px;
        height: 120px;
    }

    .container .left {
        display: flex;
        flex-direction: column;
        justify-content: space-between;
        margin-right: 50px;
    }

    .container .left div {
        display: flex;
    }

    h5 {
        background-color: var(--color-accent);
        color: black;
        padding: 5px 40px;
        font-weight: bold;
        font-size: 16px;
        margin-right: 20px;
    }

    h4 {
        font-size: 20px;
        padding: 5px;
        font-weight: 500;
        color: grey;
    }

    .left p {
        font-size: 20px;
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
</style>
