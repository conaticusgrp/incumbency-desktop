<script setup lang="ts">
import { listen } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/tauri";
import {
  MIN_WINDOW_HEIGHT,
  MIN_WINDOW_WIDTH,
  RESIZE_BAR_SIZE,
  USERNAME,
  WINDOW_HEADER_HEIGHT,
} from "../../constants";
import { PropType, computed, ref, nextTick } from "vue";

enum Apps {
  Finance = 1,
  Healthcare = 2,
  Welfare = 3,
  Business = 4,
}

type AppString = "finance" | "healthcare" | "welfare" | "business";

type WindowOpened = {
  window: string;
  data: OpenEvents;
}

const props = defineProps({
  title: {
    type: String,
    default: "?",
  },
  pos: {
    type: Object as PropType<Pos>,
    default: () => ({ x: 0, y: 0 }),
  },
  size: {
    type: Object as PropType<Size>,
    default: () => ({ width: 0, height: 0, maximized: false }),
  },
});

const defaultCriticalWindowData = () => ({ opened: false, focused: false, index: -1 });
const pos = ref<Pos>(props.pos);
const size = ref<Size>(props.size);
const windowData = ref<CriticalWindowData>(defaultCriticalWindowData());
const thisObj = ref<HTMLElement | null>(null);
const dragOffset = ref({ dx: 0, dy: 0 });
const resizeType = ref<{
  w?: "r" | "l", h?: "t" | "b"
}>();
const boundsBeforeMaximizing = ref({ x: 0, y: 0, width: 0, height: 0, });
const emits = defineEmits<{
  (e: 'appUpdate', data: UpdateAppPayloads): void;
  (e: 'windowSendNotification', data: NotificationData): void;
  (e: 'emailCreate', data: EmailData): void;
  (e: 'windowOpened', data: OpenEvents): void;
  (e: 'windowMaximize', status: boolean): void;
  (e: 'windowMinimize', status: boolean): void;
  (e: 'windowResize'): void;
  (e: 'windowClose'): void;
  (e: 'windowAquireFocus'): void;
}>();
const transition = ref("");

nextTick(async () => {
  if (!windowData.value.opened || windowData.value.focused) return;
  try {
    const d = await invoke<OpenEvents>("app_open", {
      appId: windowData.value.index,
    });
    emits("windowOpened", d);
  } catch (e) {
    console.error(e);

    emits('windowSendNotification', {
      app: props.title,
      header: "App open error",
      content: "Error occured while opening the app",
      severity: "error",
    });
  }
})

const getAppNameFromId = (id: number): AppString => {
  return Apps[id].toLowerCase() as any;
};

listen<UpdateAppEventTypes>("update_app", ({ payload }) => {
  if (payload.app_id === windowData.value.index) {
    emits("appUpdate", payload.data);
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
      emits("emailCreate", {
        title: "EXPECTED CRISIS",
        content: `
${USERNAME}! I have done some calculations and based on our statistics I estimate that we are going to experience a financial crash next month and our budget is going to fall to -$${-appData.expected_balance}. You need to save at least $${-appData.expected_balance + totalBudgetSpending} to gain a digit above $0.

To gain a safe budget with 30% spare you need to save at least $${-appData.expected_balance + totalBudgetSpending + totalBudgetSpending * 0.3}.

You can save by either increasing tax rates or reducing budget allocations.

If action isn't taken, the damage could be irreversible,
Tarun.`,
        sender: "Tarun",
        severity: "error",
      });
    } else if (!enoughSaved) {
      emits("emailCreate", {
        title: "Expected Balance Below Safe Zone",
        content: `
Hello ${USERNAME},

I have been doing some digging and I wanted to give you a quick heads up. The expected balance next month is $${appData.expected_balance}.

It is generally good practise to keep at least 30% of our expected budget spending for some leeyway incase of miscalculation.
Our estimated budget spending next month is $${totalBudgetSpending}. This means that you need to save at least $${totalBudgetSpending * 0.3 + totalBudgetSpending} to maintain a safe budget.

You can save by either increasing tax rates or reducing budget allocations.

Thanks for reading and good luck,
Tarun.
                        `,
        sender: "Tarun",
        severity: "warning",
      });
    }
  }
});

const parentBox = computed<{
  x: number;
  y: number;
  width: number;
  height: number;
}>(() => {
  const box = thisObj.value?.parentElement?.getBoundingClientRect();
  if (box == undefined) {
    return {
      x: 0,
      y: 0,
      width: 0,
      height: 0,
    };
  }
  return box;
});

const requestFocus = () => emits('windowAquireFocus');

const startTransition = (duration: number, ease: boolean = true) => {
  transition.value = `${duration}s ${ease ? "ease" : ""}`;

  setTimeout(() => {
    transition.value = "";
  }, duration * 1000);
};

const maximize = (): void => {
  startTransition(0.2);
  boundsBeforeMaximizing.value = { ...pos.value, ...size.value };
  pos.value = { x: 0, y: 0 };
  size.value = {
    width: thisObj.value?.parentElement?.clientWidth ?? 0,
    height: thisObj.value?.parentElement?.clientHeight ?? 0,
    maximized: true,
  };
  emits('windowMaximize', true);
};

const unmaximize = (): void => {
  startTransition(0.2);
  pos.value = { x: boundsBeforeMaximizing.value.x, y: boundsBeforeMaximizing.value.y };
  size.value = {
    width: boundsBeforeMaximizing.value.width,
    height: boundsBeforeMaximizing.value.height,
    maximized: false,
  };
  emits('windowMaximize', false);
};

const handleClose = (): void => {
  windowData.value.focused = true;
  emits('windowClose');
  invoke("app_close", { appId: windowData.value.index });
};

const handleMaximize = (): void => {
  if (size.value.maximized) {
    unmaximize();
  } else {
    maximize();
  }
};

const handleMinimize = (): void => {
  emits('windowMinimize', true);
  invoke("app_close", { appId: windowData.value.index });
};

const handleDragStart = (e: MouseEvent): void => {
  if (
    e.target instanceof HTMLImageElement ||
    e.target instanceof HTMLButtonElement
  )
    return;

  document.body.style.cursor = "move";

  if (size.value.maximized) {
    // cursorPos(max)/width(max) = cursorPos(min)/width(min)
    const cursorWindowPercentageXMax = e.clientX / size.value.width;
    unmaximize();
    const cursorOffsetMin = cursorWindowPercentageXMax * size.value.width;
    pos.value.x = e.clientX - cursorOffsetMin;
    pos.value.y = 0;
  }

  document.addEventListener("mousemove", handleDrag);
  document.addEventListener("mouseup", handleDragEnd);

  dragOffset.value = {
    dx: e.clientX - pos.value.x,
    dy: e.clientY - pos.value.y,
  };
};

const handleDrag = (e: MouseEvent): void => {
  pos.value.x = Math.max(
    Math.min(
      e.clientX - dragOffset.value.dx,
      parentBox.value.width - size.value.width - 2
    ),
    0
  );
  pos.value.y = Math.max(
    Math.min(
      e.clientY - dragOffset.value.dy,
      parentBox.value.height - size.value.height
    ),
    0
  );
};

const handleDragEnd = (e: MouseEvent): void => {
  document.body.style.cursor = "initial";
  handleDrag(e);
  if (pos.value.y === 0) {
    maximize();
  }
  document.removeEventListener("mousemove", handleDrag);
  document.removeEventListener("mouseup", handleDragEnd);
};

const handleResizeStart = (e: MouseEvent): void => {
  const classList = (e.target as HTMLElement).classList;
  resizeType.value = {};
  if (classList.contains("resize-bar-right")) {
    resizeType.value.w = "r";
  } else if (classList.contains("resize-bar-left")) {
    resizeType.value.w = "l";
  }
  if (classList.contains("resize-bar-bottom")) {
    resizeType.value.h = "b";
  } else if (classList.contains("resize-bar-top")) {
    resizeType.value.h = "t";
  }

  document.addEventListener("mousemove", handleResize);
  document.addEventListener("mouseup", handleResizeEnd);
};

const handleResize = (e: MouseEvent): void => {
  if (resizeType.value?.w === "r") {
    const untilRightBorder = parentBox.value.width - pos.value.x;
    const x = e.clientX - parentBox.value.x - pos.value.x;
    size.value.width = Math.max(
      Math.min(x, untilRightBorder - 2),
      MIN_WINDOW_WIDTH.value
    );
  } else if (resizeType.value?.w === "l") {
    const x = e.clientX - parentBox.value.x;
    const untilMinWidth = pos.value.x + size.value.width - MIN_WINDOW_WIDTH.value;
    const newX = Math.max(Math.min(x, untilMinWidth - 2), 0);
    size.value.width = size.value.width + (pos.value.x - newX);
    pos.value.x = newX;
  }

  if (resizeType.value?.h === "b") {
    const untilParentHeight = parentBox.value.height - pos.value.y;
    const y = e.clientY - parentBox.value.y - pos.value.y;
    size.value.height = Math.max(
      Math.min(y, untilParentHeight),
      MIN_WINDOW_HEIGHT.value
    );
  } else if (resizeType.value?.h === "t") {
    const untilMinHeight =
      pos.value.y + size.value.height - MIN_WINDOW_HEIGHT.value;
    const y = e.clientY - parentBox.value.y;
    const newY = Math.max(Math.min(y, untilMinHeight), 0);
    size.value.height = size.value.height + (pos.value.y - newY);
    pos.value.y = newY;
  }

  emits('windowResize');
};

const handleResizeEnd = (e: MouseEvent): void => {
  handleResize(e);
  document.removeEventListener("mousemove", handleResize);
  document.removeEventListener("mouseup", handleResizeEnd);
};

const unlisten = listen("game_generated", () => {
  setTimeout(() => {
    emits('emailCreate', {
      title: "The Start of Your Incumbency",
      severity: 'normal',
      content: `
Hello, ${USERNAME}. You are now in leading position for the country's economy. Gary set the bar high during his incumbency, however, his sex scandal led him to step down from office. My name is Ned, I will be here to advise you about your user interface and and updates it may receive.

What you are currently looking at is the desktop environment that you will use to manage the economy. From here you will control everything - taxes, laws & regulations, healthcare and more. Take a look at your left panel and you will see all the available desktop apps that are at your disposal. You can open them and take a look around. Just.. try not to break everything, okay?

Kind regards,
Ned
`,
      sender: "Ned",
    })
  }, 5000);
});

listen("unemployed_high", ({ payload }: any) => {
  if (payload.severity === "mild") {
    emits('emailCreate', {
      title: "High Unemployment Rate",
      content: `
Hi ${USERNAME}, hope you're doing well. It has been brought to my attention that the unemployment rate for the country needs to be addressed, as it currently sits at ${payload.unemployed_count
        } people (${Number(payload.percent)}%).

This was likely caused by a large coorporation going bust. Ensure that you cover expenses for these people to keep them healthy while they seek for new employment.

Many thanks, Ralph
`,
      sender: "Ralph",
      severity: "warning",
    });
  } else {
    emits('emailCreate', {
      title: "Huge business has gone bust!",
      content: `
Hello ${USERNAME}, you need to act ASAP! A large business has just gone bust and ${payload.unemployed_count
        } people (${Number(
          payload.percent
        )}%) are now unemployed. Fund expenses for as many people as you can while they seek new employment.
`,
      sender: "Ralph",
      severity: "error",

    });
  }
});

// styles
const resizeBarLeftStyle = `
  width: ${RESIZE_BAR_SIZE};
  height: calc(100% - ${RESIZE_BAR_SIZE} * 2);
  top: ${RESIZE_BAR_SIZE};
`;
const resizeBarTopStyle = `
  width: calc(100% - ${RESIZE_BAR_SIZE} * 2);
  height: ${RESIZE_BAR_SIZE};
  left: ${RESIZE_BAR_SIZE};
`;
const resizeBarCornerStyle = `
  width: ${RESIZE_BAR_SIZE};
  height: ${RESIZE_BAR_SIZE};
`;
const resizeBarBottomStyle = resizeBarTopStyle;
const resizeBarRightStyle = resizeBarLeftStyle;
const parentStyle = computed(() => `
    display: ${windowData.value.opened ? 'initial' : 'none'};
    left: ${pos.value.x}px;
    top: ${pos.value.y}px;
    width: ${size.value.width}px;
    height: ${size.value.height}px;
    z-index: ${windowData.value.focused ? 10_000 : 9999};
`);
const viewPortStyle = `
  width: 100%;
  height: calc(100% - ${WINDOW_HEADER_HEIGHT});
`;
const parentHeaderStyle = `height: ${WINDOW_HEADER_HEIGHT};`;
</script>

<!-- PARENT COMPONENT -->
<template v-model="thisObj">
  <div class="header" :style=parentHeaderStyle @mousedown=handleDragStart>
    <button class="close-button" title="Close" @click=handleClose>
      Close
    </button>

    <button class="minimize-button" title="Minimize" @click=handleMinimize>
      Minimize
    </button>

    <button class="maximize-button" title="Maximize" @click=handleMaximize>
      Maximize
    </button>

    <div>
      {{ title }}
    </div>
  </div>

  <!-- Viewport -->
  <div class="viewport" :style=viewPortStyle>
    <slot />

    <div class="resize-bar-left" :style=resizeBarLeftStyle @mousedown=handleResizeStart></div>
    <div class="resize-bar-right" :style=resizeBarRightStyle @mousedown=handleResizeStart></div>
    <div class="resize-bar-top" :style=resizeBarTopStyle @mousedown=handleResizeStart></div>
    <div class="resize-bar-bottom" :style=resizeBarBottomStyle @mousedown=handleResizeStart></div>

    <div class="resize-bar-top resize-bar-left" :style=resizeBarCornerStyle @mousedown=handleResizeStart></div>
    <div class="resize-bar-bottom resize-bar-right" :style=resizeBarCornerStyle @mousedown=handleResizeStart></div>
    <div class="resize-bar-bottom resize-bar-left" :style=resizeBarCornerStyle @mousedown=handleResizeStart></div>
    <div class="resize-bar-top resize-bar-right" :style=resizeBarCornerStyle @mousedown=handleResizeStart></div>
  </div>
</template>

<style scoped>
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

.header>button {
  padding: 0 1em 0 1em;
  border-right: 1px solid var(--color-accent);
}

.header>button:hover {
  color: var(--color-bg);
  background-color: var(--color-accent);
  font-weight: bold;
}

.header>div {
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
