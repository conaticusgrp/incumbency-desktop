import { Severity } from "../notifications";

export const getRalphEmails = (username: string) => ({
  highUneploymentEmail: (percent: number, unemployedCount: number) => ({
    title: "High Unemployment Rate",
    content: `
Hi ${username}, hope you're doing well. It has been brought to my attention that the unemployment rate for the country needs to be addressed, as it currently sits at ${unemployedCount} people (${Number(percent)}%).

This was likely caused by a large coorporation going bust. Ensure that you cover expenses for these people to keep them healthy while they seek for new employment.

Many thanks, Ralph
`,
    sender: "Ralph",
    severity: Severity.Warning,
  }),
  businessBust: (percent: number, unemployedCount: number) => ({
      title: "Huge business has gone bust!",
      content: `
Hello ${username}, you need to act ASAP! A large business has just gone bust and ${unemployedCount} people (${Number(percent)}%) are now unemployed. Fund expenses for as many people as you can while they seek new employment.`,
      sender: "Ralph",
      severity: Severity.Error,
  }),
});
