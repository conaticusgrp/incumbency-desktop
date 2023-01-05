<script lang="ts">
  
  import { createEventDispatcher } from "svelte";

  export let name = "?";
  export let icon: string | undefined;
  export let badgeCount = 0;
  export let gridRow: string;
  export let index: number;
  
  let dispatcher = createEventDispatcher();

  const open = (): void => {
    dispatcher('openApp', index);
  }

</script>

<main style="gridRow: {gridRow};" on:mouseup={open}>

  <div class="icon">

    {#if icon != undefined}
      <img src={icon} alt={name + " app icon"} />
    {/if}

    {#if badgeCount > 0}
      <div class="badge">{badgeCount}</div>
    {/if}

  </div>

  <div>{name}</div>

</main>

<style>

  main {
    /* DEBUG: magic number */
    width: 80px;

    display: flex;
    flex-direction: column;
    align-items: center;
    /* justify-content: center; */
  }

  .icon {
    width: 70px;
    height: 70px;
    background: #3d3d3d;
    background-size: cover;
    position: relative;
  }

  .icon img {
    width: 100%;
    height: 100%;
    object-fit: cover;
  }

  .badge {
    position: absolute;
    /* DEBUG: magic numbers */
    width: 20px;
    height: 20px;
    top: calc(100% - 10px);
    left: calc(100% - 10px);
    z-index: 1;
    border-radius: 50%;
    background: #c2403a;
  }

</style>
