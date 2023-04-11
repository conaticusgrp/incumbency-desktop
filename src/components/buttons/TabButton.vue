<script setup lang="ts">
    import { TAB_LIST_ENTRY_MARGIN } from "../../constants";

    const props = defineProps({
      selected: {
        type: Boolean,
        default: false,
      }
    });
    const emits = defineEmits<{ (e: 'selectTab'): void }>();
    const select = (): void => emits("selectTab");
    const buttonStyle = `margin: ${TAB_LIST_ENTRY_MARGIN};`;
    const buttonClass = props.selected ? 'selected' : '';
</script>

<template>
  <button :style=buttonStyle :class=buttonClass @click=select>
    <slot />
  </button>
</template>

<style>
    button {
        border: 1px solid var(--color-accent);
    }

    .selected {
        background-color: var(--color-accent);
        color: var(--color-bg);
        font-weight: bold;
    }
</style>
