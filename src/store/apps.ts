import { defineStore } from "pinia";
import { Component, ref } from "vue";

export type CreateApp = {
    appName: string;
    component: Component;
    tabs: Tab[];
    window: {
        title: string;
        pos: Pos;
        size: Size;
    };
    index: number;
};

export type AppState = {
    badgeCount: number;
    opened: boolean;
    minimized: boolean;
} & CreateApp;

// NOTE(dylhack): Steve Jobs not included
export const useAppStore = defineStore("windows", () => {
    const focusedApp = ref<string | null>(null);
    const apps = ref<{ [key: string]: AppState }>({});

    function registerApp(app: CreateApp) {
        if (!(app.appName in apps.value)) {
            apps.value[app.appName] = {
                ...app,
                badgeCount: 0,
                opened: false,
                minimized: false,
                index: app.index,
            };
        }
    }

    function setAppState(appName: string, cb: (app: AppState) => void) {
        const app = apps.value[appName];
        if (app) {
            cb(app);
            apps.value[appName] = app;
        } else {
            console.error(`App ${appName} not found`);
        }
    }

    function resize(appName: string, size: Size) {
        setAppState(appName, (app) => (app.window.size = size));
    }

    function move(appName: string, pos: Pos) {
        setAppState(appName, (app) => (app.window.pos = pos));
    }

    function openApp(appName: string) {
        setAppState(appName, (app) => {
            focusedApp.value = appName;
            app.opened = true;
            app.minimized = false;
            app.badgeCount = 0;
        });
    }

    function acquireFocus(appName: string | null) {
        focusedApp.value = appName;
    }

    function place(appName: string, placement: Placement) {
        setAppState(appName, (app) => {
            app.window.pos = placement.pos;
            app.window.size = placement.size;
        });
    }

    // NOTE(dylhack): minimize
    function hide(appName: string) {
        setAppState(appName, (app) => (app.minimized = true));
    }

    function close(appName: string) {
        setAppState(appName, (app) => (app.opened = false));
        if (focusedApp.value === appName) {
            focusedApp.value = null;
        }
    }

    // NOTE(dylhack): unminimize
    function show(appName: string) {
        setAppState(appName, (app) => (app.minimized = false));
    }

    return {
        apps,
        registerApp,
        setAppState,
        openApp,
        acquireFocus,
        hide,
        focusedApp,
        close,
        show,
        resize,
        move,
        place,
    };
});
