<script setup lang="ts">
import { listen } from "@tauri-apps/api/event";
import {
    APP_LIST_MIN_WIDTH,
    APP_LIST_WIDTH,
    TOP_PANEL_HEIGHT,
    TOOLBAR_HEIGHT,
    MODAL_TIMER_DELAY,
} from "src/constants";
import { computed, nextTick, ref, useSlots } from "vue";
import { NotificationData } from "src/store/notifications";
import Notifications from "src/components/Notifications.vue";

// import Email from "../windows/Email/Email.svelte";
// import Finance from "../windows/Finance/Finance.svelte";
// import Healthcare from "../windows/Healthcare/Healthcare.svelte";
// import Welfare from "../windows/Welfare/Welfare.svelte";
// import Business from "../windows/Business/Business.svelte";
// import Notification from "src/components/Notification.vue";

type DesktopAppShortcut = {
    componentConstructor: any;
    name: string;
    component?: any;
    props?: any;
    badgeCount: number;
    opened?: boolean;
    minimized?: boolean;
}

type ModalState = "closed" | "log off" | "shut down";

type AppState = {
    badgeCount: number;
    opened: boolean;
    minimized: boolean;
}

let startMenu: HTMLElement;
let startMenuExpanded: boolean = false;
let notificationSectionExpanded = false;
let date: string = "undefined date";
let wallpaperPath: string | null = "./src/assets/Wallpaper.png";
const apps = useSlots();
const appNames = Object.keys(apps);
const appStates = ref<AppState[]>([]);
const focusedApp = ref<number | null>(null);
// let apps: DesktopAppShortcut[] = [
//     { componentConstructor: Email, name: "Email", badgeCount: 0 },
//     { componentConstructor: Finance, name: "Finance", badgeCount: 0 },
//     { componentConstructor: Healthcare, name: "Healthcare", badgeCount: 0 },
//     { componentConstructor: Welfare, name: "Welfare", badgeCount: 0 },
//     { componentConstructor: Business, name: "Business", badgeCount: 0 },
// ];

const notifications = ref<NotificationData[]>([]);
const modalState = ref<ModalState>("closed");
let modalTimerResolve: (() => void) | null = null;
let modalTimerCountdown = 0;

nextTick(() => {
    if (modalState.value !== "closed") {
        new Promise<void>(async (resolve) => {
            modalTimerResolve = resolve;
            for (let i = 0; i < MODAL_TIMER_DELAY; i++) {
                modalTimerCountdown = MODAL_TIMER_DELAY - i;
                await new Promise<void>((res0) => {
                    setTimeout(res0, 1_000);
                });
            }
            modalTimerResolve = null;
            modalResolve();
            resolve();
        });
    }
});

const onWindowSendNotification = (i: number, n: NotificationData) => receiveNotification(n);

const handleCriticalEvent = (index: number, e: CustomEvent): void => {
    switch (e.detail.type) {
        case WINDOW_CLOSE:
            {
            }
            break;

        case WINDOW_AQUIRE_FOCUS:
            {
            }
            break;

        case WINDOW_MINIMIZE:
            {
            }
            break;

        case WINDOW_SEND_NOTIFICATION:
            {
            }
            break;

        default:
            break;
    }
};

const openStartMenu = (): void => {
    startMenuExpanded = true;

    document.addEventListener("click", closeStartMenuIfClickedAway);
};

// const toggleNotificationsSection = (): void => {
//     notificationSectionExpanded = !notificationSectionExpanded;

//     if (notificationSectionExpanded) {
//         notifications.value.forEach((noti, i) => {
//             noti.shown = true;
//             notifications.value[i] = noti;
//         });
//         for (let i = 0; i < notifications.value.length; i++) {
//             notifications.value[i].shown = false;
//         }
//     }
// };

const closeStartMenuIfClickedAway = (e: MouseEvent): void => {
    if (startMenu == null) return;

    if (e.target == null || startMenu.contains(e.target as HTMLElement))
        return;

    startMenuExpanded = false;
};

// const receiveNotification = (n: NotificationData): void => {
//     const app = apps.find(
//         (app) => app.name.toLowerCase() === n.app?.toLowerCase()
//     );

//     if (app && !app.opened) {
//         app.badgeCount += 1;
//     }

//     n.date = date;
//     n.shown = !notificationSectionExpanded;

//     notifications = [...notifications, n]; // for mutability updates

//     updateUI();
// };

const modalTitle = computed(() => {
    switch (modalState.value) {
        case "log off":
            return "logoff";
        case "shut down":
            return "shutdown";
        default:
            return "";
    }
});

const modalAutoAction = computed(() => {
    switch (modalState.value) {
        case "log off":
            return "Logging off";
        case "shut down":
            return "Shutting down";
        default:
            return "";
    }
});

const modalAction = computed(() => {
    switch (modalState.value) {
        case "log off":
            return "Log off";
        case "shut down":
            return "Shut down";
        default:
            return "";
    }
});

const modalResolve = (): void => {
    if (modalTimerResolve != null) modalTimerResolve();

    switch (modalState.value) {
        case "log off":
            break;
        case "shut down":
            break;
        default:
            break;
    }
    modalState.value = "closed";
};

const modalReject = (): void => {
    if (modalTimerResolve != null) modalTimerResolve();

    switch (modalState.value) {
        case "log off":
            break;
        case "shut down":
            break;
        default:
            break;
    }
    modalState.value = "closed";
};

listen<{ date: string }>("new_day", ({ payload }) => {
    date = payload.date as string;
});

document.addEventListener("keydown", (e) => {
    if (e.altKey && e.key == "F4") {
        if (focusedApp.value !== null) {
            updateApp(focusedApp.value, (app) => app.opened = false);
            focusedApp.value = null;
        }
        e.preventDefault();
    }
});

const getApp = (i: number) => {
    const app = appStates.value[i];
    console.error(`appStates.value[${i}] is undefined!`);
    return app ?? { opened: false, minimized: false, badgeCount: 0 }
}

const updateApp = (i: number, cb: (app: AppState) => void): void => {
    const app = appStates.value[i];
    if (app) {
        cb(app);
        appStates.value[i] = app;
    } else {
        console.log(`appStates.value[${i}] is undefined!`);
    }
}

const getBadgeCount = (i: number) => getApp(i).badgeCount;

const shutdown = () => modalState.value = "shut down";
const restart = () => { };
const logOff = () => modalState.value = "log off";
const setFocusedApp = (i: number) => {
    focusedApp.value = i;
    unminimizeApp(i);
}

// styles
const appListSectionStyle = `width: ${APP_LIST_WIDTH}; min-width: ${APP_LIST_MIN_WIDTH};`;
const appShortcutStyle = (i: number): string => {
    const cssVar = getApp(i).opened
        ? "--color-highlight"
        : "--color-shaded";
    return `color: var(${cssVar});`;
}
const contentStyle = `width: calc(100% - ${APP_LIST_WIDTH});`;
const topPanelStyle = `height: ${TOP_PANEL_HEIGHT};`;
const toolbarStyle = `height: ${TOOLBAR_HEIGHT};`;
const windowsStyle = computed(() => {
    const background = wallpaperPath !== null ? `url(${wallpaperPath})` : "none";
    const style = `height: calc(100% - ${TOP_PANEL_HEIGHT} - ${TOOLBAR_HEIGHT}); background-image: ${background};`;
    return style;
});
</script>

<template>
    <main>
        <div class="app-list-section" :style=appListSectionStyle>
            <h2>Installed Software</h2>

            <div class="app-list">
                <div v-for="appName, i in appNames" :style={ appShortcutStyle(i) } @click="handleOpenApp(i)">
                    {{ appName }}
                    <span v-if="getBadgeCount(i) > 0">({{ getBadgeCount(i) }})</span>
                </div>
            </div>
        </div>

        <div class="content" :style="contentStyle">
            <div class="top-panel" :style=topPanelStyle @click=openStartMenu()>
                <div class="start-menu" :aria-expanded="startMenuExpanded" bind:this={startMenu}>
                    <span v-if="!startMenuExpanded">
                        {{ date }}
                    </span>
                    <div v-else>
                        <button>{date}</button>
                        <button @click=shutdown()>Shut down</button>
                        <button @click=logOff()>Logoff</button>
                        <button @click=restart()>Restart</button>
                    </div>
                </div>

                <button class="notification-section-toggle" on:click={toggleNotificationsSection}>
                    Notifications
                </button>
            </div>

            <div class="windows" :style="windowsStyle">
                <div v-for="appName, i in appNames">
                    <slot :name="appName" v-if="getApp(i).opened"></slot>
                </div>
                <Notifications />
            </div>

            <div class="toolbar" :style=toolbarStyle>
                <div v-for="state, i in appStates">
                    <span v-if="state.opened" @click="setFocusedApp(i)">
                        {{ appNames[i] }}
                    </span>
                </div>
            </div>
        </div>

        <div v-if="modalState !== 'closed'" class="modal">
            <div class="modal-label">
                <div class="content">
                    <h1>{{ modalTitle }}</h1>
                    <span>{{ modalAutoAction }} automatically in {{ modalTimerCountdown }}s</span>
                </div>

                <div class="actions">
                    <button @click=modalResolve()>{{ modalAction }}</button>
                    <button @click=modalReject()>Cancel</button>
                </div>
            </div>
        </div>
    </main>
</template>

<style scoped>
main {
    display: flex;
    flex-direction: row;
    position: relative;
    width: 100%;
    height: 100%;
    z-index: 0;
    isolation: isolate;
    color: var(--color-highlight);
    background-color: black;
}

.app-list-section {
    display: flex;
    flex-direction: column;
    align-items: center;
    border-right: 1px solid var(--color-accent);
}

.app-list-section>h2 {
    margin: 2em;
    font-size: 14px;
    font-weight: bolder;
}

.app-list {
    display: flex;
    flex-direction: column;
    align-items: flex-start;
    width: calc(100% - 1em * 2);
    margin: 0 1em 0 1em;
    font-size: 14px;
}

.app-list>div {
    width: 100%;
    margin: 0.5em 0 0.5em 0;
    text-align: left;
    cursor: pointer;
}

.app-list>div>span {
    color: var(--color-critical);
    font-weight: bold;
}

.content {
    display: flex;
    flex-direction: column;
}

.top-panel {
    display: flex;
    justify-content: center;
    /* align-items: center; */
    position: relative;
    min-height: min-content;
    border-bottom: 1px solid var(--color-accent);
}

.start-menu {
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;
    width: min-content;
    height: min-content;
    cursor: pointer;
}

.start-menu[aria-expanded="false"] {
    align-self: center;
}

.start-menu[aria-expanded="true"] {
    width: 30%;
    z-index: 2;
    background-color: var(--color-bg);
    border: 1px solid var(--color-accent);
    border-top: none;
}

.start-menu[aria-expanded="true"]>button {
    width: 100%;
}

.start-menu[aria-expanded="true"]>button:hover {
    background-color: var(--color-accent);
    color: var(--color-bg);
    font-weight: bold;
}

/* 
    .single-notification-container {
        position: absolute;
        top: 0;
        z-index: 20000;
    } */

.notification-section-toggle {
    position: absolute;
    top: 0;
    right: 0;
    height: 100%;
    border-left: 1px solid var(--color-accent);
}

.windows {
    position: relative;
    /* z-index: 100; */
    isolation: isolate;
    background-repeat: no-repeat;
    background-position: center;
}

.toolbar {
    /* width: 100%; */
    display: flex;
    flex-direction: row;
    min-height: min-content;
    border-top: 1px solid var(--color-accent);
}

.toolbar>span {
    position: relative;
    display: flex;
    justify-content: center;
    align-items: center;
    padding: 0 2em 0 2em;
    border: 1px solid var(--color-accent);

    background-color: var(--color-accent);
    color: var(--color-bg);
    font-weight: bold;
}

.toolbar>span[data-minimized="true"] {
    background-color: unset;
    color: unset;
    font-weight: unset;
}

.notifications-section {
    display: flex;
    flex-direction: column;
    align-items: center;
    position: absolute;
    top: 0;
    right: 0;
    height: 100%;
    background-color: var(--color-bg);
    border-left: 1px solid var(--color-accent);
    z-index: 20000;
}

.modal {
    display: flex;
    align-items: center;
    position: absolute;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    background-color: rgba(0, 0, 0, 0.5);
    z-index: 9999;
}

.modal-label {
    display: flex;
    flex-direction: column;
    width: 640px;
    aspect-ratio: 5 / 0.6;
    margin: auto;
    background-color: var(--color-bg);
    border: 1px solid var(--color-accent);
    border-radius: 1rem;
}

.modal-label>.content {
    height: 100%;
    border-radius: 0.5em;
    background-color: var(--color-accent);
    color: var(--color-bg);
    font-weight: bold;
}

.modal-label>.content>h1 {
    padding: 1em 0 0.5em 0;
    letter-spacing: 0.25em;
    text-align: center;
    text-transform: uppercase;
    white-space: nowrap;
    text-overflow: ellipsis;
    font-weight: bold;
    font-size: 20px;
}

.modal-label>.actions {
    display: flex;
}

.modal-label>.actions>button {
    width: 100%;
}

.modal-label>.actions>button:last-of-type {
    border-left: 1px solid var(--color-accent);
    color: grey;
}
</style>
