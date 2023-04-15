<script setup lang="ts">
import { invoke } from "@tauri-apps/api/tauri";
import { ref } from "vue";
import { useHealthcareStore } from "src/store/graphs";

const graphStore = useHealthcareStore();
const data = ref<HealthcareData>(graphStore.$state.data);

enum GameValue {
    ChildcareCapacity,
    AdultcareCapacity,
    EldercareCapacity,
}

const updateGameValue = async (gameValue: GameValue, newValue: any) => {
    if (!data) throw new Error("Data is undefined.");

    switch (gameValue) {
        case GameValue.ChildcareCapacity:
            // BEEPBOOP(conaticus): remember error handling lol im loosing my sanity as we speak!!
            await invoke("update_childcare_capacity", {
                newCapacity: newValue,
            });

            data.child_care.total_capacity = newValue; // idk why it complains :(
            break;
        case GameValue.AdultcareCapacity:
            await invoke("update_adultcare_capacity", {
                newCapacity: newValue,
            });

            data.adult_care.total_capacity = newValue;
            break;
        case GameValue.EldercareCapacity: // I am racing against my vision rn it's in 360p
            await invoke("update_eldercare_capacity", {
                newCapacity: newValue,
            });

            data.elder_care.total_capacity = newValue;
            break;
        default:
            break;
    }
};
</script>

<template>
    <main>
        <h1>CAPACITIES</h1>

        <ValueCard title="Childcare Capacity"
        currentValue={data.child_care.total_capacity} data={{}}
            assignValueFn={(val) =>
                updateGameValue(GameValue.ChildcareCapacity, Number(val))}
        />

        <ValueCard
            title="Adultcare Capacity"
            currentValue={data.adult_care.total_capacity}
            data={{}}
        assignValueFn={(val) => updateGameValue(GameValue.AdultcareCapacity,
        Number(val))} /> <ValueCard title="Eldercare Capacity"
        currentValue={data.elder_care.total_capacity} data={{}}
        assignValueFn={(val) => updateGameValue(GameValue.EldercareCapacity,
        Number(val))} />
        <!-- idk ill look tmr -->
    </main>
</template>

<style scoped>
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
