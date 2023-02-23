<script lang="ts" context="module">
  export interface Pos {
    x: number;
    y: number;
  }

  export interface Size {
    width: number;
    height: number;
    maximized?: boolean;
  }

  export interface CriticalWindowData {
    opened: boolean;
    focused: boolean;
    index: number;
  }

  export const defaultCriticalWindowData = (): CriticalWindowData => {
    return { opened: false, focused: false, index: -1 };
  };
</script>

<script lang="ts">
  import { listen } from "@tauri-apps/api/event";
  import { invoke } from "@tauri-apps/api/tauri";
  import { createEventDispatcher } from "svelte";
  import { tick } from "svelte";
  import {
    MIN_WINDOW_HEIGHT,
    MIN_WINDOW_WIDTH,
    RESIZE_BAR_SIZE,
    WINDOW_HEADER_HEIGHT,
  } from "../../../scripts/desktopConstants";
  import {
    APP_UPDATE,
    WINDOW_AQUIRE_FOCUS,
    WINDOW_CLOSE,
    WINDOW_MAXIMIZE,
    WINDOW_MINIMIZE,
    WINDOW_OPENED,
    WINDOW_RESIZE,
    WINDOW_SEND_NOTIFICATION,
  } from "../../../scripts/windowEvent";

  export let title: string = "?";
  export let pos: Pos = { x: 0, y: 0 };
  export let size: Size = {
    width: 600,
    height: 400,
    maximized: false,
  };
  export let windowData: CriticalWindowData = defaultCriticalWindowData();
  let prevWindowData: CriticalWindowData = defaultCriticalWindowData();

  let thisObj: HTMLElement;
  let dragOffset: { dx: number; dy: number };
  let resizeType: { w?: "r" | "l"; h?: "t" | "b" };
  let boundsBeforeMaximizing: {
    x: number;
    y: number;
    width: number;
    height: number;
  };
  let dispatcher = createEventDispatcher();

  $: if (windowData.opened && !prevWindowData.opened) {
    (async () => {
      const d = await invoke("app_open", { appId: windowData.index }).catch(
        (e) => {
          console.error(e);

          dispatcher("criticalWindowEvent", {
            type: WINDOW_SEND_NOTIFICATION,
            data: {
              app: title,
              header: "App open error",
              content: "Error occured while opening the app",
              severity: "error",
            },
          });
        }
      );

      dispatcher("windowEvent", { type: WINDOW_OPENED, data: d });
    })();
  }

  $: {
    (async () => {
      await tick();
      prevWindowData = { ...windowData };
    })();
  }

  listen("update_app", ({ payload }: any) => {
    if (payload.app_id !== windowData.index) return;
    dispatcher("windowEvent", { type: APP_UPDATE, data: payload });
  });

  const getParentBox = (): {
    x: number;
    y: number;
    width: number;
    height: number;
  } => {
    const box = thisObj.parentElement?.getBoundingClientRect();
    if (box == undefined) {
      return {
        x: 0,
        y: 0,
        width: 0,
        height: 0,
      };
    }
    return box;
  };

  const requestFocus = () => {
    dispatcher("criticalWindowEvent", { type: WINDOW_AQUIRE_FOCUS });
  };

  const maximize = (): void => {
    boundsBeforeMaximizing = { ...pos, ...size };
    pos = { x: 0, y: 0 };
    size = {
      width: thisObj.parentElement?.clientWidth ?? 0,
      height: thisObj.parentElement?.clientHeight ?? 0,
      maximized: true,
    };
    dispatcher("windowEvent", { type: WINDOW_MAXIMIZE, status: true });
  };

  const unmaximize = (): void => {
    pos = { x: boundsBeforeMaximizing.x, y: boundsBeforeMaximizing.y };
    size = {
      width: boundsBeforeMaximizing.width,
      height: boundsBeforeMaximizing.height,
      maximized: false,
    };
    dispatcher("windowEvent", { type: WINDOW_MAXIMIZE, status: false });
  };

  const handleClose = (): void => {
    windowData.focused = true;
    windowData = windowData;
    dispatcher("criticalWindowEvent", { type: WINDOW_CLOSE });
    invoke("app_close", { appId: windowData.index });
  };

  const handleMaximize = (): void => {
    if (size.maximized) {
      unmaximize();
    } else {
      maximize();
    }
  };

  const handleMinimize = (): void => {
    dispatcher("criticalWindowEvent", { type: WINDOW_MINIMIZE });
    invoke("app_close", { appId: windowData.index });
  };

  const handleDragStart = (e: MouseEvent): void => {
    if (
      e.target instanceof HTMLImageElement ||
      e.target instanceof HTMLButtonElement
    )
      return;

    document.body.style.cursor = "move";

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
  };

  const handleDrag = (e: MouseEvent): void => {
    pos.x = Math.max(
      Math.min(
        e.clientX - dragOffset.dx,
        getParentBox().width - size.width - 2
      ),
      0
    );
    pos.y = Math.max(
      Math.min(e.clientY - dragOffset.dy, getParentBox().height - size.height),
      0
    );
  };

  const handleDragEnd = (e: MouseEvent): void => {
    document.body.style.cursor = "initial";
    handleDrag(e);
    if (pos.y === 0) {
      maximize();
    }
    document.removeEventListener("mousemove", handleDrag);
    document.removeEventListener("mouseup", handleDragEnd);
  };

  const handleResizeStart = (e: MouseEvent): void => {
    const classList = (e.target as HTMLElement).classList;
    resizeType = {};
    if (classList.contains("resize-bar-right")) {
      resizeType.w = "r";
    } else if (classList.contains("resize-bar-left")) {
      resizeType.w = "l";
    }
    if (classList.contains("resize-bar-bottom")) {
      resizeType.h = "b";
    } else if (classList.contains("resize-bar-top")) {
      resizeType.h = "t";
    }

    document.addEventListener("mousemove", handleResize);
    document.addEventListener("mouseup", handleResizeEnd);
  };

  const handleResize = (e: MouseEvent): void => {
    if (resizeType.w === "r") {
      const untilRightBorder = getParentBox().width - pos.x;
      const x = e.clientX - getParentBox().x - pos.x;
      size.width = Math.max(
        Math.min(x, untilRightBorder - 2),
        MIN_WINDOW_WIDTH.value
      );
    } else if (resizeType.w === "l") {
      const x = e.clientX - getParentBox().x;
      const untilMinWidth = pos.x + size.width - MIN_WINDOW_WIDTH.value;
      const newX = Math.max(Math.min(x, untilMinWidth - 2), 0);
      size.width = size.width + (pos.x - newX);
      pos.x = newX;
    }

    if (resizeType.h === "b") {
      const untilParentHeight = getParentBox().height - pos.y;
      const y = e.clientY - getParentBox().y - pos.y;
      size.height = Math.max(
        Math.min(y, untilParentHeight),
        MIN_WINDOW_HEIGHT.value
      );
    } else if (resizeType.h === "t") {
      const untilMinHeight = pos.y + size.height - MIN_WINDOW_HEIGHT.value;
      const y = e.clientY - getParentBox().y;
      const newY = Math.max(Math.min(y, untilMinHeight), 0);
      size.height = size.height + (pos.y - newY);
      pos.y = newY;
    }

    dispatcher("windowEvent", { type: WINDOW_RESIZE });
  };

  const handleResizeEnd = (e: MouseEvent): void => {
    handleResize(e);
    document.removeEventListener("mousemove", handleResize);
    document.removeEventListener("mouseup", handleResizeEnd);
  };
</script>

<!-- PARENT COMPONENT -->
<main
  style="
    display: {windowData.opened ? 'initial' : 'none'};
    left: {pos.x}px;
    top: {pos.y}px;
    width: {size.width}px;
    height: {size.height}px;
    z-index: {windowData.focused ? 10_000 : 9999};
  "
  on:mousedown={requestFocus}
  bind:this={thisObj}
>
  <div
    class="header"
    style="height: {WINDOW_HEADER_HEIGHT};"
    on:mousedown={handleDragStart}
  >
    <button class="close-button" title="Close" on:click={handleClose}>
      Close
    </button>

    <button class="minimize-button" title="Minimize" on:click={handleMinimize}>
      Minimize
    </button>

    <button class="maximize-button" title="Maximize" on:click={handleMaximize}>
      Maximize
    </button>

    <div>
      {title}
    </div>
  </div>

  <!-- Viewport -->
  <div
    class="viewport"
    style="width: 100%; height: calc(100% - {WINDOW_HEADER_HEIGHT});"
  >
    <slot />

    <div
      class="resize-bar-left"
      style="
        width: {RESIZE_BAR_SIZE};
        height: calc(100% - {RESIZE_BAR_SIZE} * 2);
        top: {RESIZE_BAR_SIZE};
      "
      on:mousedown={handleResizeStart}
    />
    <div
      class="resize-bar-right"
      style="
        width: {RESIZE_BAR_SIZE};
        height: calc(100% - {RESIZE_BAR_SIZE} * 2);
        top: {RESIZE_BAR_SIZE};
      "
      on:mousedown={handleResizeStart}
    />
    <div
      class="resize-bar-top"
      style="
        width: calc(100% - {RESIZE_BAR_SIZE} * 2);
        height: {RESIZE_BAR_SIZE};
        left: {RESIZE_BAR_SIZE};
        "
      on:mousedown={handleResizeStart}
    />
    <div
      class="resize-bar-bottom"
      style="
        width: calc(100% - {RESIZE_BAR_SIZE} * 2);
        height: {RESIZE_BAR_SIZE};
        left: {RESIZE_BAR_SIZE};
      "
      on:mousedown={handleResizeStart}
    />

    <div
      class="resize-bar-top resize-bar-left"
      style="
        width: {RESIZE_BAR_SIZE};
        height: {RESIZE_BAR_SIZE};
      "
      on:mousedown={handleResizeStart}
    />
    <div
      class="resize-bar-bottom resize-bar-right"
      style="
        width: {RESIZE_BAR_SIZE};
        height: {RESIZE_BAR_SIZE};
      "
      on:mousedown={handleResizeStart}
    />
    <div
      class="resize-bar-bottom resize-bar-left"
      style="
        width: {RESIZE_BAR_SIZE};
        height: {RESIZE_BAR_SIZE};
      "
      on:mousedown={handleResizeStart}
    />
    <div
      class="resize-bar-top resize-bar-right"
      style="
        width: {RESIZE_BAR_SIZE};
        height: {RESIZE_BAR_SIZE};
      "
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

  .header > button:hover {
    color: var(--color-bg);
    background-color: var(--color-accent);
    font-weight: bold;
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

  .resize-bar-right {
    cursor: ew-resize;
    position: absolute;
    /* top: 0; */
    right: 0;
    bottom: initial;
    left: initial;
    z-index: 9999;
    /* background-color: white; */
  }

  .resize-bar-left {
    cursor: ew-resize;
    position: absolute;
    /* top: 0; */
    right: initial;
    bottom: initial;
    left: 0;
    z-index: 9999;
    /* background-color: white; */
  }

  .resize-bar-bottom {
    cursor: ns-resize;
    position: absolute;
    top: initial;
    right: initial;
    bottom: 0;
    /* left: 0; */
    z-index: 9999;
    /* background-color: white; */
  }

  .resize-bar-top {
    cursor: ns-resize;
    position: absolute;
    top: 0;
    right: initial;
    bottom: initial;
    /* left: 0; */
    z-index: 9999;
    /* background-color: white; */
  }

  .resize-bar-bottom.resize-bar-right {
    cursor: nwse-resize;
    position: absolute;
    top: initial;
    right: 0;
    bottom: 0;
    left: initial;
    z-index: 9999;
    /* background-color: white; */
  }

  .resize-bar-top.resize-bar-left {
    cursor: nwse-resize;
    position: absolute;
    top: 0;
    right: initial;
    bottom: initial;
    left: 0;
    z-index: 9999;
    /* background-color: white; */
  }

  .resize-bar-top.resize-bar-right {
    cursor: nesw-resize;
    position: absolute;
    top: 0;
    right: 0;
    bottom: initial;
    left: initial;
    z-index: 9999;
    /* background-color: white; */
  }

  .resize-bar-bottom.resize-bar-left {
    cursor: nesw-resize;
    position: absolute;
    top: initial;
    right: initial;
    bottom: 0;
    left: 0;
    z-index: 9999;
    /* background-color: white; */
  }
</style>
