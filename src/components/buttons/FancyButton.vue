<script setup lang="ts">
  import { ref } from 'vue';
  const props = defineProps({
    width: {
      type: String,
      required: true,
    },
    height: {
      type: String,
      default: "34px",
    },
    style: {
      type: String,
      default: "",
    },
    enabled: {
      type: Boolean,
      default: true,
    },
  });
  const isEnabled = ref(props.enabled);
  const emits = defineEmits<{
    (e: 'click'): void;
  }>();
  const onClick = () => emits('click');
</script>

<template>
  <button v-bind:disabled="!isEnabled" @click={onClick} style="--width: {width}; --height: {height};{props.style}">
    <slot />
  </button>
</template>

<style>
  button {
    border: 1px solid var(--color-accent);
    width: var(--width);
    padding: 0;
    height: var(--height);
  }

  button:hover:enabled {
    background-color: var(--color-accent);
    color: var(--color-bg);
    font-weight: bold;
  }
</style>
