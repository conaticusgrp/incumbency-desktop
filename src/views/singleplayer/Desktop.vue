<script setup lang="ts">
import { listen } from "@tauri-apps/api/event";
import { exit as shutdownTauri, relaunch as restartTauri } from "@tauri-apps/api/process";
import {
    APP_LIST_MIN_WIDTH,
    APP_LIST_WIDTH,
    TOP_PANEL_HEIGHT,
    TOOLBAR_HEIGHT,
} from "src/constants";
import { computed, onMounted, ref } from "vue";
import { useAppStore, type AppState } from "src/store/apps.js";
import Window from "src/components/Window.vue";
import { useRouter } from "vue-router";
import { invoke } from "@tauri-apps/api/tauri";
import wallpaper from 'src/assets/Wallpaper.png';

const router = useRouter();
const startMenu = ref<HTMLElement | null>(null);
const startMenuExpanded = ref(false);
const date = ref("00/00/0000");
const appStore = useAppStore();
const windowTransition = ref('');

const onWindowClose = (appName: string) => appStore.close(appName);
const onWindowAquireFocus = (appName: string) => appStore.acquireFocus(appName);
const onWindowOpen = (appName: string) => appStore.openApp(appName);
const onWindowMinimize = (appName: string) => appStore.hide(appName);
const onWindowResize = (appName: string, size: Size) => appStore.resize(appName, size);
const onWindowPlace = (appName: string, placement: Placement) => appStore.place(appName, placement);

const openStartMenu = () => startMenuExpanded.value = true;

const closeStartMenu = () => startMenuExpanded.value = false;

const toggleStartMenu = (): void => {
    if (startMenuExpanded.value) {
        closeStartMenu();
    } else {
        openStartMenu();
    }
};

const toggleNotificationsSection = () => { };

const getApp = (appName: string) => {
    const app = appStore.apps[appName];

    if (!app) {
        console.error(`appStore app [${appName}] is undefined!`);
    }

    return app ?? { opened: false, minimized: false, badgeCount: 0 };
};

const updateApp = (appName: string, cb: (app: AppState) => void): void => {
    appStore.setAppState(appName, cb);
};

const getBadgeCount = (appName: string) => getApp(appName).badgeCount;
const shutdown = () => shutdownTauri(0);
const logoff = () => router.push({ name: "new-game" });
const restart = () => restartTauri();

// Events
listen<{ date: string }>("new_day", ({ payload }) => {
    date.value = payload.date;
});

document.addEventListener("keydown", (e) => {
    if (e.altKey && e.key == "F4") {
        const focusedApp = appStore.focusedApp;
        if (focusedApp !== null) {
            updateApp(focusedApp, (app) => (app.opened = false));
            appStore.acquireFocus(null);
        }
        e.preventDefault();
    }
});

// styles
const appListSectionStyle = `width: ${APP_LIST_WIDTH}; min-width: ${APP_LIST_MIN_WIDTH};`;
const getAppShortcutStyle = (appName: string): string => {
    const cssVar = getApp(appName).opened
        ? "--color-highlight"
        : "--color-shaded";
    return `color: var(${cssVar});`;
};
const contentStyle = `width: calc(100% - ${APP_LIST_WIDTH});`;
const topPanelStyle = `height: ${TOP_PANEL_HEIGHT};`;
const toolbarStyle = `height: ${TOOLBAR_HEIGHT};`;
const windowsStyle = computed(() => `height: calc(100% - ${TOP_PANEL_HEIGHT} - ${TOOLBAR_HEIGHT}); background: 'black';`);
const mainStyle = computed(() => `
    background-image: url('${wallpaper}'); 
    background-repeat: no-repeat;
    background-position: center center;
`);
const getWindowStyle = (appName: string) => {
    const app = getApp(appName);
    const { pos, size } = app.window;
    const { opened } = app;
    const focused = appStore.focusedApp === appName;

    return `
        display: ${opened ? 'initial' : 'none'};
        left: ${pos.x}px;
        top: ${pos.y}px;
        width: ${size.width}px;
        height: ${size.height}px;
        z-index: ${focused ? 10_000 : 9999};
    ` + windowsStyle.value;
}

onMounted(async () => {
    await invoke("create_game", {
        name: "some dumbass name idc anymore lets just finish this on time",
    });
});
</script>

<template>
    <div class="desktop" :style="mainStyle">
        <div class="app-list-section" :style="appListSectionStyle">
            <h2>Installed Software</h2>

            <div class="app-list">
                <div v-for="app in appStore.apps" :style="getAppShortcutStyle(app.appName)"
                    @click="onWindowOpen(app.appName)">
                    {{ app.appName }}
                    <span v-if="getBadgeCount(app.appName) > 0">({{ getBadgeCount(app.appName) }})</span>
                </div>
            </div>
        </div>

        <div class="content" :style="contentStyle">
            <div class="top-panel" :style="topPanelStyle" @click="toggleStartMenu()">
                <div class="start-menu" :aria-expanded="startMenuExpanded" v-bind="startMenu">
                    <div v-if="startMenuExpanded">
                        <button>Close</button>
                        <button @click="shutdown()">Shut down</button>
                        <button @click="logoff()">Logoff</button>
                        <button @click="restart()">Restart</button>
                    </div>
                    <span v-else>
                        {{ date }}
                    </span>
                </div>

                <button class="notification-section-toggle" @click="toggleNotificationsSection()">
                    Notifications
                </button>
            </div>

            <div>
                <div v-for="app in appStore.apps">
                    <Window v-if="app.appName === appStore.focusedApp" :style="getWindowStyle(app.appName)" class="windows"
                        :index="app.index" :app-name="app.appName" :tabs="app.tabs" :title="app.window.title"
                        @window-close="onWindowClose(app.appName)" @window-maximize="onWindowPlace(app.appName, $event)"
                        @window-resize="onWindowResize(app.appName, $event)"
                        @window-aquire-focus="onWindowAquireFocus(app.appName)" @window-opened="onWindowOpen(app.appName)"
                        @window-minimize="onWindowMinimize(app.appName)">
                        <component class="window-content" :is="app.component" />
                    </Window>
                </div>
                <Notifications />
            </div>

            <div class="toolbar" :style="toolbarStyle">
                <div v-for="state in appStore.apps">
                    <span v-if="state.opened" @click="onWindowAquireFocus(state.appName)">
                        {{ state.appName }}
                    </span>
                </div>
            </div>
        </div>
    </div>
</template>

<style scoped>
.desktop {
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

.notification-section-toggle {
    position: absolute;
    top: 0;
    right: 0;
    height: 100%;
    border-left: 1px solid var(--color-accent);
}

.windows {
    position: relative;
    z-index: 100;
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
