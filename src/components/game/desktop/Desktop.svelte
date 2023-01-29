<script lang="ts">

  import { listen } from "@tauri-apps/api/event";
  import { invoke } from "@tauri-apps/api/tauri";
  import type { DesktopAppShortcut } from "../../../scripts/desktopApp";
  //import type { NotificationData } from "../../../scripts/notificationData";
  import { APP_LIST_MIN_WIDTH, APP_LIST_WIDTH_PERCENT, DATE_TIME_HEIGHT, TOOLBAR_HEIGHT } from "../../../scripts/desktopConstants";
  import { WINDOW_AQUIRE_FOCUS, WINDOW_CLOSE, WINDOW_MINIMIZE } from "../../../scripts/windowEvent";

  import Email from "../windows/Email.svelte";
  import BudgetPanel from "../windows/BudgetPanel.svelte";
  
  // DEBUG
  import { onMount } from 'svelte'
  import DebuggerApp from "../windows/DebuggerApp.svelte";

  let date: string = "undefined date";
  let windowContainer: HTMLElement;
  let wallpaperPath: string | null = "./src/assets/Wallpaper.png";
  let apps: DesktopAppShortcut[] = [
    { componentConstructor: DebuggerApp, name: "DEBUG" },
    { componentConstructor: Email,       name: "Email", badgeCount: 2 },
    { componentConstructor: BudgetPanel, name: "Budget Panel", badgeCount: 1 }
  ];
  let focusedApp: number | null = null;
  
  //let notifications: NotificationData[] = [];

  /*
  $: if (notifications.length > 0 && !KEEP_NOTIFICATIONS_DISPLAYED) {
    setTimeout(() => {
      notifications.splice(0, 1);
      notifications = notifications; // trigger render
    }, (notifications[0].displayTime || DEFAULT_NOTIFICATION_DISPLAY_TIME) * 1_000);
  }
  */

  const handleOpenApp = (index: number): void => {
    if (index < 0 || index >= apps.length) return;

    if (!apps[index].opened) {
      apps[index].opened = true;
      focusedApp = index;
      updateUI();
    } else {
      unminimizeApp(index);
    }
  }

  const unminimizeApp = (index: number) => {
    if (index < 0 || index >= apps.length) return;
    
    apps[index].minimized = !apps[index].minimized;
    updateUI();
  }

  const updateUI = () => {
    apps = apps;
  }

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

      default: break;
    }
  }

  listen('new_day', (d) => {
    //@ts-ignore
    date = d.payload.date as string;
  });

  listen('open_debugger_app', (e) => {
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

  // DEBUG
  onMount(async () => {
    /*
    setTimeout(() => {
      notifications = [
        {
          header: "Test notification",
          content: "This is the first notification to ever pop up! Isn't it exciting? You should be happy!",
          iconPath: "https://encrypted-tbn0.gstatic.com/images?q=tbn:ANd9GcQ-tf1a7Cwsujdb5k5YKQUP18mR_7dDJd5Bj9CGQv3CaQ&s"
        }
      ];
    }, 2000);
    */

   invoke("frontend_ready");
  });

</script>

<main>

  <div
    class="app-list-section"
    style="width: {APP_LIST_WIDTH_PERCENT}%; min-width: {APP_LIST_MIN_WIDTH}px;"
  >

    <h2>Installed Software</h2>

    <div class="app-list">

      {#each apps as shortcut, i}

      <!-- empty on:keydown to supress a warning -->
      <div
        style="color: var({apps[i].opened ? '--color-highlight' : '--color-shaded'});"
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
    style="width: calc({100 - APP_LIST_WIDTH_PERCENT}%);"
  >

    <div
      class="date-time"
      style="height: {DATE_TIME_HEIGHT}em;"
    >
      {date}
    </div>

    <div
      class="windows"
      style="
        height: calc(100% - {DATE_TIME_HEIGHT}em - {TOOLBAR_HEIGHT}em);
        background-image: {(wallpaperPath != null) ? `url(${wallpaperPath})` : "none"};
      "
      bind:this={windowContainer}
    >

      {#each apps as app, i}

      <!-- !! to cast (boolean | undefined) to boolean -->

      <svelte:component
        this={app.componentConstructor}
        bind:this={app.component}
        opened={!!app.opened && !app.minimized}
        focused={i === focusedApp}
        {...app.props}
        on:criticalWindowEvent={(e) => handleCriticalEvent(i, e)}
      />

      {/each}

    </div>

    <div
      class="toolbar"
      style="height: {TOOLBAR_HEIGHT}em;"
    >

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
    font-weight: bold;
  }

  .app-list {
    display: flex;
    flex-direction: column;
    align-items: flex-start;
    width: calc(100% - 1em * 2);
    margin: 0 1em 0 1em;
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

  .date-time {
    display: flex;
    justify-content: center;
    align-items: center;
    min-height: min-content;
    border-bottom: 1px solid green;
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

</style>
