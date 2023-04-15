<script setup lang="ts">
import { NOTIFICATION_MARGIN_X, NOTIFICATION_WIDTH } from 'src/constants';
import { useNotificationsStore, type NotificationData, type NotificationId, Action } from 'src/store/notifications';
import { computed, ref } from 'vue';
import Notification from './Notification.vue';

const props = defineProps({
  isExpanded: {
    type: Boolean,
    default: false,
  },
});
const emits = defineEmits<{
  (e: 'openApp', v: string): void;
}>();
const notiStore = useNotificationsStore();
const notifications = ref(notiStore.$state.data);
const reversed = computed(() => notifications.value.reverse());

const openApp = (noti: NotificationData) => emits('openApp', noti.app ?? '');
notiStore.$subscribe((_, state) => notifications.value = state.data);

const onDismiss = (noti: NotificationData) => notiStore.dismiss(noti.id);
const onAction = (noti: NotificationData) => {
  if (noti.action === Action.OpenApp) openApp(noti);
}

// Styles
const contractedStyle = `width: calc(${NOTIFICATION_WIDTH} + ${NOTIFICATION_MARGIN_X} * 2);`
const expandedStyle = contractedStyle + ' border: none; background: none;';
const notificationStyle = computed(() => props.isExpanded ? expandedStyle : contractedStyle);
</script>

<template>
  <div class="notifications-section" :style=notificationStyle>
    <Notification v-for="noti in reversed" :data="noti" @dismissed="onDismiss(noti)" @action="onAction(noti)" />
  </div>
</template>

<style scoped>
.notifications-section {
  display: flex;
  flex-direction: column;
  align-items: center;
  position: absolute;
  top: 0;
  right: 0;
  height: 100%;
  background-color: var(--color-bg);
  border-left: 1px solid var(--color-accent);
  z-index: 20000;
}

.notification-section-toggle {
  position: absolute;
  top: 0;
  right: 0;
  height: 100%;
  border-left: 1px solid var(--color-accent);
}
</style>
