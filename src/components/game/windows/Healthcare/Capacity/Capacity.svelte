<script lang="ts">
    import { invoke } from "@tauri-apps/api/tauri";
    import ValueCard from "../../templates/ValueCard.svelte";
    import type { HealthcareData } from "../Healthcare.svelte";

    export let data: HealthcareData | undefined;

    enum GameValue {
        ChildcareCapacity,
        AdultcareCapacity,
        EldercareCapacity,
    }

    const handleErrorFromResult = (result: any): boolean => {
        if (result.error) {
            console.error(result.error);
            return true;
        }

        return false;
    };

    const updateGameValue = async (gameValue: GameValue, newValue: any) => {
        if (!data) throw new Error("Data is undefined.");

        switch (gameValue) {
            case GameValue.ChildcareCapacity:
                const childRes = await invoke("update_childcare_capacity", {
                    newCapacity: newValue,
                });

                if (!handleErrorFromResult(childRes)) {
                    data.child_care.total_capacity = newValue;
                }

                break;
            case GameValue.AdultcareCapacity:
                const adultRes = await invoke("update_adultcare_capacity", {
                    newCapacity: newValue,
                });

                if (!handleErrorFromResult(adultRes)) {
                    data.adult_care.total_capacity = newValue;
                }
                break;
            case GameValue.EldercareCapacity:
                const elderRes = await invoke("update_eldercare_capacity", {
                    newCapacity: newValue,
                });

                if (!handleErrorFromResult(elderRes)) {
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
