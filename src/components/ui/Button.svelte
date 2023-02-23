<script lang="ts">
  export let width: string;
  export let onClick: () => any;
</script>

<button on:click={onClick} style="--width: {width}">
  <slot />
</button>

<style>
  button {
    border: 1px solid var(--color-accent);
    width: var(--width);
    padding: 0;
    height: 34px;
  }

  button:hover {
    background-color: var(--color-accent);
    color: var(--color-bg);
    font-weight: bold;
  }
</style>
