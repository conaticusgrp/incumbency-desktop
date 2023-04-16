<script setup lang="ts">
import { PropType } from "vue";

defineProps({
    key: { type: String, required: true },
    value: { type: Object as PropType<string | number> },
});
</script>

<template>
    <div class="container">
        <div style="display: flex">
            <h1>{{ key.replace("_", " ").toUpperCase() }}</h1>
        </div>
        <h3>{{ value }}</h3>
    </div>
</template>
