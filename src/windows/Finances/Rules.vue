<script setup lang="ts">
import { ref } from "vue";
import { useFinanceStore } from "/src/store/graphs";
import { invoke } from "@tauri-apps/api/tauri";
import RuleCard from "/src/components/cards/RuleCard.vue";

const graphStore = useFinanceStore();
const data = ref<FinanceData>(graphStore.$state.data);

const updateTaxRule = async (updateData: any[]) => {
    const payload = {
        minimum_salary: Number(updateData[0]),
        tax_rate: Number(updateData[1]),
    };

    await invoke("update_rule", {
        ruleId: Rules.Tax,
        data: payload,
    });

    data.value.rules.tax = { ...data.value.rules.tax, ...payload };
};
const onTaxRuleEnable = async (activated: boolean) => {
    if (activated) {
        invoke("enable_rule", {
            ruleId: Rules.Tax,
        });

        data.value.rules.tax.enabled = true;
        return;
    }

    await invoke("disable_rule", {
        ruleId: Rules.Tax,
    });

    data.value.rules.tax.enabled = false;
};

const updateBusinessTaxRule = async (updateData: any[]) => {
    const payload = {
        minimum_monthly_income: Number(updateData[0]),
        tax_rate: Number(updateData[1]),
    };

    await invoke("update_rule", {
        ruleId: Rules.BusinessTax,
        data: payload,
    });

    data.value.rules.business_tax = {
        ...data.value.rules.business_tax,
        ...payload,
    };
};
const onBusinessRuleEnabled = async (activated: boolean) => {
    if (activated) {
        await invoke("enable_rule", {
            ruleId: Rules.BusinessTax,
        });

        data.value.rules.business_tax.enabled = true;
        return;
    }

    await invoke("disable_rule", {
        ruleId: Rules.BusinessTax,
    });

    data.value.rules.business_tax.enabled = false;
};
</script>

<template>
    <div class="container">
        <RuleCard
            @updateRuleFn="updateTaxRule"
            @onActivationToggle="onTaxRuleEnable"
            category="Tax"
            title="Tax"
            :values="[
                {
                    startStr: `If Salary > $`,
                    value: data.rules.tax.minimum_salary,
                },
                {
                    startStr: ` then Tax = `,
                    value: data.rules.tax.tax_rate,
                    endStr: `%`,
                },
            ]"
            :data="{ 'Global Tax Rate': `${data.tax_rate}%` }"
            :enabled="data.rules.tax.enabled"
        />

        <RuleCard
            @updateRuleFn="updateBusinessTaxRule"
            @onActivationToggle="onBusinessRuleEnabled"
            category="Tax"
            title="Business Tax"
            :values="[
                {
                    startStr: `If Income > $`,
                    value: data.rules.business_tax.minimum_monthly_income,
                },
                {
                    startStr: ` then Tax = `,
                    value: data.rules.business_tax.tax_rate,
                    endStr: `%`,
                },
            ]"
            :data="{
                'Global Tax Rate': `${data.business_tax_rate}%`,
            }"
            :enabled="data.rules.business_tax.enabled"
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
