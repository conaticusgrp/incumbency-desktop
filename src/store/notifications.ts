import { defineStore } from "pinia";
import { computed, ref } from "vue";

export enum Severity { Normal, Warning, Error };
export enum Action { OpenApp, Nothing };

export type CreateNotification = {
  app: string;
  header: string;
  content: string;
  severity: Severity;
  action: Action;
  date?: string;
  actionTitle?: string;
}

export type NotificationData = { 
  id: number, 
  createdAt: Date
} & CreateNotification;

// NOTE(dylhack): simple incrementing ID
let _id = 0;
const getId = () => _id++;
// NOTE(dylhack): 5 seconds (in ms)
const JUST_DISPLAYED = 5_000;

export type NotificationId = NotificationData['id'];

export const justDisplayed = (n: NotificationData) => n.createdAt.getTime() + JUST_DISPLAYED > Date.now();

export const useNotificationsStore = defineStore("notifications", () => {
  const state = ref<NotificationData[]>([]); 

  const addNotification = (data: CreateNotification) => {
    const newNoti = { ...data, id: getId(), createdAt: new Date(), };
    state.value.push(newNoti);
    return newNoti;
  };

  const dismiss = (id: NotificationId) => {
    const i = state.value.findIndex((n) => n.id === id);
    if (i !== -1) {
      state.value.splice(i, 1);
    }
  };

  const latestNotification = computed<NotificationData | undefined>(() => {
    return state.value[state.value.length - 1];
  });

  const notifications = computed(() => state.value);

  return { addNotification, dismiss, latestNotification, notifications };
});
