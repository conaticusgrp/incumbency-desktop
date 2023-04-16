<script setup lang="ts">
import { invoke } from "@tauri-apps/api/tauri";
import { ref } from "vue";
import { useHealthcareStore } from "src/store/graphs";
import ValueCard from "src/components/cards/ValueCard.vue";

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

            data.value.child_care.total_capacity = newValue; // idk why it complains :(
            break;
        case GameValue.AdultcareCapacity:
            await invoke("update_adultcare_capacity", {
                newCapacity: newValue,
            });

            data.value.adult_care.total_capacity = newValue;
            break;
        case GameValue.EldercareCapacity:
            await invoke("update_eldercare_capacity", {
                newCapacity: newValue,
            });

            data.value.elder_care.total_capacity = newValue;
            break;
        default:
            break;
    }
};
</script>

<template>
    <main>
        <h1>CAPACITIES</h1>
        <ValueCard
            title="Childcare Capacity"
            :value="data.child_care.total_capacity"
            @assignValueFn="($val: any) => updateGameValue(GameValue.ChildcareCapacity, Number($val))"
            :hasAmendButton="true"
            :detail="{}"
        />

        <ValueCard
            title="Adultcare Capacity"
            :value="data.adult_care.total_capacity"
            @assignValueFn="($val: any) => updateGameValue(GameValue.AdultcareCapacity, Number($val))"
            :hasAmendButton="true"
            :detail="{}"
        />

        <ValueCard
            title="Eldercare Capacity"
            :value="data.elder_care.total_capacity"
            @assignValueFn="($val: any) => updateGameValue(GameValue.EldercareCapacity, Number($val))"
            :hasAmendButton="true"
            :detail="{}"
        />
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
