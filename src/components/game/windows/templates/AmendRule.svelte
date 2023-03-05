<script lang="ts">
  import { onMount } from "svelte";
  import { RuleCardType, type RuleCardValue } from "./RuleCard.svelte";


  export let updateValues: RuleCardValue[];
  export let shown = false;
  export let type: RuleCardType;
  export let updateRuleFn: (data: any[]) => void;
  // export let appendValueStart: string; // TODO: use these
  // export let appendValueEnd: string;

  let values: RuleCardValue[] = [];

  onMount(() => {
    values = updateValues;
  })

  let currentPage = 1;
</script>

{#if values.length > 0 && shown && type === RuleCardType.IfThen}
  <div class="bg" />
  <div class="container">
    {#if currentPage === 1}
      <div class="title-container">
        <p>If {values[0].str} {values[0].value}</p>
      </div>
      <div class="input-container">
        <input type="text" bind:value={values[0].value} />
      </div>
      <div class="btns">
        <button
          on:click={() => {
            currentPage = 2;
          }}
          class="left-btn">Continue</button
        >
        <button on:click={() => (shown = false)} class="right-btn">Cancel</button>
      </div>
    {/if}

    {#if currentPage === 2}
        <div class="title-container">
            <p>Then {values[1].str} {values[1].value}</p>
        </div>
        <div class="input-container">
            <input type="text" bind:value={values[1].value} />
        </div>
        <div class="btns">
            <button
            on:click={() => {
                updateRuleFn([values[0].value, values[1].value]);
                shown = false;
            }}
            class="left-btn">Confirm</button
            >
            <button on:click={() => (currentPage = 1)} class="right-btn">Go back</button>
        </div>
    {/if}
  </div>
{/if}

<style>
  .bg {
    position: absolute;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    background-color: black;
    opacity: 0.8;
  }

  .container {
    position: fixed;
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);
    width: 500px;
    height: 180px;
    border-radius: 10px;
    background-color: black;
    border: solid 1px var(--color-accent);
    z-index: 99999;
  }

  .title-container {
    width: 100%;
    height: 60px;
    background-color: var(--color-accent);
    color: black;
    border-radius: 10px 10px 0 0;
    font-weight: bold;
    display: flex;
    justify-content: center;
    align-items: center;
  }

  .title-container p {
    font-size: 20px;
  }

  .input-container {
    position: relative;
  }

  .input-container p {
    position: absolute;
    font-size: 40px;
    font-weight: bold;
    color: grey;
  }

  input {
    background: none;
    border: none;
    color: white;
    font-size: 40px;
    font-weight: bold;
    outline: none;
    width: 80%;
    text-align: center;
    margin-top: 10px;
    border-bottom: solid 1px #3b530b;
  }

  input:focus {
    border-bottom: solid 1px var(--color-accent);
  }

  div .btns {
    position: absolute;
    bottom: 0;
    width: 100%;
    display: flex;
  }

  div .btns button {
    width: 100%;
    border: solid 1px var(--color-accent);
    color: grey;
  }

  div .btns button:hover {
    color: white;
    font-weight: bold;
  }

  .left-btn {
    border-bottom-left-radius: 10px;
  }

  .right-btn {
    border-bottom-right-radius: 10px;
  }
</style>
