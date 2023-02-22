<script lang="ts">
    import { invoke } from "@tauri-apps/api/tauri";
    import type { FinanceData } from "../Finance.svelte";
    import BudgetCard from "./BudgetCard.svelte";

    export let data: FinanceData | undefined;

    enum GameValue {
        TaxRate,
        BusinessTaxRate,
        HealthcareBudget,
        WelfareBudget,
        BusinessBudget,
        SpareBudget,
    }

    /*
        TODO:
            - Handle update tax rate return value
    */

    const updateGameValue = (gameValue: GameValue, newValue: any): any => {
        if (!data) throw new Error("Data is undefined.");

        switch (gameValue) {
            case GameValue.TaxRate:
                data.tax_rate = newValue;
                return invoke("update_tax_rate", { taxRate: newValue });
            case GameValue.BusinessTaxRate:
                data.business_tax_rate = newValue;
                return invoke("update_business_tax_rate", { taxRate: newValue })
            case GameValue.HealthcareBudget:
                data.healthcare_budget = newValue;
                return invoke("update_healthcare_budget", { newBudget: newValue })
            case GameValue.WelfareBudget:
                data.welfare_budget = newValue;
                return invoke("update_welfare_budget", { newBudget: newValue })
            case GameValue.BusinessBudget:
                data.business_budget = newValue;
                return invoke("update_business_budget", { newBudget: newValue })
            default: break;
        }
    }
</script>

<main>
    {#if data != null}
        <h1>TAXES</h1>

        <BudgetCard title="Tax Rate" appendValueEnd="%" currentValue={data.tax_rate} data={{}} assignValueFn={(val) => updateGameValue(GameValue.TaxRate, Number(val))} />
        <BudgetCard title="Business Tax Rate" appendValueEnd="%" currentValue={data.business_tax_rate} data={{}} assignValueFn={(val) => updateGameValue(GameValue.BusinessTaxRate, Number(val))} />

        <h1>BUDGETS</h1>

        <BudgetCard title="Healthcare Budget" appendValueStart="$" currentValue={data.healthcare_budget} data={{}} assignValueFn={(val) => updateGameValue(GameValue.HealthcareBudget, Number(val))} />
        <BudgetCard title="Welfare Budget" appendValueStart="$" currentValue={data.welfare_budget} data={{}} assignValueFn={(val) => updateGameValue(GameValue.WelfareBudget, Number(val))} />
        <BudgetCard title="Business Budget" appendValueStart="$" currentValue={data.business_budget} data={{}} assignValueFn={(val) => updateGameValue(GameValue.BusinessBudget, Number(val))} />
        <BudgetCard title="Spare Budget" hasAmendButton={false} appendValueStart="$" currentValue={data.spare_budget} data={{}} assignValueFn={(val) => updateGameValue(GameValue.SpareBudget, Number(val))} />

    {/if}
</main>

<style>
    main {
        width: 100%;
        height: 100%;
        overflow: scroll; 
        position: relative;
    }

    h1 {
        font-size: 22px;
        font-weight: bold;
        margin-top: 20px;
    }
</style>