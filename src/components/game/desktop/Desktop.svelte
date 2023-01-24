<script lang="ts">

  import { listen } from "@tauri-apps/api/event";
  import type { DesktopAppShortcut } from "../../../scripts/desktopApp";
  import type { NotificationData } from "../../../scripts/notificationData";
  import { DATE_TIME_HEIGHT, TOOLBAR_HEIGHT } from "../../../scripts/desktopConstants";

  import Email from "../windows/Email.svelte";
  import BudgetPanel from "../windows/BudgetPanel.svelte";
  
  // DEBUG
  import { onMount } from 'svelte'
  import DebuggerApp from "../windows/DebuggerApp.svelte";

  let date: string = "undefined date";
  let windowContainer: HTMLElement;
  let appListWidthPixels = 300;
  let wallpaperPath: string | null = null;
  let apps: DesktopAppShortcut[] = [
    { name: "DEBUG" },
    { name: "Email", badgeCount: 2 },
    { name: "Government Spending", badgeCount: 1 }
  ];
  
  let notifications: NotificationData[] = [];

  const getWindow = (index: number): HTMLElement => {
    console.assert(windowContainer != undefined && index >= 0 && index < windowContainer.children.length, "Tried to get a window that doesn't exist");
    return windowContainer.children[index] as HTMLElement;
  }

  /*
  $: if (notifications.length > 0 && !KEEP_NOTIFICATIONS_DISPLAYED) {
    setTimeout(() => {
      notifications.splice(0, 1);
      notifications = notifications; // trigger render
    }, (notifications[0].displayTime || DEFAULT_NOTIFICATION_DISPLAY_TIME) * 1_000);
  }
  */

  const handleOpenApp = (e: CustomEvent): void => {
    const index = e.detail.index;
    if (index < 0 || index >= windowContainer.children.length) return;

    if (getWindow(index).dataset['minimized'] !== 'true') {
      getWindow(index).style.display = 'initial';
      updateUI();
    } else {
      unminimizeApp(index);
    }
  }

  const unminimizeApp = (index: number) => {
    getWindow(index).dataset['minimized'] = 'false';
    updateUI();
  }

  const updateUI = () => {
    // console.log("update");
    // console.assert(apps.length <= windowContainer.children.length);
    for (let i = 0; i < windowContainer.children.length; i++) {
      apps[i].minimized = getWindow(i).dataset['minimized'] === 'true';
    }

    apps = apps;
  }

  listen('new_day', (d) => {
    //@ts-ignore
    date = d.payload.date as string;
  });

  listen('open_debugger_app', (e) => {
    //@ts-ignore
    handleOpenApp({ detail: { index: apps.findIndex((v) => v.name === "DEBUG") } });
  });

  // DEBUG
  onMount(() => {
    setTimeout(() => {
      notifications = [
        {
          header: "Test notification",
          content: "This is the first notification to ever pop up! Isn't it exciting? You should be happy!",
          iconPath: "https://encrypted-tbn0.gstatic.com/images?q=tbn:ANd9GcQ-tf1a7Cwsujdb5k5YKQUP18mR_7dDJd5Bj9CGQv3CaQ&s"
        }
      ];
    }, 2000);
  });

</script>

<main style="background-image: {(wallpaperPath != null) ? `url(${wallpaperPath})` : "none"};">

  <div
    class="app-list-section"
    style="width: {appListWidthPixels}px;"
  >

    <h2>Installed Software</h2>

    <div class="app-list">

      {#each apps as shortcut, i}

      {@const appExists = windowContainer != null && windowContainer.children.length > i}
      {@const appOpened = appExists &&
                          ((windowContainer.children[i].style.display != 'none' &&
                          windowContainer.children[i].dataset['minimized'] == 'false') ||
                          (windowContainer.children[i].style.display == 'none' &&
                          windowContainer.children[i].dataset['minimized'] == 'true'))}
      <div style="color: {appOpened ? 'grey' : 'white'};">{shortcut.name}</div>

      {/each}

    </div>

  </div>

  <div
    class="content"
    style="background-image: url({wallpaperPath ?? ''}); width: calc(100% - {appListWidthPixels}px);"
  >

    <div
      class="date-time"
      style="height: {DATE_TIME_HEIGHT}%;"
    >
      {date}
    </div>

    <div
      class="windows"
      style="height: {100 - DATE_TIME_HEIGHT - TOOLBAR_HEIGHT}%"
      bind:this={windowContainer}
    >

      <DebuggerApp on:criticalWindowEvent={updateUI} />
      <Email on:criticalWindowEvent={updateUI} />
      <BudgetPanel on:criticalWindowEvent={updateUI} />

    </div>

    <div
      class="toolbar"
      style="height: {TOOLBAR_HEIGHT}%;"
    >

      {#each apps as shortcut, i}

      {#if windowContainer != null && windowContainer.children.length > i && (apps[i].name !== "DEBUG" || true)}

      {@const appShown = windowContainer.children[i].style.display != 'none'}
      {@const appMinimized = windowContainer.children[i].dataset['minimized'] == 'true'}

      {#if (appShown && !appMinimized) || (!appShown && appMinimized)}

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
    background-color: black;
    background-repeat: no-repeat;
    background-position: center;
    background-size: contain;
  }

  .app-list-section {
    display: flex;
    flex-direction: column;
    align-items: center;
    border-right: 1px solid green;
  }

  .app-list-section > h2 {
    margin: 1em 2em 2em 2em;
  }

  .app-list {
    display: flex;
    flex-direction: column;
    align-items: flex-start;
    width: calc(100% - 1em * 2);
    margin: 0 1em 0 1em;
  }

  .app-list > div {
    margin: 0.5em 0 0.5em 0;
    
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
    z-index: 100;
    isolation: isolate;
    pointer-events: none;
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
  }

  /* to be updated */

  .toolbar > .items > span[data-minimized="true"]:after {
    content: '';
    position: absolute;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    background-color: rgba(0, 0, 0, 0.5);
  }

</style>
