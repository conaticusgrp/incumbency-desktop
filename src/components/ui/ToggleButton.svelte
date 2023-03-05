<script lang="ts">
  export let width: string;
  export let height = "34px";

  export let onClick: (toggled: boolean) => any;
  export let isToggled = false;
  export let activeText: string;
  export let inactiveText: string;
  export let style: string = "";
</script>

<button
  class:toggled={isToggled}
  style="--width: {width};--height: {height};{style}"
  on:click={() => {
    isToggled = !isToggled;
    onClick(isToggled)
  }}
>
  {isToggled ? activeText : inactiveText}
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
