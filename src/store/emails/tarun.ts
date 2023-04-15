import { Severity } from "../notifications";

const sender = "Tarun";

export const getTarunEmails = (username: string) => ({
  expectedCrisis: (safeBudget: number, expectedBalance: number, totalBudgetSpending: number) => ({
    title: "EXPECTED CRISIS",
    content: `
${username}! I have done some calculations and based on our statistics I estimate that we are going to experience a financial crash next month and our budget is going to fall to -$${-expectedBalance}. You need to save at least $${-expectedBalance + totalBudgetSpending} to gain a digit above $0.

To gain a safe budget with 30% spare you need to save at least $${safeBudget}.

You can save by either increasing tax rates or reducing budget allocations.

If action isn't taken, the damage could be irreversible,
Tarun.`,
    sender,
    severity: Severity.Error,
  }),
  expectedBalance: (totalBudgetSpending: number, safeBudget: number, expectedBalance: number) => ({
        title: "Expected Balance Below Safe Zone",
        content: `
Hello ${username},

I have been doing some digging and I wanted to give you a quick heads up. The expected balance next month is $${expectedBalance}.

It is generally good practise to keep at least 30% of our expected budget spending for some leeyway incase of miscalculation.
Our estimated budget spending next month is $${totalBudgetSpending}. This means that you need to save at least $${safeBudget} to maintain a safe budget.

You can save by either increasing tax rates or reducing budget allocations.

Thanks for reading and good luck,
Tarun.
                        `,
        sender,
        severity: Severity.Warning,
  }),
});
