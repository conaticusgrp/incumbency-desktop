<script setup lang="ts">
import { invoke } from "@tauri-apps/api/tauri";
import { useHealthcareStore } from "src/store/graphs";
import { ref } from "vue";

const graphStore = useHealthcareStore();
const data = ref<HealthcareData>(graphStore.$state.data);

// beepboop(conaticus): error handling

const onAgeRuleEnabled = async (activated: boolean) => {
    let success: any;
    if (activated) {
        await invoke("enable_rule", {
            ruleId: Rules.DenyAge,
        });

        data.value.rules.deny_past_age.enabled = true;
        return;
    }

    await invoke("disable_rule", {
        ruleId: Rules.DenyAge,
    });

    data.value.rules.deny_past_age.enabled = true;
};

const onAgeRuleUpdated = async (updateData: any[]) => {
    const payload = {
        maximum_age: Number(updateData[0]),
    };

    const res = await invoke("update_rule", {
        ruleId: Rules.DenyAge,
        data: payload,
    });

    data.value.rules.deny_past_age = {
        ...data.value.rules.deny_past_age,
        ...payload,
    };
};
const onHealthRuleEnabled = async (activated: boolean) => {
    if (activated) {
        await invoke("enable_rule", {
            ruleId: Rules.DenyHealthPercentage,
        });

        data.value.rules.deny_past_health.enabled = true;
        return;
    }

    await invoke("disable_rule", {
        ruleId: Rules.DenyHealthPercentage,
    });

    data.value.rules.deny_past_health.enabled = false;
};

const onHealthRuleUpdated = async (updateData: any[]) => {
    const payload = {
        maximum_percentage: Number(updateData[0]),
    };

    await invoke("update_rule", {
        ruleId: Rules.DenyHealthPercentage,
        data: payload,
    });

    data.value.rules.deny_past_health = {
        ...data.value.rules.deny_past_health,
        ...payload,
    };
};
</script>

<template>
    <div class="container">
        <RuleCard
            category="General"
            title="Age Restrictions"
            :values="[
                {
                    startStr: `Deny healthcare if age > `,
                    value: data.rules.deny_past_age.maximum_age,
                },
            ]"
            :detail="{}"
            @onActivationToggle="onAgeRuleEnabled"
            @updateRuleFn="onAgeRuleUpdated"
            :enabled="data.rules.deny_past_age.enabled"
        />

        <RuleCard
            category="General"
            title="Health Restrictions"
            :values="[
                {
                    startStr: `Deny healthcare if health > `,
                    value: data.rules.deny_past_health.maximum_percentage,
                    endStr: `%`,
                },
            ]"
            :detail="{}"
            @onActivationToggle="onHealthRuleEnabled"
            @updateRuleFn="onHealthRuleUpdated"
            :enabled="data.rules.deny_past_health.enabled"
        />
    </div>
</template>

<style>
.container {
    overflow-y: scroll;
}
.container::-webkit-scrollbar {
    display: none;
}
</style>
