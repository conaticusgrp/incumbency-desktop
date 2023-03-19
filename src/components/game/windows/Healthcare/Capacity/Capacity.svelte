<script lang="ts">
    import { invoke } from "@tauri-apps/api/tauri";
    import { createEventDispatcher } from "svelte";
    import { handleInvoke } from "../../../../../scripts/util";
    import ValueCard from "../../templates/ValueCard.svelte";
    import type { HealthcareData } from "../Healthcare.svelte";

    export let data: HealthcareData | undefined;
    const dispatcher = createEventDispatcher();

    enum GameValue {
        ChildcareCapacity,
        AdultcareCapacity,
        EldercareCapacity,
    }

    const updateGameValue = async (gameValue: GameValue, newValue: any) => {
        if (!data) throw new Error("Data is undefined.");

        switch (gameValue) {
            case GameValue.ChildcareCapacity:
                const childRes = await handleInvoke(
                    dispatcher,
                    invoke("update_childcare_capacity", {
                        newCapacity: newValue,
                    }),
                    "healthcare"
                );

                if (childRes !== false) {
                    data.child_care.total_capacity = newValue;
                }

                break;
            case GameValue.AdultcareCapacity:
                const adultRes = await handleInvoke(
                    dispatcher,
                    invoke("update_adultcare_capacity", {
                        newCapacity: newValue,
                    }),
                    "healthcare"
                );

                if (adultRes !== false) {
                    data.adult_care.total_capacity = newValue;
                }

                break;
            case GameValue.EldercareCapacity:
                const elderRes = await handleInvoke(
                    dispatcher,
                    invoke("update_eldercare_capacity", {
                        newCapacity: newValue,
                    }),
                    "healthcare"
                );

                if (elderRes !== false) {
                    data.elder_care.total_capacity = newValue;
                }

                break;
            default:
                break;
        }
    };
</script>

<main>
    {#if data != null}
        <h1>CAPACITIES</h1>

        <ValueCard
            title="Childcare Capacity"
            currentValue={data.child_care.total_capacity}
            data={{}}
            assignValueFn={(val) =>
                updateGameValue(GameValue.ChildcareCapacity, Number(val))}
        />

        <ValueCard
            title="Adultcare Capacity"
            currentValue={data.adult_care.total_capacity}
            data={{}}
            assignValueFn={(val) =>
                updateGameValue(GameValue.AdultcareCapacity, Number(val))}
        />

        <ValueCard
            title="Eldercare Capacity"
            currentValue={data.elder_care.total_capacity}
            data={{}}
            assignValueFn={(val) =>
                updateGameValue(GameValue.EldercareCapacity, Number(val))}
        />
    {/if}
</main>

<style>
    main {
        width: 100%;
        height: 100%;
        position: relative;
        overflow-y: scroll;
    }

    main::-webkit-scrollbar {
        display: none;
    }

    h1 {
        font-size: 25px;
        font-weight: bold;
        margin: 20px 0;
    }
</style>
