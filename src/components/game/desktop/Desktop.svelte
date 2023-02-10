<script lang="ts" context="module">

  interface DesktopAppShortcut {
    componentConstructor: any,
    name: string,

    component?: any,
    props?: any,
    badgeCount?: number,
    opened?: boolean,
    minimized?: boolean
  };

</script>

<script lang="ts">
  import { listen } from "@tauri-apps/api/event";
  import Notification, { type NotificationData } from "./Notification.svelte";
  import { APP_LIST_MIN_WIDTH, APP_LIST_WIDTH, TOP_PANEL_HEIGHT, NOTIFICATION_MARGIN_X, NOTIFICATION_WIDTH, TOOLBAR_HEIGHT } from "../../../scripts/desktopConstants";
  import { WINDOW_AQUIRE_FOCUS, WINDOW_CLOSE, WINDOW_MINIMIZE } from "../../../scripts/windowEvent";

  import DebuggerApp from "../windows/DebuggerApp.svelte";

  import Email from "../windows/Email.svelte";
  import Finance from "../windows/Finance.svelte";
  import Healthcare from "../windows/Healthcare.svelte";
  import Welfare from "../windows/Welfare.svelte";
  import Business from "../windows/Business.svelte";
  
  let startMenu: HTMLElement;
  let startMenuExpanded: boolean = false;
  let notificationSectionExpanded = false;
  let date: string = "undefined date";
  let wallpaperPath: string | null = "./src/assets/Wallpaper.png";
  let apps: DesktopAppShortcut[] = [
    { componentConstructor: DebuggerApp, name: "DEBUG" },
    { componentConstructor: Email,       name: "Email", badgeCount: 2 },
    { componentConstructor: Finance,     name: "Finance", badgeCount: 1 },
    { componentConstructor: Healthcare,  name: "Healthcare" },
    { componentConstructor: Welfare,     name: "Welfare" },
    { componentConstructor: Business,    name: "Business" }
  ];
  let focusedApp: number | null = null;

  let notifications: NotificationData[] = [
    {
      app: "debug",
      header: "Test",
      content: "Test notification",
      date: "now",
      iconPath: "https://w7.pngwing.com/pngs/821/338/png-transparent-warning-sign-computer-icons-warning-icon-angle-triangle-warning-sign-thumbnail.png"
    }
  ];

  const handleOpenApp = (index: number): void => {
    if (index < 0 || index >= apps.length) return;

    if (!apps[index].opened) {
      apps[index].opened = true;
      apps[index].badgeCount = 0;
      focusedApp = index;
      updateUI();
    } else {
      unminimizeApp(index);
    }
  };

  const unminimizeApp = (index: number) => {
    if (index < 0 || index >= apps.length) return;

    apps[index].minimized = !apps[index].minimized;
    updateUI();
  };

  const updateUI = () => {
    apps = apps;
  };

  const handleCriticalEvent = (index: number, e: CustomEvent): void => {
    if (index < 0 || index >= apps.length) return;

    switch (e.detail.type) {
      case WINDOW_CLOSE:
        {
          apps[index].opened = false;
          if (focusedApp === index) {
            focusedApp = null;
          }
          updateUI();
        }
        break;

      case WINDOW_AQUIRE_FOCUS:
        {
          focusedApp = index;
          updateUI();
        }
        break;

      case WINDOW_MINIMIZE:
        {
          apps[index].minimized = true;
          updateUI();
        }
        break;

      default:
        break;
    }
  };

  const openStartMenu = (): void => {
    startMenuExpanded = true;

    document.addEventListener('click', closeStartMenuIfClickedAway);
  }

  const toggleNotificationsSection = (): void => {
    notificationSectionExpanded = !notificationSectionExpanded;
  }

  const closeStartMenuIfClickedAway = (e: MouseEvent): void => {
    if (e.target == null || startMenu.contains(e.target as HTMLElement)) return;

    startMenuExpanded = false;
  }

  listen("new_day", (d) => {
    //@ts-ignore
    date = d.payload.date as string;
  });

  listen("open_debugger_app", (e) => {
    //@ts-ignore
    handleOpenApp(apps.findIndex((v) => v.name === "DEBUG"));
  });

  document.addEventListener("keydown", (e) => {
    if (e.altKey && e.key == "F4") {
      if (focusedApp != null) {
        apps[focusedApp].opened = false;
        focusedApp = null;
      }
      e.preventDefault();
    }
  });

</script>

<main>
  <div
    class="app-list-section"
    style="width: {APP_LIST_WIDTH}; min-width: {APP_LIST_MIN_WIDTH};"
  >
    <h2>Installed Software</h2>

    <div class="app-list">
      {#each apps as shortcut, i}
        <!-- empty on:keydown to supress a warning -->
        <div
          style="color: var({apps[i].opened
            ? '--color-highlight'
            : '--color-shaded'});"
          on:click={() => handleOpenApp(i)}
          on:keydown={() => {}}
        >
          {shortcut.name}

          {#if shortcut.badgeCount != undefined && shortcut.badgeCount > 0}
            <span>({shortcut.badgeCount})</span>
          {/if}
        </div>
      {/each}
    </div>
  </div>

  <div
    class="content"
    style="width: calc(100% - {APP_LIST_WIDTH});"
  >
    <div
      class="top-panel"
      style="height: {TOP_PANEL_HEIGHT};"
      on:click={openStartMenu}
      on:keydown={() => {}}
    >

      <div
        class="start-menu"
        aria-expanded={startMenuExpanded}
        bind:this={startMenu}
      >
        {#if !startMenuExpanded}

        {date}
        
        {:else}
        
        <button>{date}</button>
        <button>Shut down</button>
        <button>Logoff</button>
        <button>Restart</button>
        
        {/if}
      </div>

      <button
        class="notification-section-toggle"
        on:click={toggleNotificationsSection}
      >
        Notifications
      </button>

    </div>

    <div
      class="windows"
      style="
        height: calc(100% - {TOP_PANEL_HEIGHT} - {TOOLBAR_HEIGHT});
        background-image: {wallpaperPath != null
        ? `url(${wallpaperPath})`
        : 'none'};
      "
    >
      {#each apps as app, i}
        <!-- !! to cast (boolean | undefined) to boolean -->

        <svelte:component
          this={app.componentConstructor}
          bind:this={app.component}
          windowData={{
            opened: !!app.opened && !app.minimized,
            focused: i === focusedApp,
            index: i,
          }}
          {...app.props}
          on:criticalWindowEvent={(e) => handleCriticalEvent(i, e)}
        />
      {/each}

      {#if notificationSectionExpanded}
      <div
        class="notifications-section"
        style="width: calc({NOTIFICATION_WIDTH} + {NOTIFICATION_MARGIN_X} * 2);"
      >
        
        {#each notifications as notif}

        <Notification data={notif} />
        
        {/each}

      </div>
      {/if}

    </div>

    <div class="toolbar" style="height: {TOOLBAR_HEIGHT};">
      {#each apps as shortcut, i}
        {#if apps[i].opened}
          <!-- !! to cast (boolean | undefined) to boolean -->
          <!-- empty on:keydown to supress a warning -->
          <span
            data-minimized={!!apps[i].minimized}
            title={shortcut.name}
            on:click={() => unminimizeApp(i)}
            on:keydown={() => {}}
          >
            {shortcut.name}
          </span>
        {/if}
      {/each}
    </div>
  </div>

</main>

<style>
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
    border-right: 1px solid green;
  }

  .app-list-section > h2 {
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

  .app-list > div {
    width: 100%;
    margin: 0.5em 0 0.5em 0;
    text-align: left;
    cursor: pointer;
  }

  .app-list > div > span {
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
    border-bottom: 1px solid green;
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

  .start-menu[aria-expanded="true"] > button {
    width: 100%;
  }

  .start-menu[aria-expanded="true"] > button:hover {
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
    /* z-index: 100; */
    isolation: isolate;
    pointer-events: none;
    background-repeat: no-repeat;
    background-position: center;
  }

  .toolbar {
    /* width: 100%; */
    display: flex;
    flex-direction: row;
    min-height: min-content;
    border-top: 1px solid green;
  }

  .toolbar > span {
    position: relative;
    display: flex;
    justify-content: center;
    align-items: center;
    padding: 0 2em 0 2em;
    border: 1px solid green;

    background-color: var(--color-accent);
    color: var(--color-bg);
    font-weight: bold;
  }

  .toolbar > span[data-minimized="true"] {
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
    z-index: 10000;
  }
</style>
