<script lang="ts">
    import { invoke } from "@tauri-apps/api/tauri";
    import RuleCard, { Rules } from "../../templates/RuleCard.svelte";
    import type { HealthcareData } from "../Healthcare.svelte";

    export let data: HealthcareData;

    const onAgeRuleEnabled = (activated: boolean) => {
        data.rules.deny_past_age.enabled = activated;

        if (activated) {
            invoke("enable_rule", {
                ruleId: Rules.DenyAge,
            });

            return;
        }

        invoke("disable_rule", {
            ruleId: Rules.DenyAge,
        });
    };

    const onAgeRuleUpdated = (updateData: any[]) => {
        const payload = {
            maximum_age: Number(updateData[0]),
        };

        invoke("update_rule", {
            ruleId: Rules.DenyAge,
            data: payload,
        });

        data.rules.deny_past_age = { ...data.rules.deny_past_age, ...payload };
    };

    const onHealthRuleEnabled = (activated: boolean) => {
        data.rules.deny_past_health.enabled = activated;

        if (activated) {
            invoke("enable_rule", {
                ruleId: Rules.DenyHealthPercentage,
            });

            return;
        }

        invoke("disable_rule", {
            ruleId: Rules.DenyHealthPercentage,
        });
    };

    const onHealthRuleUpdated = (updateData: any[]) => {
        const payload = {
            maximum_percentage: Number(updateData[0]),
        };

        invoke("update_rule", {
            ruleId: Rules.DenyHealthPercentage,
            data: payload,
        });

        data.rules.deny_past_health = {
            ...data.rules.deny_past_health,
            ...payload,
        };
    };
</script>

<div class="container">
    <RuleCard
        category="General"
        title="Age Restrictions"
        values={[
            {
                startStr: "Deny healthcare if age > ",
                value: data.rules.deny_past_age.maximum_age,
            },
        ]}
        data={{}}
        onActivationToggle={onAgeRuleEnabled}
        updateRuleFn={onAgeRuleUpdated}
        enabled={data.rules.deny_past_age.enabled}
    />

    <RuleCard
        category="General"
        title="Health Restrictions"
        values={[
            {
                startStr: "Deny healthcare if health > ",
                value: data.rules.deny_past_health.maximum_percentage,
                endStr: "%",
            },
        ]}
        data={{}}
        onActivationToggle={onHealthRuleEnabled}
        updateRuleFn={onHealthRuleUpdated}
        enabled={data.rules.deny_past_health.enabled}
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
