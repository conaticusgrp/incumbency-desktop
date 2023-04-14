//       emits("emailCreate", {
//         title: "EXPECTED CRISIS",
//         content: `
// ${USERNAME}! I have done some calculations and based on our statistics I estimate that we are going to experience a financial crash next month and our budget is going to fall to -$${-appData.expected_balance}. You need to save at least $${-appData.expected_balance + totalBudgetSpending} to gain a digit above $0.

// To gain a safe budget with 30% spare you need to save at least $${-appData.expected_balance + totalBudgetSpending + totalBudgetSpending * 0.3}.

// You can save by either increasing tax rates or reducing budget allocations.

// If action isn't taken, the damage could be irreversible,
// Tarun.`,
//         sender: "Tarun",
//         severity: "error",
//       });
import { defineStore } from "pinia";
import { Severity } from "./notifications";

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
