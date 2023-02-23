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
        const healthRes: any = await invoke("update_healthcare_budget", {
          newBudget: newValue,
        });

        if (healthRes.error) {
          // TODO: handle error in notifications
          console.error(healthRes.error);
          break;
        }

        data.healthcare_budget = newValue;
        data.used_hospital_capacity = healthRes.used_hospital_capacity;
        data.total_hospital_capacity = healthRes.total_hospital_capacity;
        break;
      case GameValue.WelfareBudget:
        const welfareRes: any = await invoke("update_welfare_budget", {
          newBudget: newValue,
        });

        if (welfareRes.error) {
          // TODO: handle error in notifications
          console.error(welfareRes.error);
          break;
        }

        data.welfare_budget = newValue;
        break;
      case GameValue.BusinessBudget:
        const busRes: any = await invoke("update_business_budget", {
          newBudget: newValue,
        });

        if (busRes.error) {
          // TODO: handle error in notifications
          console.error(busRes.error);
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

    <BudgetCard
      title="Tax Rate"
      appendValueEnd="%"
      currentValue={data.tax_rate}
      data={{
        "Expected Tax Income": data.expected_person_income,
      }}
      assignValueFn={(val) => updateGameValue(GameValue.TaxRate, Number(val))}
    />
    <BudgetCard
      title="Business Tax"
      appendValueEnd="%"
      currentValue={data.business_tax_rate}
      data={{
        "Expected Business Tax Income": data.expected_business_income,
      }}
      assignValueFn={(val) =>
        updateGameValue(GameValue.BusinessTaxRate, Number(val))}
    />

    <h1>BUDGETS</h1>

    <BudgetCard
      title="Healthcare"
      appendValueStart="$"
      currentValue={data.healthcare_budget}
      data={{
        "Used Capacity": data.used_hospital_capacity,
        "Total Capacity": data.total_hospital_capacity,
      }}
      assignValueFn={(val) =>
        updateGameValue(GameValue.HealthcareBudget, Number(val))}
    />
    <BudgetCard
      title="Welfare"
      appendValueStart="$"
      currentValue={data.welfare_budget}
      data={{}}
      assignValueFn={(val) =>
        updateGameValue(GameValue.WelfareBudget, Number(val))}
    />
    <BudgetCard
      title="Business"
      appendValueStart="$"
      currentValue={data.business_budget}
      data={{}}
      assignValueFn={(val) =>
        updateGameValue(GameValue.BusinessBudget, Number(val))}
    />
    <BudgetCard
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
    overflow: scroll;
    position: relative;
  }

  h1 {
    font-size: 25px;
    font-weight: bold;
    margin: 20px 0;
  }
</style>
