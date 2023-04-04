<script lang="ts">
    import { invoke } from "@tauri-apps/api/tauri";
    import { createEventDispatcher } from "svelte";
    import { handleInvoke } from "../../../../../scripts/util";
    import RuleCard, { Rules } from "../../templates/RuleCard.svelte";
    import type { HealthcareData } from "../Healthcare.svelte";

    export let data: HealthcareData;
    const dispatcher = createEventDispatcher();

    const onAgeRuleEnabled = async (activated: boolean) => {
        let success: any;

        if (activated) {
            success = await handleInvoke(
                dispatcher,
                invoke("enable_rule", {
                    ruleId: Rules.DenyAge,
                }),
                "healthcare"
            );

            if (success !== false) {
                data.rules.deny_past_age.enabled = true;
            }

            return;
        }

        success = await handleInvoke(
            dispatcher,
            invoke("disable_rule", {
                ruleId: Rules.DenyAge,
            }),
            "healthcare"
        );

        if (success !== false) {
            data.rules.deny_past_age.enabled = false;
        }
    };

    const onAgeRuleUpdated = async (updateData: any[]) => {
        const payload = {
            maximum_age: Number(updateData[0]),
        };

        const res = await handleInvoke(
            dispatcher,
            invoke("update_rule", {
                ruleId: Rules.DenyAge,
                data: payload,
            }),
            "healthcare"
        );

        if (res !== false) {
            data.rules.deny_past_age = {
                ...data.rules.deny_past_age,
                ...payload,
            };
        }
    };

    const onHealthRuleEnabled = async (activated: boolean) => {
        let success: any;

        if (activated) {
            success = await handleInvoke(
                dispatcher,
                invoke("enable_rule", {
                    ruleId: Rules.DenyHealthPercentage,
                }),
                "healthcare"
            );

            if (success !== false) {
                data.rules.deny_past_health.enabled = true;
            }

            return;
        }

        success = await handleInvoke(
            dispatcher,
            invoke("disable_rule", {
                ruleId: Rules.DenyHealthPercentage,
            }),
            "healthcare"
        );

        if (success !== false) {
            data.rules.deny_past_health.enabled = false;
        }
    };

    const onHealthRuleUpdated = async (updateData: any[]) => {
        const payload = {
            maximum_percentage: Number(updateData[0]),
        };

        const res = await handleInvoke(
            dispatcher,
            invoke("update_rule", {
                ruleId: Rules.DenyHealthPercentage,
                data: payload,
            }),
            "healthcare"
        );

        if (res !== false) {
            data.rules.deny_past_health = {
                ...data.rules.deny_past_health,
                ...payload,
            };
        }
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
