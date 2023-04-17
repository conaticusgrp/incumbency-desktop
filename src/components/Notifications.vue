<script setup lang="ts">
import { NOTIFICATION_MARGIN_X, NOTIFICATION_WIDTH } from 'src/constants';
import { useNotificationsStore, type NotificationData, Action } from 'src/store/notifications';
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
const notifications = ref(notiStore.notifications);
const reversed = computed(() => notifications.value.reverse());
const openApp = (noti: NotificationData) => emits('openApp', noti.app ?? '');

notiStore.$subscribe(() => {
  const latest = notiStore.latestNotification;
  if (latest) {
    notifications.value.push(latest);
  }
});

const onDismiss = (noti: NotificationData) => notiStore.dismiss(noti.id);
const onAction = (noti: NotificationData) => {
  if (noti.action === Action.OpenApp) openApp(noti);
}
</script>

<template>
  <Notification v-for="noti in reversed" :key="noti.id" :data="noti" @dismissed="onDismiss(noti)" @action="onAction(noti)" />
</template>
