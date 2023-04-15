import { defineStore } from "pinia";
import { Component, ref } from "vue";

export type CreateApp = {
  appName: string;
  component: Component;
  tabs: Component[];
  window: {
    title: string;
    pos: Pos;
    size: Size;
  };
}

export type AppState = {
  badgeCount: number;
  opened: boolean;
  minimized: boolean;
} & CreateApp;

export type WindowEvents = {
  (e: 'appUpdate', data: UpdateAppPayloads): void;
  (e: 'windowOpened', data: OpenEvents): void;
  (e: 'windowResize'): void;
  (e: 'windowAquireFocus'): void;
  (e: 'windowMinimize', min: boolean): void;
  (e: 'windowUnminimize', min: boolean): void;
  (e: 'windowMaximize', max: boolean): void;
  (e: 'windowClose'): void;
}

// NOTE(dylhack): Steve Jobs not included
export const useAppStore = defineStore('windows', () => {
  const focusedApp = ref<string | null>(null);
  const apps = ref<{ [key: string]: AppState }>({});

  function registerApp(app: CreateApp) {
    if (!(app.appName in apps.value)) {
      apps.value[app.appName] = {
        ...app,
        badgeCount: 0,
        opened: false,
        minimized: false,
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

  function openApp(appName: string) {
    setAppState(appName, app => {
      focusedApp.value = appName;
      app.opened = true;
      app.minimized = false
      app.badgeCount = 0;
    });
  }

  function acquireFocus(appName: string | null) {
    focusedApp.value = appName;
  }

  // NOTE(dylhack): minimize
  function hide(appName: string) {
    setAppState(appName, app => app.minimized = true);
  }

  function close(appName: string) {
    setAppState(appName, app => app.opened = false);
    if (focusedApp.value === appName) {
      focusedApp.value = null;
    }
  }

  // NOTE(dylhack): unminimize
  function show(appName: string) {
    setAppState(appName, app => app.minimized = false);
  }

  return { apps, registerApp, setAppState, openApp, acquireFocus, hide, focusedApp, close, show };
});
