<script lang="ts">
    import { invoke } from "@tauri-apps/api/tauri";
    import type { FinanceData } from "../Finance.svelte";
    import ValueCard from "../../templates/ValueCard.svelte";
    import { createEventDispatcher } from "svelte";
    import { handleInvoke } from "../../../../../scripts/util";

    export let data: FinanceData | undefined;
    const dispatcher = createEventDispatcher();

    enum GameValue {
        TaxRate,
        BusinessTaxRate,
        HealthcareBudget,
        WelfareBudget,
        BusinessBudget,
        SpareBudget,
    }

    const updateGameValue = async (gameValue: GameValue, newValue: any) => {
        if (!data) throw new Error("Data is undefined.");

        switch (gameValue) {
            case GameValue.TaxRate:
                data.tax_rate = newValue;
                data.expected_person_income = await invoke("update_tax_rate", {
                    taxRate: newValue,
                });
                break;
            case GameValue.BusinessTaxRate:
                data.business_tax_rate = newValue;
                data.expected_business_income = await invoke(
                    "update_business_tax_rate",
                    { taxRate: newValue }
                );
                break;
            case GameValue.HealthcareBudget:
                const healthRes = await handleInvoke(
                    dispatcher,
                    invoke("update_healthcare_budget", {
                        newBudget: newValue,
                    })
                );

                if (healthRes === false) {
                    break;
                }

                data.healthcare_budget = newValue;
                data.used_hospital_capacity = healthRes.used_hospital_capacity;
                data.total_hospital_capacity =
                    healthRes.total_hospital_capacity;
                break;
            case GameValue.WelfareBudget:
                const welfareRes = await handleInvoke(
                    dispatcher,
                    invoke("update_welfare_budget", {
                        newBudget: newValue,
                    })
                );

                if (welfareRes === false) {
                    break;
                }

                data.welfare_budget = newValue;
                break;
            case GameValue.BusinessBudget:
                const busRes = await handleInvoke(
                    dispatcher,
                    invoke("update_business_budget", {
                        newBudget: newValue,
                    })
                );

                if (busRes === false) {
                    break;
                }

                data.business_budget = newValue;
                break;
            default:
                break;
        }
    };
</script>

<main>
    {#if data != null}
        <h1>TAXES</h1>

        <ValueCard
            title="Tax Rate"
            appendValueEnd="%"
            currentValue={data.tax_rate}
            data={{
                "Expected Tax Income": `$${data.expected_person_income}`,
            }}
            assignValueFn={(val) =>
                updateGameValue(GameValue.TaxRate, Number(val))}
        />
        <ValueCard
            title="Business Tax"
            appendValueEnd="%"
            currentValue={data.business_tax_rate}
            data={{
                "Expected Business Tax Income": `$${data.expected_business_income}`,
            }}
            assignValueFn={(val) =>
                updateGameValue(GameValue.BusinessTaxRate, Number(val))}
        />

        <h1>BUDGETS</h1>

        <ValueCard
            title="Healthcare"
            appendValueStart="$"
            currentValue={data.healthcare_budget}
            data={{
                Capacity: `${data.used_hospital_capacity}/${
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
            }}
            assignValueFn={(val) =>
                updateGameValue(GameValue.HealthcareBudget, Number(val))}
        />
        <ValueCard
            title="Welfare"
            appendValueStart="$"
            currentValue={data.welfare_budget}
            data={{
                "Average Welfare": `${data.average_welfare}%`,
                "Average Unemployed Welfare": `${data.average_welfare_unemployed}%`,
            }}
            assignValueFn={(val) =>
                updateGameValue(GameValue.WelfareBudget, Number(val))}
        />
        <ValueCard
            title="Business"
            appendValueStart="$"
            currentValue={data.business_budget}
            data={{}}
            assignValueFn={(val) =>
                updateGameValue(GameValue.BusinessBudget, Number(val))}
        />
        <ValueCard
            title="Spare"
            hasAmendButton={false}
            appendValueStart="$"
            currentValue={data.spare_budget}
            data={{}}
            assignValueFn={(val) =>
                updateGameValue(GameValue.SpareBudget, Number(val))}
        />
    {/if}
</main>

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
