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
    import App from "../../../App.svelte";
    import {
        MIN_WINDOW_HEIGHT,
        MIN_WINDOW_WIDTH,
        RESIZE_BAR_SIZE,
        USERNAME,
        WINDOW_HEADER_HEIGHT,
    } from "../../../scripts/desktopConstants";
    67;
    import {
        APP_UPDATE,
        EMAIL_CREATE,
        WINDOW_AQUIRE_FOCUS,
        WINDOW_CLOSE,
        WINDOW_MAXIMIZE,
        WINDOW_MINIMIZE,
        WINDOW_OPENED,
        WINDOW_RESIZE,
        WINDOW_SEND_NOTIFICATION,
    } from "../../../scripts/windowEvent";
    import type { FinanceData } from "./Finance/Finance.svelte";

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
    let transition = "";

    $: if (windowData.opened && !prevWindowData.opened) {
        (async () => {
            const d = await invoke("app_open", {
                appId: windowData.index,
            }).catch((e) => {
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
            });

            dispatcher("windowEvent", { type: WINDOW_OPENED, data: d });
        })();
    }

    $: {
        (async () => {
            await tick();
            prevWindowData = { ...windowData };
        })();
    }

    enum Apps {
        Finance = 1,
        Healthcare = 2,
        Welfare = 3,
        Business = 4,
    }

    type AppString = "finance" | "healthcare" | "welfare" | "business";

    const getAppNameFromId = (id: number): AppString => {
        return Apps[id].toLowerCase() as any;
    };

    listen("update_app", ({ payload }: any) => {
        if (payload.app_id === windowData.index) {
            dispatcher("windowEvent", { type: APP_UPDATE, data: payload });
        }

        const app = getAppNameFromId(payload.app_id);
        const { data, update_type } = payload;

        if (update_type !== "month") return;

        if (app == "finance") {
            const appData: FinanceData = data;
            const totalBudgetSpending =
                appData.welfare_budget +
                appData.business_budget +
                appData.healthcare_budget;

            const enoughToSpend =
                appData.expected_balance - totalBudgetSpending > 0;

            const enoughSaved =
                appData.expected_balance - totalBudgetSpending * 0.3 > 0;

            if (!enoughToSpend) {
                dispatcher("windowEvent", {
                    type: EMAIL_CREATE,
                    data: {
                        title: "EXPECTED CRISIS",
                        content: `
                Thomas! I have done some calculations and based on our statistics I estimate that we are going to experience a financial crash next month and our budget is going to fall to -$${-appData.expected_balance}. You need to cut at least $${
                            -appData.expected_balance + totalBudgetSpending
                        } off our budgets to gain a digit above $0.

                To gain a safe budget with 30% spare you need to cut at least $${
                    -appData.expected_balance +
                    totalBudgetSpending +
                    totalBudgetSpending * 0.3
                } off our budgets.

                If action isn't taken, the damage could be irreversible,
                Tarun.
                                        `,
                        sender: "Tarun",
                        severity: "error",
                    },
                });
            } else if (!enoughSaved) {
                dispatcher("windowEvent", {
                    type: EMAIL_CREATE,
                    data: {
                        title: "Expected Balance Below Safe Zone",
                        content: `
Hello Thomas,

I have been doing some digging and I wanted to give you a quick heads up. The expected balance next month is $${
                            appData.expected_balance
                        }.

It is generally good practise to keep at least 30% of our expected budget spending for some leeyway incase of miscalculation.
Our estimated budget spending next month is $${totalBudgetSpending}. This means that you need to cut at least $${
                            totalBudgetSpending * 0.3 + totalBudgetSpending
                        } from our budgets spending to maintain a safe budget.

Thanks for reading and good luck,
Tarun.
                        `,
                        sender: "Tarun",
                        severity: "warning",
                    },
                });
            }
        }
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

    const startTransition = (duration: number, ease: boolean = true) => {
        transition = `${duration}s ${ease ? "ease" : ""}`;

        setTimeout(() => {
            transition = "";
        }, duration * 1000);
    };

    const maximize = (): void => {
        startTransition(0.2);
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
        startTransition(0.2);
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
            Math.min(
                e.clientY - dragOffset.dy,
                getParentBox().height - size.height
            ),
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
            const untilMinHeight =
                pos.y + size.height - MIN_WINDOW_HEIGHT.value;
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

    const unlisten = listen("game_generated", () => {
        setTimeout(() => {
            dispatcher("windowEvent", {
                type: EMAIL_CREATE,
                data: {
                    title: "The Start of Your Incumbency",
                    content: `
Hello, ${USERNAME}. You are now in leading position for the country's economy. Gary set the bar high during his incumbency, however, his sex scandal led him to step down from office. My name is Ned, I will be here to advise you about your user interface and and updates it may receive.

What you are currently looking at is the desktop environment that you will use to manage the economy. From here you will control everything - taxes, laws & regulations, healthcare and more. Take a look at your left panel and you will see all the available desktop apps that are at your disposal. You can open them and take a look around. Just.. try not to break everything, okay?

Kind regards,
Ned
`,
                    sender: "Ned",
                },
            });
        }, 5000);
    });
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
    transition: {transition}
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

    main {
        animation-name: popup;
        animation-delay: 0s;
        animation-duration: 0.2s;
        animation-fill-mode: forwards;
        animation-timing-function: ease;
    }

    @keyframes popup {
        from {
            scale: 0;
        }
        to {
            scale: 1;
        }
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
