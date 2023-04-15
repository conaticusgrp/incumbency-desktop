<script setup lang="ts">
import { listen, once } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/tauri";
import {
  MIN_WINDOW_HEIGHT,
  MIN_WINDOW_WIDTH,
  RESIZE_BAR_SIZE,
  TAB_LIST_ENTRY_MARGIN,
  TAB_LIST_MIN_WIDTH,
  TAB_LIST_WIDTH,
  USERNAME,
  USERNAME_HEIGHT,
  WINDOW_HEADER_HEIGHT,
} from "../constants";
import { PropType, computed, ref, nextTick, useSlots, Component } from "vue";
import { Action, Severity, useNotificationsStore } from "src/store/notifications";
import { useEmailsStore, useEmails } from "src/store/emails";
import { type WindowEvents } from 'src/store/apps';

enum Apps {
  Finance = 1,
  Healthcare = 2,
  Welfare = 3,
  Business = 4,
}

type AppString = "finance" | "healthcare" | "welfare" | "business";

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
  appName: { required: true, type: String },
  tabs: { type: Array as PropType<Component[]>, default: () => [] },
});
const isTabbed = computed(() => props.tabs.length > 0);
const emits = defineEmits<WindowEvents>();

const defaultCriticalWindowData = () => ({ opened: false, focused: false, index: -1 });
const notiStore = useNotificationsStore();
const emailStore = useEmailsStore();
const emails = useEmails(USERNAME);
const pos = ref<Pos>(props.pos);
const size = ref<Size>(props.size);
const windowData = ref<CriticalWindowData>(defaultCriticalWindowData());
const thisObj = ref<HTMLElement | null>(null);
const dragOffset = ref({ dx: 0, dy: 0 });
const resizeType = ref<{
  w?: "r" | "l", h?: "t" | "b"
}>();
const boundsBeforeMaximizing = ref({ x: 0, y: 0, width: 0, height: 0, });
const transition = ref("");
const getAppNameFromId = (id: number): AppString => {
  return Apps[id].toLowerCase() as any;
};

nextTick(async () => {
  if (!windowData.value.opened || windowData.value.focused) return;
  try {
    const d = await invoke<OpenEvents>("app_open", {
      appId: windowData.value.index,
    });
    emits("windowOpened", d);
  } catch (e) {
    console.error(e);

    notiStore.addNotification({
      app: props.title,
      header: "App open error",
      content: "Error occured while opening the app",
      severity: Severity.Error,
      action: Action.Nothing,
    });
  }
})

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
      const safeBudget = -appData.expected_balance + totalBudgetSpending + totalBudgetSpending * 0.3
      emailStore.createEmail(emails.tarun.expectedCrisis(safeBudget, appData.expected_balance, totalBudgetSpending));
    } else if (!enoughSaved) {
      const safeBudget = totalBudgetSpending * 0.3 + totalBudgetSpending
      const expectedBalance = appData.expected_balance;
      emailStore.createEmail(emails.tarun.expectedBalance(totalBudgetSpending, safeBudget, expectedBalance))
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

once("game_generated", () => {
  setTimeout(() => {
    emailStore.createEmail({
      title: "The Start of Your Incumbency",
      severity: Severity.Normal,
      content: `
Hello, ${USERNAME}. You are now in leading position for the country's economy. Gary set the bar high during his incumbency, however, his sex scandal led him to step down from office. My name is Ned, I will be here to advise you about your user interface and and updates it may receive.

What you are currently looking at is the desktop environment that you will use to manage the economy. From here you will control everything - taxes, laws & regulations, healthcare and more. Take a look at your left panel and you will see all the available desktop apps that are at your disposal. You can open them and take a look around. Just.. try not to break everything, okay?

Kind regards,
Ned
`,
      sender: "Ned",
    });
  }, 5000);
});

listen<{ severity: string; percent: number; unemployed_count: number }>("unemployed_high", ({ payload }) => {
  if (payload.severity === "mild") {
    emailStore.createEmail(emails.ralph.highUneploymentEmail(payload.percent, payload.unemployed_count));
  } else {
    emailStore.createEmail(emails.ralph.businessBust(payload.percent, payload.unemployed_count))
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
const parentStyle = `
    display: ${windowData.value.opened ? 'initial' : 'none'};
    left: ${pos.value.x}px;
    top: ${pos.value.y}px;
    width: ${size.value.width}px;
    height: ${size.value.height}px;
    z-index: ${windowData.value.focused ? 10_000 : 9999};
`;
const viewPortStyle = `
  width: 100%;
  height: calc(100% - ${WINDOW_HEADER_HEIGHT});
`;
const parentHeaderStyle = `height: ${WINDOW_HEADER_HEIGHT};`;


const slots = useSlots();
const slotArray = Object.keys(slots);
const currentTabI = ref(0);
const onTabSelect = (newTabI: number) => {
  currentTabI.value = newTabI;
}

// styles
const tabbedWindow = `
  --tab-list-width: ${TAB_LIST_WIDTH};
  --tab-list-min-width: ${TAB_LIST_MIN_WIDTH};
  --tab-list-entry-margin: ${TAB_LIST_ENTRY_MARGIN};
  --username-height: ${USERNAME_HEIGHT};
`;
</script>

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

  <!-- Regular Window -->
  <div v-if="!isTabbed" class="window regular-window" :style=viewPortStyle>
    <slot/>

    <div class="resize-bar-left" :style=resizeBarLeftStyle @mousedown=handleResizeStart></div>
    <div class="resize-bar-right" :style=resizeBarRightStyle @mousedown=handleResizeStart></div>
    <div class="resize-bar-top" :style=resizeBarTopStyle @mousedown=handleResizeStart></div>
    <div class="resize-bar-bottom" :style=resizeBarBottomStyle @mousedown=handleResizeStart></div>

    <div class="resize-bar-top resize-bar-left" :style=resizeBarCornerStyle @mousedown=handleResizeStart></div>
    <div class="resize-bar-bottom resize-bar-right" :style=resizeBarCornerStyle @mousedown=handleResizeStart></div>
    <div class="resize-bar-bottom resize-bar-left" :style=resizeBarCornerStyle @mousedown=handleResizeStart></div>
    <div class="resize-bar-top resize-bar-right" :style=resizeBarCornerStyle @mousedown=handleResizeStart></div>
  </div>
  <!-- Tabbed Window -->
  <div v-else :style=tabbedWindow class="window tabbed-window">
      <section>
        <div class="tab-list">
          <div v-for="tabName, i in slotArray">
            <TabButton :selected="i === currentTabI" @select-tab=onTabSelect(i)>
              {{ tabName }}
            </TabButton>
          </div>
        </div>

        <div class="username">
          Authenticated as:
          <div>{{ USERNAME }}</div>
        </div>
      </section>

      <section>
        <div v-for="slotName, i in slotArray">
          <slot :name=slotName v-if="i === currentTabI"></slot>
        </div>
      </section>
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

.regular-window {
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
