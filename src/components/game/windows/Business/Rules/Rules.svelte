<script lang="ts">
    import { invoke } from "@tauri-apps/api/tauri";
    import RuleCard, { Rules } from "../../templates/RuleCard.svelte";
    import type { BusinessData } from "../Business.svelte";

    export let data: BusinessData;

    let fundBudgetCost = 0;
    $: fundBudgetCost = data.rules.funding.budget_cost;

    const onFundingEnabled = (activated: boolean) => {
        data.rules.funding.enabled = activated;

        if (activated) {
            invoke("enable_rule", {
                ruleId: Rules.BusinessFunding,
            });

            return;
        }

        invoke("disable_rule", {
            ruleId: Rules.BusinessFunding,
        });
    };

    const onFundingUpdate = async (updateData: any[]) => {
        const payload = {
            business_count: Number(updateData[0]),
            fund: Number(updateData[1]),
            maximum_income: Number(updateData[2]),
        };

        const { budget_cost } = (await invoke("update_rule", {
            ruleId: Rules.BusinessFunding,
            data: payload,
        })) as any;

        data.rules.funding.budget_cost = budget_cost;

        data.rules.funding = {
            ...data.rules.funding,
            ...payload,
        };
    };
</script>

<div>
    <RuleCard
        category="Funds"
        title="Fund low incomes"
        values={[
            {
                startStr: "Each month, fund ",
                value: data.rules.funding.business_count,
                endStr: " businesses",
            },
            {
                startStr: " with $",
                value: data.rules.funding.fund,
            },
            {
                startStr: " with an income of $",
                value: data.rules.funding.maximum_income,
                endStr: " or below",
            },
        ]}
        data={{
            "Budget Cost": `$${fundBudgetCost}/$${data.business_budget}`,
        }}
        onActivationToggle={onFundingEnabled}
        updateRuleFn={onFundingUpdate}
        enabled={data.rules.funding.enabled}
    />
</div>
