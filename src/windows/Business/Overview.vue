<script setup lang="ts" context="module">
import { useBusinessStore } from "src/store/graphs";
import { NotificationData } from "src/store/notifications";
import { ref } from "vue";

interface DataItem {
    title: string;
    dataKey: keyof BusinessData;
    data: any;
    pinned?: boolean;
    prefix?: string;
}

const graphStore = useBusinessStore();
const data = ref<BusinessData>(graphStore.$state.data);
const app = "finance";
const emits = defineEmits<{
    (e: "windowSendNotification", v: NotificationData): void;
}>();
</script>

<template>
    <div v-for="(value, key) in data" :key="key">
        <div class="container">
            <div style="display: flex">
                <h1>{{ key.replace("_", " ").toUpperCase() }}</h1>
            </div>
            <h3>{{ value }}</h3>
        </div>
    </div>
</template>

<style scoped>
h1 {
    font-size: 18px;
    font-weight: bold;
}
h3 {
    text-align: left;
    margin-top: 20px;
    font-size: 30px;
}
.btn {
    margin-right: 20px;
}
.container {
    margin-left: 20px;
    margin-top: 20px;
    margin-bottom: 50px;
    border: solid 1px var(--color-accent);
    padding: 20px;
}
</style>
