<script setup lang="ts">
import { PropType } from "vue";

const props = defineProps({
    keyName: { type: String, required: true },
    value: { type: Object as PropType<string | number>, required: true },
});
</script>

<template>
    <div class="container">
        <div style="display: flex">
            <h1>{{ keyName.replace("_", " ").toUpperCase() }}</h1>
        </div>
        <h3>{{ value }}</h3>
    </div>
</template>
