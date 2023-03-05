<script lang="ts">
    import type { FinanceData } from "../Finance.svelte";
    import RuleCard, { RuleCardType, Rules } from "../../templates/RuleCard.svelte";
    import { invoke } from "@tauri-apps/api/tauri";

    export let data: FinanceData;
</script>

<div>
    <RuleCard updateRuleFn={(updateData) => {
        const payload = {
            minimum_salary: Number(updateData[0]),
            tax_rate: Number(updateData[1]),
        }

        invoke("update_rule", {
            ruleId: Rules.Tax,
            data: payload,
        })

        data.rules.tax = { ...data.rules.tax, ...payload };
    }} onActivationToggle={(activated) => {
        data.rules.tax.enabled = activated;

        if (activated) {
            invoke("enable_rule", {
                ruleId: Rules.Tax,
            });

            return;
        }

        invoke("disable_rule", {
            ruleId: Rules.Tax,
        });
    }} type={RuleCardType.IfThen} category="Tax" title="Tax" values={[
        {
            str: "Salary >",
            value: data.rules.tax.minimum_salary,
        },
        {
            str: "Tax =",
            value: data.rules.tax.tax_rate,
        }
    ]} data={{
        "Global Tax Rate": `${data.tax_rate}%`
    }} enabled={data.rules.tax.enabled} />

    <RuleCard updateRuleFn={(updateData) => {
        const payload = {
            minimum_monthly_income: Number(updateData[0]),
            tax_rate: Number(updateData[1]),
        }

        invoke("update_rule", {
            ruleId: Rules.BusinessTax,
            data: payload,
        })

        data.rules.business_tax = { ...data.rules.business_tax, ...payload };
    }} onActivationToggle={(activated) => {
        data.rules.business_tax.enabled = activated;

        if (activated) {
            invoke("enable_rule", {
                ruleId: Rules.BusinessTax,
            });

            return;
        }

        invoke("disable_rule", {
            ruleId: Rules.BusinessTax,
        });
    }} type={RuleCardType.IfThen} category="Tax" title="Business Tax" values={[
        {
            str: "Income >",
            value: data.rules.business_tax.minimum_monthly_income,
        },
        {
            str: "Tax =",
            value: data.rules.business_tax.tax_rate,
        }
    ]} data={{
        "Global Tax Rate": `${data.business_tax_rate}%`
    }} enabled={data.rules.business_tax.enabled} />
</div>