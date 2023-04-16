<script setup lang="ts">
import { useFinanceStore } from "/src/store/graphs";
import { invoke } from "@tauri-apps/api/tauri";
import RuleCard from "/src/components/cards/RuleCard.vue";

const graphStore = useFinanceStore();
const data = graphStore.graphData;

const updateTaxRule = async (updateData: any[]) => {
    const payload = {
        minimum_salary: Number(updateData[0]),
        tax_rate: Number(updateData[1]),
    };

    await invoke("update_rule", {
        ruleId: Rules.Tax,
        data: payload,
    });

    graphStore.setGraphData((data) => {
        data.rules.tax = { ...data.rules.tax, ...payload };
        return data;
    });
};
const onTaxRuleEnable = async (activated: boolean) => {
    if (activated) {
        invoke("enable_rule", {
            ruleId: Rules.Tax,
        });

        graphStore.setGraphData((data) => {
            data.rules.tax.enabled = true;
            return data;
        })
        return;
    }

    await invoke("disable_rule", {
        ruleId: Rules.Tax,
    });

    graphStore.setGraphData((data) => {
        data.rules.tax.enabled = false;
        return data;
    });
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

    graphStore.setGraphData((data) => {
        data.rules.business_tax = {
            ...data.rules.business_tax,
            ...payload,
        };
        return data;
    });
};
const onBusinessRuleEnabled = async (activated: boolean) => {
    if (activated) {
        await invoke("enable_rule", {
            ruleId: Rules.BusinessTax,
        });

        graphStore.setGraphData((data) => {
            data.rules.business_tax.enabled = true;
            return data;
        })
        return;
    }

    await invoke("disable_rule", {
        ruleId: Rules.BusinessTax,
    });

    graphStore.setGraphData((data) => {
        data.rules.business_tax.enabled = false;
        return data;
    })
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
