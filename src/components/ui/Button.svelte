<script lang="ts">
  export let width: string;
  export let onClick: () => any;
  export let height = "34px";
  export let style = "";
  export let enabled: boolean = true;
</script>

<button disabled={!enabled} on:click={onClick} style="--width: {width}; --height: {height};{style}">
  <slot />
</button>

<style>
  button {
    border: 1px solid var(--color-accent);
    width: var(--width);
    padding: 0;
    height: var(--height);
  }

  button:hover:enabled {
    background-color: var(--color-accent);
    color: var(--color-bg);
    font-weight: bold;
  }
</style>
