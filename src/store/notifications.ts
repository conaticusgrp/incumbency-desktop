import { defineStore } from "pinia";

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
export const useNotificationsStore = defineStore("notifications", {
  state: () => ([] as NotificationData[]),
  actions: {
    addNotification(data: CreateNotification) {
      const newNoti = {...data, id: getId(), createdAt: new Date()};
      this.$state.push(newNoti);
      return newNoti;
    },
    dismiss(id: NotificationId) {
      const i = this.$state.findIndex((n) => n.id === id);
      if (i !== -1) {
        this.$state.splice(i, 1);
      }
    }
  },
  getters: {
    latestNotification(): NotificationData | undefined {
      return this[this.length - 1];
    },
  },
});
