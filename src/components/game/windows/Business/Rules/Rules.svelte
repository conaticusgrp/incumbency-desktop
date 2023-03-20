<script lang="ts">
    import { invoke } from "@tauri-apps/api/tauri";
    import { createEventDispatcher } from "svelte";
    import { handleInvoke } from "../../../../../scripts/util";
    import RuleCard, { Rules } from "../../templates/RuleCard.svelte";
    import type { BusinessData } from "../Business.svelte";

    export let data: BusinessData;
    const dispatcher = createEventDispatcher();

    let fundBudgetCost = 0;
    $: fundBudgetCost = data.rules.funding.budget_cost;

    const onFundingEnabled = async (activated: boolean) => {
        let success: any;

        if (activated) {
            success = await handleInvoke(
                dispatcher,
                invoke("enable_rule", {
                    ruleId: Rules.BusinessFunding,
                }),
                "business"
            );

            if (success !== false) {
                data.rules.funding.enabled = true;
            }

            return;
        }

        success = await handleInvoke(
            dispatcher,
            invoke("disable_rule", {
                ruleId: Rules.BusinessFunding,
            }),
            "business"
        );

        if (success !== false) {
            data.rules.funding.enabled = false;
        }
    };

    const onFundingUpdate = async (updateData: any[]) => {
        const payload = {
            business_count: Number(updateData[0]),
            fund: Number(updateData[1]),
            maximum_income: Number(updateData[2]),
        };

        const res = await handleInvoke(
            dispatcher,
            invoke("update_rule", {
                ruleId: Rules.BusinessFunding,
                data: payload,
            }),
            "business"
        );

        if (res !== false) {
            data.rules.funding.budget_cost = res.budget_cost;

            data.rules.funding = {
                ...data.rules.funding,
                ...payload,
            };
        }
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
