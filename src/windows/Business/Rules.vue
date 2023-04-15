<script setup lang="ts">
import { invoke } from "@tauri-apps/api/tauri";
import { useBusinessStore } from "src/store/graphs";
import { ref } from "vue";

/* BEEPBOOP BEEPBOOP (conaticus): REMEMBER TO DO ERROR HANDLING DYLAN thx */

const graphStore = useBusinessStore();
const data = ref<BusinessData>(graphStore.$state.data);

const onFundingEnabled = async (activated: boolean) => {
    if (activated) {
        await invoke("enable_rule", {
            ruleId: Rules.BusinessFunding,
        });

        data.rules.funding.enabled = true; // BEEPBOOP(conaticus): it's crying about rules and idk why :(
        return;
    }

    await invoke("disable_rule", {
        ruleId: Rules.BusinessFunding,
    }),
        (data.rules.funding.enabled = false);
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

    data.rules.funding.budget_cost = res.budget_cost;
    data.rules.funding = {
        ...data.rules.funding,
        ...payload,
    };
};
</script>

<template>
    <!-- BEEPBOOP BEEPBOOP BEEPBOOP(conaticus): Idk the fuckin syntax -->
    <!-- <RuleCard category="Funds" title="Fund low incomes" values={[ { startStr:
    "Each month, fund ", value: data.rules.funding.business_count, endStr: "
    businesses", }, { startStr: " with $", value: data.rules.funding.fund, }, {
    startStr: " with an income of $", value: data.rules.funding.maximum_income,
    endStr: " or below", }, ]} data={{
            "Budget Cost": `$${fundBudgetCost}/$${data.business_budget}`,
    }}
    onActivationToggle={onFundingEnabled} updateRuleFn={onFundingUpdate}
    enabled={data.rules.funding.enabled} /> -->
</template>
