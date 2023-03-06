<script lang="ts">
    import { invoke } from "@tauri-apps/api/tauri";
    import RuleCard, { Rules } from "../../templates/RuleCard.svelte";
    import type { WelfareData } from "../Welfare.svelte";

    export let data: WelfareData;

    let coverCost = 0;
    let unemployedCoverCost = 0;

    $: coverCost = data.rules.cover_food.budget_cost;
    $: unemployedCoverCost = data.rules.cover_food_unemployed.budget_cost;

    const onCoverEnabled = (activated: boolean) => {
        data.rules.cover_food.enabled = activated;

        if (activated) {
            invoke("enable_rule", {
                ruleId: Rules.CoverFood,
            });

            return;
        }

        invoke("disable_rule", {
            ruleId: Rules.CoverFood,
        });
    };

    const onCoverUpdate = async (updateData: any[]) => {
        const payload = {
            people_count: Number(updateData[0]),
            maximum_salary: Number(updateData[1]),
        };

        const { budget_cost } = (await invoke("update_rule", {
            ruleId: Rules.CoverFood,
            data: payload,
        })) as any;

        data.rules.cover_food.budget_cost = budget_cost;

        data.rules.cover_food = {
            ...data.rules.cover_food,
            ...payload,
        };
    };

    const onCoverUnemployedEnabled = (activated: boolean) => {
        data.rules.cover_food_unemployed.enabled = activated;

        if (activated) {
            invoke("enable_rule", {
                ruleId: Rules.CoverFoodUnemployed,
            });

            return;
        }

        invoke("disable_rule", {
            ruleId: Rules.CoverFoodUnemployed,
        });
    };

    const onCoverUnemployedUpdate = async (updateData: any[]) => {
        const payload = {
            people_count: Number(updateData[0]),
        };

        const { budget_cost } = (await invoke("update_rule", {
            ruleId: Rules.CoverFoodUnemployed,
            data: payload,
        })) as any;

        data.rules.cover_food_unemployed.budget_cost = budget_cost;

        data.rules.cover_food_unemployed = {
            ...data.rules.cover_food_unemployed,
            ...payload,
        };
    };
</script>

<div class="container">
    <RuleCard
        category="Food Coverage"
        title="Cover Food for Everyone"
        values={[
            {
                startStr: "Cover food expenses for ",
                value: data.rules.cover_food.people_count,
                endStr: " people",
            },
            {
                startStr: " with a salary under $",
                value: data.rules.cover_food.maximum_salary,
            },
        ]}
        data={{
            "Budget Cost": `$${coverCost}/${data.welfare_budget}`,
        }}
        onActivationToggle={onCoverEnabled}
        updateRuleFn={onCoverUpdate}
        enabled={data.rules.cover_food.enabled}
    />

    <RuleCard
        category="Food Coverage"
        title="Cover Food for Unemployed"
        values={[
            {
                startStr: "Cover food expenses for ",
                value: data.rules.cover_food_unemployed.people_count,
                endStr: " unemployed people",
            },
        ]}
        data={{
            "Budget Cost": `$${unemployedCoverCost}/$${data.welfare_budget}`,
        }}
        onActivationToggle={onCoverUnemployedEnabled}
        updateRuleFn={onCoverUnemployedUpdate}
        enabled={data.rules.cover_food_unemployed.enabled}
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
