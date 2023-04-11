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
  isToggled: {
    type: Boolean,
    default: false,
  },
  activeText: {
    type: String,
    required: true,
  },
  inactiveText: {
    type: String,
    required: true,
  },
  style: {
    type: String,
    default: "",
  },
});
const emits = defineEmits<{
  (e: 'click', isToggled: boolean): void;
}>();
const isToggled = ref(props.isToggled);
const buttonClass = ref(props.isToggled ? "toggled" : "");
const style = `--width: ${props.width};--height: ${props.height};${props.style}`;
const onClick = () => {
  isToggled.value = !isToggled.value;
  emits('click', isToggled.value);
}
</script>

<template>
  <button :class=buttonClass :style=style v-on:click=onClick>
    {isToggled ? activeText : inactiveText}
  </button>
</template>

<style scoped>
button {
  border: 1px solid var(--color-accent);
  width: var(--width);
  padding: 0;
  height: var(--height);
}

.toggled {
  background-color: var(--color-accent);
  color: var(--color-bg);
  font-weight: bold;
}

button:hover {
  background-color: var(--color-accent);
  color: var(--color-bg);
  font-weight: bold;
}
</style>
