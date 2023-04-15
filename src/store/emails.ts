import { defineStore } from "pinia";
import { Severity } from "./notifications";
import { getRalphEmails } from "./emails/ralph";
import { getTarunEmails } from "./emails/tarun";

export type EmailCreate = {
  title: string;
  content: string;
  sender: string;
  severity: Severity;
}

export const useEmailsStore = defineStore('emails', {
  state: () => ([] as EmailCreate[]),
  actions: {
    createEmail(email: EmailCreate) {
      this.$state.push(email);
    },
  },
  getters: {
    latestEmail(): EmailCreate | undefined {
      return this[this.length - 1];
    }
  }
});

export const useEmails = (username: string) => {
  return {
    ralph: getRalphEmails(username),
    tarun: getTarunEmails(username),
  }
}
