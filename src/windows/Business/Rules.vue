<script setup lang="ts">
import { invoke } from "@tauri-apps/api/tauri";
import RuleCard from "src/components/cards/RuleCard.vue";
import { useBusinessStore } from "src/store/graphs";

/* BEEPBOOP BEEPBOOP (conaticus): REMEMBER TO DO ERROR HANDLING DYLAN thx */

const graphStore = useBusinessStore();
const data = graphStore.$state.data;

const onFundingEnabled = async (activated: boolean) => {
    if (activated) {
        await invoke("enable_rule", {
            ruleId: Rules.BusinessFunding,
        });

        graphStore.setGraphData((data) => {
            data.rules.funding.enabled = true;
            return data;
        });
        return;
    }

    await invoke("disable_rule", {
        ruleId: Rules.BusinessFunding,
    });

    graphStore.setGraphData((data) => {
        data.rules.funding.enabled = false;
        return data;
    });
};

const onFundingUpdate = async (updateData: any[]) => {
    const payload = {
        business_count: Number(updateData[0]),
        fund: Number(updateData[1]),
        maximum_income: Number(updateData[2]),
    };

    const res = await invoke<{ budget_cost: number }>("update_rule", {
        ruleId: Rules.BusinessFunding,
        data: payload,
    });

    graphStore.setGraphData((data) => {
        data.rules.funding.budget_cost = res.budget_cost;
        data.rules.funding = {
            ...data.rules.funding,
            ...payload,
        };
        return data;
    });
};
</script>

<template>
    <RuleCard
        category="Funds"
        title="Fund low incomes"
        :values="[
            {
                startStr: `Each month, fund `,
                value: data.rules.funding.business_count,
                endStr: `businesses`,
            },
            { startStr: ` with $`, value: data.rules.funding.fund },
            {
                startStr: ` with an income of $`,
                value: data.rules.funding.maximum_income,
                endStr: ` or below`,
            },
        ]"
        :data="{
            'Budget Cost': `$${data.rules.funding.budget_cost}/$${data.business_budget}`,
        }"
        @onActivationToggle="onFundingEnabled"
        @updateRuleFn="onFundingUpdate"
        :enabled="data.rules.funding.enabled"
    />
</template>
