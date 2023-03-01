<script lang="ts">
  export let width: string;
  export let height = "34px";

  export let onClick: (toggled: boolean) => any;
  let isToggled = false;
</script>

<button
  class:toggled={isToggled}
  style="--width: {width};--height: {height}"
  on:click={() => {
    isToggled = !isToggled;
    onClick(isToggled)
  }}
>
  <slot />
</button>

<style>
  button {
    border: 1px solid var(--color-accent);
    width: var(--width);
    padding: 0;
    height: var(--height);
  }

  .toggled {
    background-color: var(--color-accent);
    color: var(--color-bg);
    font-weight: bold;
  }

  button:hover {
    background-color: var(--color-accent);
    color: var(--color-bg);
    font-weight: bold;
  }
</style>
