<script lang="ts">

  import Desktop from "../game/desktop/Desktop.svelte"

  let header: HTMLElement;
  $: headerHeight = header?.clientHeight;

</script>

<main>

  <h2 bind:this={header}>Singleplayer</h2>
  <!-- Viewport -->
  <div style="width: 100%; height: calc(100% - {headerHeight}px);">

    <Desktop />

  </div>

</main>

<style>

  main {
    position: relative;
    width: 100%;
    height: 100%;
  }

  div {
    position: relative;
  }

</style>
