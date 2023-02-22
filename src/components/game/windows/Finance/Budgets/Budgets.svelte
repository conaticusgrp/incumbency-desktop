<script lang="ts">
  import type { FinanceData } from "../Finance.svelte";
  import BudgetCard from "./BudgetCard.svelte";

  export let data: FinanceData | undefined;
</script>

<main>
    {#if data != null}
        <h1>TAXES</h1>

        <BudgetCard title="Tax Rate" currentValue={`${data.tax_rate}%`} data={{}} assignValueFn={() => {}} />
        <BudgetCard title="Business Tax Rate" currentValue={`${data.business_tax_rate}%`} data={{}} assignValueFn={() => {}} />

        <h1>BUDGETS</h1>

        <BudgetCard title="Healthcare Budget" currentValue={`$${data.healthcare_budget}`} data={{}} assignValueFn={() => {}} />
        <BudgetCard title="Welfare Budget" currentValue={`$${data.welfare_budget}`} data={{}} assignValueFn={() => {}} />
        <BudgetCard title="Business Budget" currentValue={`$${data.business_budget}`} data={{}} assignValueFn={() => {}} />
        <BudgetCard title="Spare Budget" currentValue={`$${data.spare_budget}`} data={{}} assignValueFn={() => {}} />

    {/if}
</main>

<style>
    main {
        width: 100%;
        height: 100%;
        overflow: scroll; 
    }

    h1 {
        font-size: 22px;
        font-weight: bold;
        margin-top: 20px;
    }
</style>
