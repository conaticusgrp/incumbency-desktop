<script lang="ts">
    import type { FinanceData } from "../Finance.svelte";
    import RuleCard, { Rules } from "../../templates/RuleCard.svelte";
    import { invoke } from "@tauri-apps/api/tauri";
    import { handleInvoke } from "../../../../../scripts/util";
    import { createEventDispatcher } from "svelte";

    export let data: FinanceData;
    const dispatcher = createEventDispatcher();

    const updateTaxRule = async (updateData: any[]) => {
        const payload = {
            minimum_salary: Number(updateData[0]),
            tax_rate: Number(updateData[1]),
        };

        const res = await handleInvoke(
            dispatcher,
            invoke("update_rule", {
                ruleId: Rules.Tax,
                data: payload,
            }),
            "finance"
        );

        if (res !== false) {
            data.rules.tax = { ...data.rules.tax, ...payload };
        }
    };

    const onTaxRuleEnable = async (activated: boolean) => {
        let success: any;

        if (activated) {
            success = await handleInvoke(
                dispatcher,
                invoke("enable_rule", {
                    ruleId: Rules.Tax,
                }),
                "finance"
            );

            if (success !== false) {
                data.rules.tax.enabled = true;
            }

            return;
        }

        success = await handleInvoke(
            dispatcher,
            invoke("disable_rule", {
                ruleId: Rules.Tax,
            }),
            "finance"
        );

        if (success !== false) {
            data.rules.tax.enabled = false;
        }
    };

    const updateBusinessTaxRule = async (updateData: any[]) => {
        const payload = {
            minimum_monthly_income: Number(updateData[0]),
            tax_rate: Number(updateData[1]),
        };

        const res = await handleInvoke(
            dispatcher,
            invoke("update_rule", {
                ruleId: Rules.BusinessTax,
                data: payload,
            }),
            "finance"
        );

        if (res !== false) {
            data.rules.business_tax = {
                ...data.rules.business_tax,
                ...payload,
            };
        }
    };

    const onBusinessRuleEnabled = async (activated: boolean) => {
        let success: any;

        if (activated) {
            success = await handleInvoke(
                dispatcher,
                invoke("enable_rule", {
                    ruleId: Rules.BusinessTax,
                }),
                "finance"
            );

            if (success !== false) {
                data.rules.business_tax.enabled = true;
            }

            return;
        }

        success = await handleInvoke(
            dispatcher,
            invoke("disable_rule", {
                ruleId: Rules.BusinessTax,
            }),
            "finance"
        );

        if (success !== false) {
            data.rules.business_tax.enabled = false;
        }
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
