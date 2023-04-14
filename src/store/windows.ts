import { defineStore } from "pinia";
import { ref } from "vue";

export type AppState = {
  badgeCount: number;
  opened: boolean;
  minimized: boolean;
}

// NOTE(dylhack): Bill Gates not included
export const useWindowStore = defineStore('windows', () => {
  const focusedApp = ref<string | null>(null);
  const apps = ref<{ [key: string]: AppState }>({});

  function newApp(appName: string) {
    apps.value[appName] = {
      badgeCount: 0,
      opened: false,
      minimized: false,
    };
  }

  function setAppState(appName: string, cb: (app: AppState) => void) {
    const app = apps.value[appName];
    if (app) {
      cb(app);
      apps.value[appName] = app;
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

  function acquireFocus(appName: string) {
    if (appName in apps.value) {
      focusedApp.value = appName;
    }
  }

  function minimize(appName: string) {
    setAppState(appName, app => app.minimized = true);
  }

  function close(appName: string) {
    setAppState(appName, app => app.opened = false);
    if (focusedApp.value === appName) {
      focusedApp.value = null;
    }
  }

  function unminimize(appName: string) {
    setAppState(appName, app => app.minimized = false);
  }

  return { apps, newApp, openApp, acquireFocus, minimize, focusedApp, close, unminimize };
});
