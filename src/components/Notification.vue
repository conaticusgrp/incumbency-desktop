<script setup lang="ts">
import { PropType, computed, ref } from "vue";
import * as constants from "../constants";
import {
    NotificationData,
    Severity,
    justDisplayed,
    useNotificationsStore,
} from "src/store/notifications";

const severityColors = new Map<Severity, string>()
    .set(Severity.Normal, "var(--color-accent)")
    .set(Severity.Warning, "#D47A21")
    .set(Severity.Error, "#C82525");

const props = defineProps({
    data: {
        type: Object as PropType<NotificationData>,
        required: true,
    },
});
const emits = defineEmits<{
    (e: "dismissed"): void;
    (e: "action", data: NotificationData): void;
}>();
const dismissClass = ref("");
const notiClass = ref(
    `${dismissClass.value} ${
        justDisplayed(props.data) && !dismissClass.value ? "new" : ""
    }}`
);

const onDismissed = () => {
    emits("dismissed");
};
const onAction = () => emits("action", props.data);
const onClick = () => {
    dismissClass.value = "dismiss";
    setTimeout(onDismissed, 200);
};

// Styles
const notiStyle = `
  --notification-color: ${severityColors.get(props.data.severity ?? "normal")};
  width: ${constants.NOTIFICATION_WIDTH}; height: ${
    constants.NOTIFICATION_HEIGHT
};
  margin: ${constants.NOTIFICATION_MARGIN_Y} 0 ${
    constants.NOTIFICATION_MARGIN_X
} 0;
`;
</script>

<template>
    <main :style="notiStyle" :class="notiClass">
        <div class="header">
            <button @click="onClick">Dismiss</button>
            <h2>{{ data.app ?? "" }}</h2>
            <span>{{ data.date && !justDisplayed ? data.date : "now" }}</span>
        </div>

        <div class="content">
            <h3>{{ data.header ?? "Notification" }}</h3>
            <p>{{ data.content ?? "" }}</p>
        </div>

        <div v-if="data.actionTitle" class="actions">
            <button @click="onAction">{actionTitle}</button>
        </div>
    </main>
</template>

<style>
main {
    display: flex;
    flex-direction: column;
    border: 1px solid var(--notification-color);
    background-color: black;
}

.new {
    animation-name: fadeout, popup;
    animation-delay: 5s, 0s;
    animation-duration: 5s, 0.5s;
    animation-fill-mode: forwards;
    animation-timing-function: none, ease-out;
}

.dismiss {
    animation-name: dismissed;
    animation-fill-mode: forwards;
    animation-timing-function: ease-out;
    animation-duration: 0.2s;
    animation-delay: 0s;
}

@keyframes dismissed {
    from {
        scale: 1;
    }

    to {
        scale: 0;
    }
}

@keyframes popup {
    from {
        scale: 0;
    }

    to {
        scale: 1;
    }
}

@keyframes fadeout {
    from {
        opacity: 1;
    }

    to {
        opacity: 0;
    }
}

.header {
    display: flex;
    align-items: center;
    padding-right: 1em;
    background-color: var(--notification-color);
    border-bottom: 1px solid var(--notification-color);
    color: var(--color-bg);
    font-size: 12px;
}

.header > button {
    background-color: var(--color-bg);
    color: var(--color-highlight);
    font-size: 10px;
    cursor: pointer;
}

.header > h2 {
    width: 100%;
    padding-left: 0.5em;
    font-size: 14px;
    font-weight: bold;
    text-align: left;
    text-transform: uppercase;
}

.header > span {
    white-space: nowrap;
    text-overflow: ellipsis;
}

.content {
    height: 100%;
    padding: 0 1em 0 1em;
    text-align: left;
}

.content > h3 {
    margin-top: 0.5em;
    font-size: 15px;
}

.content > p {
    color: var(--color-shaded);
    font-size: 13px;
}

.actions {
    border-top: 1px solid var(--notification-color);
}

.actions > button {
    padding: 0.5em;
    width: 100%;
}

button:hover {
    color: black;
    background-color: var(--notification-color);
}
</style>
