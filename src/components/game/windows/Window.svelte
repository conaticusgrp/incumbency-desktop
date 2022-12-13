<script lang="ts">

  export let title: string = "?";
  export let iconPath: string | undefined = undefined;
  export let pos: { x: number, y: number } = { x: 0, y: 0 };
  export let size: { width: number, height: number } = { width: 600, height: 400 };

  let thisObj: HTMLElement;
  let header: HTMLElement;

  $: headerHeight = header?.clientHeight;

  const handleClose = (): void => {

  }

  const handleMaximize = (): void => {
    
  }

  const handleMinimize = (): void => {
    
  }

</script>

<!-- PARENT COMPONENT -->

<!-- TODO: add min and max values for the position and the size -->
<main bind:this={thisObj} style="left: {pos.x}px; top: {pos.y}px; width: {size.width}px; height: {size.height}px">
  
  <div class="header" bind:this={header}>
    <div>

      <img src={iconPath || ""} alt="icon" title={title} />
      <span>{title}</span>
      
    </div>
    <div>

      <!-- Please fix the buttons, anyone -->
      <button
        class="close-button"
        style="width: {headerHeight}px; height: {headerHeight}px; border-radius: {headerHeight}px;"
        title="Close"
        on:click={handleClose}
      ></button>

      <button
        class="maximize-button"
        style="width: {headerHeight}px; height: {headerHeight}px; border-radius: {headerHeight}px;"
        title="Maximize"
        on:click={handleMaximize}
      ></button>

      <button
        class="minimize-button"
        style="width: {headerHeight}px; height: {headerHeight}px; border-radius: {headerHeight}px;"
        title="Minimize"
        on:click={handleMinimize}
      ></button>
      
    </div>
  </div>

  <!-- Viewport -->
  <div class="viewport" style="width: 100%; height: calc(100% - {headerHeight}px);">

    <slot />

  </div>
  
</main>

<style>

  main {
    position: absolute;
    border: 1px solid grey;
  }

  .header {
    /* DEBUG: magic number */
    height: 20px;
    display: grid;
    grid-template-rows: 1fr;
    grid-template-columns: 1fr 1fr;
    align-items: center;
    background-color: grey;
  }

  .header :first-child {
    display: flex;
    flex-direction: row;
  }

  .header :last-child {
    display: flex;
    flex-direction: row-reverse;
  }

  span {
    margin-left: 1rem;
  }

  .viewport {
    background-color: #121212;
    isolation: isolate;
  }

  .close-button {
    background-color: red;
  }

  .maximize-button {
    background-color: yellow;
  }

  .minimize-button {
    background-color: green;
  }


</style>