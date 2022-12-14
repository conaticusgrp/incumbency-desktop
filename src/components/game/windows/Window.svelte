<script lang="ts">

  import { MIN_WINDOW_HEIGHT, MIN_WINDOW_WIDTH, RESIZE_BAR_SIZE, WINDOW_HEADER_HEIGHT } from "../../../scripts/desktopConstants";

  export let title: string = "?";
  export let iconPath: string | undefined = undefined;
  export let pos: { x: number, y: number } = { x: 0, y: 0 };
  export let size: { width: number, height: number } = { width: 600, height: 400 };

  let thisObj: HTMLElement;
  let dragOffset: { dx: number, dy: number };
  let resizeType: 'w' | 'h' | 'wh';

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
    pos.x = Math.max(Math.min(e.clientX - dragOffset.dx, parent.innerWidth - size.width), 0);
    pos.y = Math.max(Math.min(e.clientY - dragOffset.dy, parent.innerHeight - size.height), 0);
  }

  const handleDragEnd = (e: MouseEvent): void => {
    handleDrag(e);
    document.removeEventListener("mousemove", handleDrag);
    document.removeEventListener("mouseup", handleDragEnd);
  }

  const handleResizeStart = (e: MouseEvent): void => {
    const classList = (e.target as HTMLElement).classList;
    if (classList.contains('width-resize-bar')) {
      resizeType = 'w';
    } else if (classList.contains('height-resize-bar')) {
      resizeType = 'h';
    } else if (classList.contains('width-height-resize-bar')) {
      resizeType = 'wh';
    } else {
      return;
    }

    document.addEventListener("mousemove", handleResize);
    document.addEventListener("mouseup", handleResizeEnd);
  }

  const handleResize = (e: MouseEvent): void => {
    switch (resizeType) {
      case 'w': {
        size.width = Math.max(Math.min(e.clientX - pos.x, parent.innerWidth), MIN_WINDOW_WIDTH);
      }; break;
      
      case 'h': {
        size.height = Math.max(Math.min(e.clientY - pos.y, parent.innerHeight), MIN_WINDOW_HEIGHT);
      }; break;
      
      case 'wh': {
        size.width = Math.max(Math.min(e.clientX - pos.x, parent.innerWidth), MIN_WINDOW_WIDTH);
        size.height = Math.max(Math.min(e.clientY - pos.y, parent.innerHeight), MIN_WINDOW_HEIGHT);
      }; break;

      default: break;
    }
  }

  const handleResizeEnd = (e: MouseEvent): void => {
    handleResize(e);
    document.removeEventListener("mousemove", handleResize);
    document.removeEventListener("mouseup", handleResizeEnd);
  }

</script>

<!-- PARENT COMPONENT -->
<main
  style="
    left: {pos.x}px;
    top: {pos.y}px;
    width: {size.width}px;
    height: {size.height}px;
  "
  bind:this={thisObj}
>
  
  <div
    class="header"
    style="height: {WINDOW_HEADER_HEIGHT}px;"
    on:mousedown={handleDragStart}
  >

    <div>

      <img
        src={iconPath || ""}
        alt="icon"
        title={title}
        style="width: {WINDOW_HEADER_HEIGHT}px; height: {WINDOW_HEADER_HEIGHT}px;"
      />
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

    <div
      class="width-resize-bar"
      style="width: {RESIZE_BAR_SIZE}px; height: calc(100% - {RESIZE_BAR_SIZE}px);"
      on:mousedown={handleResizeStart}
      ></div>
    <div
      class="height-resize-bar"
      style="width: calc(100% - {RESIZE_BAR_SIZE}px); height: {RESIZE_BAR_SIZE}px;"
      on:mousedown={handleResizeStart}
      ></div>
    <div
      class="width-height-resize-bar"
      style="width: {RESIZE_BAR_SIZE}px; height: {RESIZE_BAR_SIZE}px;"
      on:mousedown={handleResizeStart}
      ></div>

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

  img {
    margin-left: 0.5rem;
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

  .width-resize-bar {
    cursor: ew-resize;
    position: absolute;
    right: 0;
    top: 0;
    z-index: 9999;
    /* background-color: white; */
  }

  .height-resize-bar {
    cursor: ns-resize;
    position: absolute;
    left: 0;
    bottom: 0;
    z-index: 9999;
    /* background-color: white; */
  }

  .width-height-resize-bar {
    cursor:nwse-resize;
    position: absolute;
    right: 0;
    bottom: 0;
    z-index: 9999;
    /* background-color: white; */
  }

</style>