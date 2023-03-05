<script lang="ts">
    import { invoke } from "@tauri-apps/api/tauri";
    import RuleCard, { Rules } from "../../templates/RuleCard.svelte";
    import type { WelfareData } from "../Welfare.svelte";

    export let data: WelfareData;

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

    const onCoverUpdate = (updateData: any[]) => {
        const payload = {
            people_count: updateData[0],
            maximum_salary: updateData[1],
        };

        invoke("update_rule", {
            ruleId: Rules.BusinessFunding,
            data: payload,
        });

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

    const onCoverUnemployedUpdate = (updateData: any[]) => {
        const payload = {
            people_count: updateData[0],
        };

        invoke("update_rule", {
            ruleId: Rules.CoverFoodUnemployed,
            data: payload,
        });

        data.rules.cover_food_unemployed = {
            ...data.rules.cover_food_unemployed,
            ...payload,
        };
    };
</script>

<div>
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
        data={{}}
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
        data={{}}
        onActivationToggle={onCoverUnemployedEnabled}
        updateRuleFn={onCoverUnemployedUpdate}
        enabled={data.rules.cover_food_unemployed.enabled}
    />
</div>
