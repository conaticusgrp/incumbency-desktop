<script setup lang="ts">
import { handleInvoke } from "src/util/events";
import { useFinanceStore } from "src/store/graphs";
import { ref } from "vue";
import ValueCard from "../../components/cards/ValueCard.vue";

type HealthRes = {
    used_hospital_capacity: number;
    total_hospital_capacity: number;
};

enum GameValue {
    TaxRate,
    BusinessTaxRate,
    HealthcareBudget,
    WelfareBudget,
    BusinessBudget,
    SpareBudget,
}

const graphStore = useFinanceStore();
const data = graphStore.graphData;
const app = "Finance";

// NOTE(dylhack): side effect
const updateGameValue = async (gameValue: GameValue, newValue: number) => {
    if (gameValue === GameValue.TaxRate) {
        const { value: taxRes, success } = await handleInvoke<number>(
            app,
            "update_tax_rate",
            { taxRate: newValue }
        );

        if (success) {
            graphStore.setGraphData((data) => {
                data.tax_rate = newValue;
                data.expected_person_income = taxRes;
                return data;
            });
        }
    } else if (gameValue === GameValue.BusinessTaxRate) {
        const { success } = await handleInvoke(
            app,
            "update_business_tax_rate",
            { taxRate: newValue }
        );

        if (success) {
            graphStore.setGraphData((data) => {
                data.business_tax_rate = newValue
                return data;
            });
        }
    } else if (gameValue === GameValue.HealthcareBudget) {
        const { value: healthRes, success } = await handleInvoke<HealthRes>(
            app,
            "update_healthcare_budget",
            { newBudget: newValue }
        );

        if (success) {
            graphStore.setGraphData((data) => {
                data.healthcare_budget = newValue;
                data.used_hospital_capacity = healthRes.used_hospital_capacity;
                data.total_hospital_capacity = healthRes.total_hospital_capacity;
                return data;
            });
        }
    } else if (gameValue === GameValue.WelfareBudget) {
        const { success } = await handleInvoke(app, "update_welfare_budget", {
            newBudget: newValue,
        });

        if (success) {
            graphStore.setGraphData((data) => {
                data.welfare_budget = newValue;
                return data;
            });
        }
    } else if (gameValue === GameValue.BusinessBudget) {
        const { success } = await handleInvoke(app, "update_business_budget", {
            newBudget: newValue,
        });

        if (success) {
            graphStore.setGraphData((data) => {
                data.business_budget = newValue;
                return data;
            });
        }
    }
};
</script>

<template>
    <div v-if="data">
        <h1>TAXES</h1>
        <ValueCard
            title="Tax Rate"
            :value="data.tax_rate"
            @amend="updateGameValue(GameValue.TaxRate, $event.valueOf())"
            :detail="{ 'Expected Tax Income': data.expected_person_income }"
        />
        <ValueCard
            title="Business Tax"
            :value="data.business_tax_rate"
            @amend="
                updateGameValue(GameValue.BusinessTaxRate, $event.valueOf())
            "
            :detail="{
                'Expected Business Tax Income': data.expected_business_income,
            }"
        />
        <h1>BUDGETS</h1>
        <ValueCard
            title="Healthcare"
            :value="data.healthcare_budget"
            @amend="
                updateGameValue(GameValue.HealthcareBudget, $event.valueOf())
            "
            :detail="{
                'Used Capacity': `${data.used_hospital_capacity}/${
                    data.total_hospital_capacity
                } (${
                    data
                        ? Math.floor(
                              (data.used_hospital_capacity /
                                  data.total_hospital_capacity) *
                                  100
                          )
                        : 0
                }%)`,
                'Spare Capacity': data.spare_hospital_capacity,
            }"
        />
        <ValueCard
            title="Welfare"
            :value="data.welfare_budget"
            @amend="updateGameValue(GameValue.WelfareBudget, $event.valueOf())"
            :detail="{
                'Used Budget': `$${data.used_welfare_budget}/${
                    data.welfare_budget
                } (${
                    data.welfare_budget != 0
                        ? Math.floor(
                              (data.used_welfare_budget / data.welfare_budget) *
                                  100
                          )
                        : 0
                }%)`,
                'Average Welfare': `${data.average_welfare}%`,
                'Average Unemployed Welfare': `${data.average_welfare_unemployed}%`,
            }"
        />
        <ValueCard
            title="Business"
            :value="data.business_budget"
            @amend="updateGameValue(GameValue.BusinessBudget, $event.valueOf())"
            :detail="{
                'Used Budget': `$${data.used_business_budget}/${
                    data.business_budget
                } (${
                    data.business_budget != 0
                        ? Math.floor(
                              (data.used_business_budget /
                                  data.business_budget) *
                                  100
                          )
                        : 0
                }%)`,
            }"
        />
        <h1>TAXES</h1>
        <ValueCard
            title="Spare"
            :hasAmendButton="false"
            :value="data.spare_budget"
            :detail="{}"
        />
    </div>
</template>

<style>
main {
    width: 100%;
    height: 100%;
    overflow-y: scroll;
}

main::-webkit-scrollbar {
    display: none;
}

h1 {
    font-size: 25px;
    font-weight: bold;
    margin: 20px 0;
}
</style>
