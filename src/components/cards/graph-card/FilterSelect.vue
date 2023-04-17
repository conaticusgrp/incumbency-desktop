<script setup lang="ts">
import { ref } from "vue";

const props = defineProps<{
    type: GraphData<any>["type_id"];
    length: number;
}>();
const emits = defineEmits<{
    (e: "selected", value: Selected): void;
}>();

const getDayChoices = (length: number) => {
    const result = ["1W"];
    if (length >= 30) {
        result.push("1M");
    }
    if (length >= 90) {
        result.push("3M");
    }
    if (length >= 180) {
        result.push("6M");
    }
    if (length >= 365) {
        result.push("1Y");
    }
    if (length >= 1095) {
        result.push("3Y");
    }
    return result;
};

const getMonthChoices = (length: number) => {
    const result = ["1M"];
    if (length >= 3) {
        result.push("3M");
    }
    if (length >= 6) {
        result.push("6M");
    }
    if (length >= 12) {
        result.push("1Y");
    }
    if (length >= 36) {
        result.push("3Y");
    }
    return result;
};

const getChoices = (typeId: 0 | 1, length: number) => {
    if (typeId === 0) {
        return getDayChoices(length);
    }
    return getMonthChoices(length);
};

const choices = getChoices(props.type, props.length);
const selected = ref(choices[0]);
const onSelect = (choice: string) => {
    selected.value = choice;
    emits("selected", choice as Selected);
};
</script>

<template>
    <button
        v-for="choice in choices"
        :class="selected === choice ? 'selected' : ''"
        @click="onSelect(choice)"
    >
        {{ choice }}
    </button>
</template>

<style scoped>
button {
    border: 1px solid var(--color-accent);
}

.selected {
    background-color: var(--color-accent);
    color: var(--color-bg);
    font-weight: bold;
}
</style>
