<script lang="ts">
  
  import { createEventDispatcher, onMount } from "svelte"

  import {
    MIN_WINDOW_HEIGHT,
    MIN_WINDOW_WIDTH,
    RESIZE_BAR_SIZE,
    WINDOW_HEADER_HEIGHT,
  } from "../../../scripts/desktopConstants"

  export let title: string = "?";
  export let pos: { x: number; y: number } = { x: 0, y: 0 };
  export let size: { width: number; height: number, maximized?: boolean } = {
    width: 600,
    height: 400,
    maximized: false
  };
  
  let thisObj: HTMLElement;
  let dragOffset: { dx: number; dy: number };
  let resizeType: { w?: 'r' | 'l', h?: 't' | 'b' };
  let boundsBeforeMaximizing: { x: number, y: number, width: number, height: number };
  
  let observer: MutationObserver;
  let opened = false;
  let focused = false;
  let dispatcher = createEventDispatcher();

  const requestFocus = (): void => {
    if (thisObj.parentElement == undefined) return;
    // NOTE: The next line breaks the focus system. Do not add it to the code
    // if (focused) return;

    const others = thisObj.parentElement.children;
    for (let i = 0; i < others.length; i++) {
      (others[i] as HTMLElement).dataset['focused'] = String(others[i] == thisObj);
    }
  }

  const maximize = (): void => {
    boundsBeforeMaximizing = { ...pos, ...size }
    pos = { x: 0, y: 0 }
    size = { width: thisObj.parentElement?.clientWidth ?? 0, height: thisObj.parentElement?.clientHeight ?? 0, maximized: true }
  }

  const unmaximize = (): void => {
    pos = { x: boundsBeforeMaximizing.x, y: boundsBeforeMaximizing.y }
    size = { width: boundsBeforeMaximizing.width, height: boundsBeforeMaximizing.height, maximized: false }
  }

  const handleClose = (): void => {
    thisObj.style.display = 'none';
    thisObj.dataset['focused'] = 'false';
    dispatcher('criticalWindowEvent', { type: 'windowClose' });
  }

  const handleMaximize = (): void => {
    if (size.maximized) {
      unmaximize();
    } else {
      maximize();
    }
  }

  const handleMinimize = (): void => {
    thisObj.dataset['minimized'] = 'true';
  }

  const handleDragStart = (e: MouseEvent): void => {
    if (
      e.target instanceof HTMLImageElement ||
      e.target instanceof HTMLButtonElement
    )
      return;

    if (size.maximized) {
      // cursorPos(max)/width(max) = cursorPos(min)/width(min)
      const cursorWindowPercentageXMax = e.clientX / size.width;
      unmaximize();
      const cursorOffsetMin = cursorWindowPercentageXMax * size.width;
      pos.x = e.clientX - cursorOffsetMin;
      pos.y = 0;
    }

    document.addEventListener("mousemove", handleDrag);
    document.addEventListener("mouseup", handleDragEnd);

    dragOffset = {
      dx: e.clientX - pos.x,
      dy: e.clientY - pos.y,
    };
  }

  const handleDrag = (e: MouseEvent): void => {
    pos.x = Math.max(
      Math.min(e.clientX - dragOffset.dx, (thisObj.parentElement?.clientWidth ?? 0) - size.width),
      0
    );
    pos.y = Math.max(
      Math.min(e.clientY - dragOffset.dy, (thisObj.parentElement?.clientHeight ?? 0) - size.height),
      0
    );
  }

  const handleDragEnd = (e: MouseEvent): void => {
    handleDrag(e);
    document.removeEventListener("mousemove", handleDrag);
    document.removeEventListener("mouseup", handleDragEnd);
  }

  const handleResizeStart = (e: MouseEvent): void => {
    const classList = (e.target as HTMLElement).classList
    if (classList.contains("width-resize-bar-right")) {
      resizeType = { w: 'r' };
    } else if (classList.contains("width-resize-bar-left")) {
      resizeType = { w: 'l' };
    } else if (classList.contains("height-resize-bar-bottom")) {
      resizeType = { h: 'b' };
    } else if (classList.contains("height-resize-bar-top")) {
      resizeType = { h: 't' };
    } else if (classList.contains("width-height-resize-bar-bottom-right")) {
      resizeType = { w: 'r', h: 'b' };
    } else if (classList.contains("width-height-resize-bar-top-left")) {
      resizeType = { w: 'l', h: 't' };
    } else if (classList.contains("width-height-resize-bar-top-right")) {
      resizeType = { w: 'r', h: 't' };
    } else if (classList.contains("width-height-resize-bar-bottom-left")) {
      resizeType = { w: 'l', h: 'b'};
    } else {
      return;
    }

    document.addEventListener("mousemove", handleResize);
    document.addEventListener("mouseup", handleResizeEnd);
  }

  const handleResize = (e: MouseEvent): void => {
    if (resizeType.w === 'r') {
      size.width = Math.max(
        Math.min(e.clientX - pos.x, (thisObj.parentElement?.clientWidth ?? 0) - pos.x),
        MIN_WINDOW_WIDTH
      );
    } else if (resizeType.w === 'l') {
      const newX = Math.max(Math.min(e.clientX, pos.x + size.width - MIN_WINDOW_WIDTH), 0);
      size.width = size.width + (pos.x - newX);
      pos.x = newX;
    }

    if (resizeType.h === 'b') {
      size.height = Math.max(
        Math.min(e.clientY - pos.y, (thisObj.parentElement?.clientHeight ?? 0) - pos.y),
        MIN_WINDOW_HEIGHT
      );
    } else if (resizeType.h === 't') {
      const newY = Math.max(Math.min(e.clientY, pos.y + size.height - MIN_WINDOW_HEIGHT), 0);
      size.height = size.height + (pos.y - newY);
      pos.y = newY;
    }
    
  }

  const handleResizeEnd = (e: MouseEvent): void => {
    handleResize(e);
    document.removeEventListener("mousemove", handleResize);
    document.removeEventListener("mouseup", handleResizeEnd);
  }

  onMount(() => {
    observer = new MutationObserver((mutations) => {
      mutations.forEach((mutation) => {
        if (mutation.type === "attributes") {
          if (mutation.attributeName === "style") {
            opened = thisObj?.style.display != 'none';
          } else if (mutation.attributeName === "data-focused") {
            focused = thisObj?.dataset['focused'] as unknown as boolean;
          } else if (mutation.attributeName === "data-minimized") {
            thisObj.style.display = (thisObj?.dataset['minimized'] !== 'true') ? 'initial' : 'none';
            dispatcher('criticalWindowEvent', { type: 'windowMinimizeStateChange' });
          }
        }
      });
    });
    thisObj.style.display = 'none';

    observer.observe(thisObj, {
      attributes: true
    });
  });
  
</script>

<!-- PARENT COMPONENT -->
<main
  style="
    left: {pos.x}px;
    top: {pos.y}px;
    width: {size.width}px;
    height: {size.height}px;
    z-index: {focused ? 10_000 : 9999};
  "
  data-focused={false}
  data-minimized={false}
  on:mousedown={requestFocus}
  bind:this={thisObj}
>

  <div
    class="header"
    style="height: {WINDOW_HEADER_HEIGHT}px;"
    on:mousedown={handleDragStart}
  >

    <button class="close-button" title="Close" on:click={handleClose}>
      Close
    </button>

    <button
      class="minimize-button"
      title="Minimize"
      on:click={handleMinimize}
    >
      Minimize
    </button>

    <button
      class="maximize-button"
      title="Maximize"
      on:click={handleMaximize}
    >
      Maximize
    </button>

    <div>
      {title}
    </div>

  </div>

  <!-- Viewport -->
  <div
    class="viewport"
    style="width: 100%; height: calc(100% - {WINDOW_HEADER_HEIGHT}px);"
  >

    <slot />

    <div
      class="width-resize-bar-left"
      style="width: {RESIZE_BAR_SIZE}px; height: calc(100% - {RESIZE_BAR_SIZE}px * 2);"
      on:mousedown={handleResizeStart}
    />
    <div
      class="width-resize-bar-right"
      style="width: {RESIZE_BAR_SIZE}px; height: calc(100% - {RESIZE_BAR_SIZE}px);"
      on:mousedown={handleResizeStart}
    />
    <div
      class="height-resize-bar-top"
      style="width: calc(100% - {RESIZE_BAR_SIZE}px * 2); height: {RESIZE_BAR_SIZE}px;"
      on:mousedown={handleResizeStart}
    />
    <div
      class="height-resize-bar-bottom"
      style="width: calc(100% - {RESIZE_BAR_SIZE}px); height: {RESIZE_BAR_SIZE}px;"
      on:mousedown={handleResizeStart}
    />
      
    <div
      class="width-height-resize-bar-top-left"
      style="width: {RESIZE_BAR_SIZE}px; height: {RESIZE_BAR_SIZE}px;"
      on:mousedown={handleResizeStart}
    />
    <div
      class="width-height-resize-bar-bottom-right"
      style="width: {RESIZE_BAR_SIZE}px; height: {RESIZE_BAR_SIZE}px;"
      on:mousedown={handleResizeStart}
    />
    <div
      class="width-height-resize-bar-bottom-left"
      style="width: {RESIZE_BAR_SIZE}px; height: {RESIZE_BAR_SIZE}px;"
      on:mousedown={handleResizeStart}
    />
    <div
      class="width-height-resize-bar-top-right"
      style="width: {RESIZE_BAR_SIZE}px; height: {RESIZE_BAR_SIZE}px;"
      on:mousedown={handleResizeStart}
    />

  </div>

</main>

<style>

  main {
    position: absolute;
    border: 1px solid var(--color-accent);
    border-top: none;
    pointer-events: all;
    background-color: var(--color-bg);
  }
  
  .header {
    display: flex;
    flex-direction: row;
    align-items: stretch;
    border-top: 1px solid var(--color-accent);
    border-bottom: 1px solid var(--color-accent);
  }

  .header > button {
    padding: 0 1em 0 1em;
    border-right: 1px solid var(--color-accent);
  }

  .header > div {
    margin: auto;
  }

  .viewport {
    background-color: var(--color-bg);
    border-bottom: 1px solid var(--color-accent);
    isolation: isolate;
  }

  /* Resize bars */

  .width-resize-bar-right {
    cursor: ew-resize;
    position: absolute;
    right: 0;
    top: 0;
    z-index: 9999;
    /* background-color: white; */
  }

  .width-resize-bar-left {
    cursor: ew-resize;
    position: absolute;
    left: 0;
    top: 0;
    z-index: 9999;
    /* background-color: white; */
  }

  .height-resize-bar-bottom {
    cursor: ns-resize;
    position: absolute;
    left: 0;
    bottom: 0;
    z-index: 9999;
    /* background-color: white; */
  }

  .height-resize-bar-top {
    cursor: ns-resize;
    position: absolute;
    left: 0;
    top: 0;
    z-index: 9999;
    /* background-color: white; */
  }

  .width-height-resize-bar-bottom-right {
    cursor: nwse-resize;
    position: absolute;
    right: 0;
    bottom: 0;
    z-index: 9999;
    /* background-color: white; */
  }

  .width-height-resize-bar-top-left {
    cursor: nwse-resize;
    position: absolute;
    left: 0;
    top: 0;
    z-index: 9999;
    /* background-color: white; */
  }

  .width-height-resize-bar-top-right {
    cursor: nesw-resize;
    position: absolute;
    right: 0;
    top: 0;
    z-index: 9999;
    /* background-color: white; */
  }

  .width-height-resize-bar-bottom-left {
    cursor: nesw-resize;
    position: absolute;
    left: 0;
    bottom: 0;
    z-index: 9999;
    /* background-color: white; */
  }

</style>
