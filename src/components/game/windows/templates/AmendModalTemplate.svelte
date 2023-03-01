<script lang="ts">
  export let title: string;
  export let value: any;
  export let shown = false;
  export let updateValueFn: (newValue: any) => any;
  export let appendValueStart: string; // TODO: use these
  export let appendValueEnd: string;
</script>

{#if shown}
  <div class="bg" />
  <div class="container">
    <div class="title-container">
      <p>{title}</p>
    </div>
    <div class="input-container">
      <input type="text" bind:value />
    </div>
    <div class="btns">
      <button
        on:click={() => {
          updateValueFn(value);
          shown = false;
        }}
        class="left-btn">Confirm</button
      >
      <button on:click={() => (shown = false)} class="right-btn">Cancel</button>
    </div>
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
