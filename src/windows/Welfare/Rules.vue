<script setup lang="ts">
import { invoke } from "@tauri-apps/api/tauri";
import RuleCard from "src/components/cards/RuleCard.vue";
import { useWelfareStore } from "src/store/graphs";
import { ref } from "vue";

const graphStore = useWelfareStore();
const data = ref<WelfareData>(graphStore.$state.data);

// beepboop(conaticus): error handling

const onCoverEnabled = async (activated: boolean) => {
    if (activated) {
        await invoke("enable_rule", {
            ruleId: Rules.CoverFood,
        });

        data.value.rules.cover_food.enabled = true;
        return;
    }

    await invoke("disable_rule", {
        ruleId: Rules.CoverFood,
    });

    data.value.rules.cover_food.enabled = false;
};
const onCoverUpdate = async (updateData: any[]) => {
    const payload = {
        people_count: Number(updateData[0]),
        maximum_salary: Number(updateData[1]),
    };

    const res = await invoke<{ budget_cost: number }>("update_rule", {
        ruleId: Rules.CoverFood,
        data: payload,
    });

    data.value.rules.cover_food.budget_cost = res.budget_cost;
    data.value.rules.cover_food = {
        ...data.value.rules.cover_food,
        ...payload,
    };
};

const onCoverUnemployedEnabled = async (activated: boolean) => {
    if (activated) {
        await invoke("enable_rule", {
            ruleId: Rules.CoverFoodUnemployed,
        });

        data.value.rules.cover_food_unemployed.enabled = true;
        return;
    }

    await invoke("disable_rule", {
        ruleId: Rules.CoverFoodUnemployed,
    });

    data.value.rules.cover_food_unemployed.enabled = false;
};
const onCoverUnemployedUpdate = async (updateData: any[]) => {
    const payload = {
        people_count: Number(updateData[0]),
    };

    const res = await invoke<{ budget_cost: number }>("update_rule", {
        ruleId: Rules.CoverFoodUnemployed,
        data: payload,
    });

    data.value.rules.cover_food_unemployed.budget_cost = res.budget_cost;
    data.value.rules.cover_food_unemployed = {
        ...data.value.rules.cover_food_unemployed,
        ...payload,
    };
};
</script>

<template>
    <div class="container">
        <RuleCard
            category="Food Coverage"
            title="Cover Food for Everyone"
            :values="[
                {
                    startStr: `Cover food expenses for `,
                    value: data.rules.cover_food.people_count,
                    endStr: ` people`,
                },
                {
                    startStr: `
        with a salary under $`,
                    value: data.rules.cover_food.maximum_salary,
                },
            ]"
            :data="{
                'Budget Cost': `$${data.rules.cover_food.budget_cost}/${data.welfare_budget}`,
            }"
            @onActivationToggle="onCoverEnabled"
            @updateRuleFn="onCoverUpdate"
            :enabled="data.rules.cover_food.enabled"
        />

        <RuleCard
            category="Food Coverage"
            title="Cover Food for Unemployed"
            :values="[
                {
                    startStr: `Cover food expenses for `,
                    value: data.rules.cover_food_unemployed.people_count,
                    endStr: ` unemployed
        people`,
                },
            ]"
            :data="{
                'Budget Cost': `$${data.rules.cover_food_unemployed.budget_cost}/$${data.welfare_budget}`,
            }"
            @onActivationToggle="onCoverUnemployedEnabled"
            @updateRuleFn="onCoverUnemployedUpdate"
            :enabled="data.rules.cover_food_unemployed.enabled"
        />
    </div>
</template>

<style scoped>
.container {
    overflow-y: scroll;
}
.container::-webkit-scrollbar {
    display: none;
}
</style>
