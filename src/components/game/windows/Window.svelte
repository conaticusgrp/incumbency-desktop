<script lang="ts">

  import { WINDOW_HEADER_HEIGHT } from "../../../scripts/desktopConstants";

  export let title: string = "?";
  export let iconPath: string | undefined = undefined;
  export let pos: { x: number, y: number } = { x: 0, y: 0 };
  export let size: { width: number, height: number } = { width: 600, height: 400 };

  let thisObj: HTMLElement;
  let dragOffset: { dx: number, dy: number };

  const handleClose = (): void => {
    console.log("close");
  }

  const handleMaximize = (): void => {
    console.log("maximize");
  }

  const handleMinimize = (): void => {
    console.log("minimize");
  }

  const handleDragStart = (e: MouseEvent): void => {
    if (e.target instanceof HTMLImageElement ||
        e.target instanceof HTMLButtonElement) return;
    
    document.addEventListener("mousemove", handleDrag);
    document.addEventListener("mouseup", handleDragEnd);

    dragOffset = { dx: e.clientX - thisObj.offsetLeft, dy: e.clientY - thisObj.offsetTop };
  }

  const handleDrag = (e: MouseEvent): void => {
    pos.x = e.clientX - dragOffset.dx;
    pos.y = e.clientY - dragOffset.dy;
  }

  const handleDragEnd = (e: MouseEvent): void => {
    handleDrag(e);
    document.removeEventListener("mousemove", handleDrag);
    document.removeEventListener("mouseup", handleDragEnd);
  }

</script>

<!-- PARENT COMPONENT -->

<!-- TODO: add min and max values for the position and the size -->
<main bind:this={thisObj} style="left: {pos.x}px; top: {pos.y}px; width: {size.width}px; height: {size.height}px">
  
  <div
    class="header"
    style="height: {WINDOW_HEADER_HEIGHT}px;"
    on:mousedown={handleDragStart}
  >

    <div>

      <img src={iconPath || ""} alt="icon" title={title} />
      <span>{title}</span>
      
    </div>
    <div>

      <!-- Please fix the buttons, anyone -->
      <button
        class="close-button"
        style="
          width: {WINDOW_HEADER_HEIGHT}px;
          height: {WINDOW_HEADER_HEIGHT}px;
          border-radius: {WINDOW_HEADER_HEIGHT}px;
        "
        title="Close"
        on:click={handleClose}
      ></button>

      <button
        class="maximize-button"
        style="
          width: {WINDOW_HEADER_HEIGHT}px;
          height: {WINDOW_HEADER_HEIGHT}px;
          border-radius: {WINDOW_HEADER_HEIGHT}px;
        "
        title="Maximize"
        on:click={handleMaximize}
      ></button>

      <button
        class="minimize-button"
        style="
          width: {WINDOW_HEADER_HEIGHT}px;
          height: {WINDOW_HEADER_HEIGHT}px;
          border-radius: {WINDOW_HEADER_HEIGHT}px;
        "
        title="Minimize"
        on:click={handleMinimize}
      ></button>
      
    </div>
  </div>

  <!-- Viewport -->
  <div class="viewport" style="width: 100%; height: calc(100% - {WINDOW_HEADER_HEIGHT}px);">

    <slot />

  </div>
  
</main>

<style>

  main {
    position: absolute;
    border: 1px solid grey;

    pointer-events: all;
  }

  .header {
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

  button {
    padding: 0;
    margin: 0 0.5rem 0 0.5rem;
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