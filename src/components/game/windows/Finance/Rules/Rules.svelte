<script lang="ts">
    import type { FinanceData } from "../Finance.svelte";
    import RuleCard, { Rules } from "../../templates/RuleCard.svelte";
    import { invoke } from "@tauri-apps/api/tauri";

    export let data: FinanceData;

    const updateTaxRule = (updateData: any[]) => {
        const payload = {
            minimum_salary: Number(updateData[0]),
            tax_rate: Number(updateData[1]),
        };

        invoke("update_rule", {
            ruleId: Rules.Tax,
            data: payload,
        });

        data.rules.tax = { ...data.rules.tax, ...payload };
    };

    const onTaxRuleEnable = (activated: boolean) => {
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
    };

    const updateBusinessTaxRule = (updateData: any[]) => {
        const payload = {
            minimum_monthly_income: Number(updateData[0]),
            tax_rate: Number(updateData[1]),
        };

        invoke("update_rule", {
            ruleId: Rules.BusinessTax,
            data: payload,
        });

        data.rules.business_tax = { ...data.rules.business_tax, ...payload };
    };

    const onBusinessRuleEnabled = (activated: boolean) => {
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
    };
</script>

<div class="container">
    <RuleCard
        updateRuleFn={updateTaxRule}
        onActivationToggle={onTaxRuleEnable}
        category="Tax"
        title="Tax"
        values={[
            {
                startStr: "If Salary > $",
                value: data.rules.tax.minimum_salary,
            },
            {
                startStr: " then Tax = ",
                value: data.rules.tax.tax_rate,
                endStr: "%",
            },
        ]}
        data={{
            "Global Tax Rate": `${data.tax_rate}%`,
        }}
        enabled={data.rules.tax.enabled}
    />

    <RuleCard
        updateRuleFn={updateBusinessTaxRule}
        onActivationToggle={onBusinessRuleEnabled}
        category="Tax"
        title="Business Tax"
        values={[
            {
                startStr: "If Income > $",
                value: data.rules.business_tax.minimum_monthly_income,
            },
            {
                startStr: " then Tax = ",
                value: data.rules.business_tax.tax_rate,
                endStr: "%",
            },
        ]}
        data={{
            "Global Tax Rate": `${data.business_tax_rate}%`,
        }}
        enabled={data.rules.business_tax.enabled}
    />
</div>

<style>
    .container {
        overflow-y: scroll;
    }

    .container::-webkit-scrollbar {
        display: none;
    }
</style>
