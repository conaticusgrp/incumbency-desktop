<script lang="ts">
    import { invoke } from "@tauri-apps/api/tauri";
    import RuleCard, { Rules } from "../../templates/RuleCard.svelte";
    import type { BusinessData } from "../Business.svelte";

    export let data: BusinessData;

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

    const onFundingUpdate = (updateData: any[]) => {
        const payload = {
            business_count: updateData[0],
            fund: updateData[1],
            maximum_income: updateData[2],
        };

        invoke("update_rule", {
            ruleId: Rules.BusinessFunding,
            data: payload,
        });

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
        data={{}}
        onActivationToggle={onFundingEnabled}
        updateRuleFn={onFundingUpdate}
        enabled={data.rules.funding.enabled}
    />
</div>
