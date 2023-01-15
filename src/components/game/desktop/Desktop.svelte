<script lang="ts">

  import { listen } from "@tauri-apps/api/event";
  import type { DesktopAppShortcut } from "../../../scripts/desktopApp";
  import type { NotificationData } from "../../../scripts/notificationData";
  import { DEFAULT_NOTIFICATION_DISPLAY_TIME, KEEP_NOTIFICATIONS_DISPLAYED, NOTIFICATIONS_WINDOW_HEIGHT, NOTIFICATIONS_WINDOW_WIDTH } from "../../../scripts/desktopConstants";
  import DesktopShortcut from "./DesktopShortcut.svelte"
  import ToolBarItem from "./ToolBarItem.svelte";
  import Notification from "./Notification.svelte";

  import Email from "../windows/Email.svelte";
  import BudgetPanel from "../windows/BudgetPanel.svelte";
  
  // DEBUG
  import { onMount } from 'svelte'
  import DebugApp from "../windows/DebugApp.svelte";

  let date: string = "undefined date";
  let windowContainer: HTMLElement;
  let toolbarHeightPercent: number = 7;
  let desktopPaddingRem = 2;
  let wallpaperPath: string | null = null;
  let apps: DesktopAppShortcut[] = [
    {
      name: "DEBUG",
      iconPath: ""
    },
    {
      name: "Email",
      iconPath: "https://seeklogo.com/images/M/mail-icon-logo-28FE0635D0-seeklogo.com.png",
      badgeCount: 2
    },
    {
      name: "Government Spending",
      iconPath: "https://cdn-icons-png.flaticon.com/512/217/217853.png",
      badgeCount: 1
    }
  ];
  
  let notifications: NotificationData[] = [];

  const getWindow = (index: number): HTMLElement => {
    console.assert(windowContainer != undefined && index >= 0 && index < windowContainer.children.length, "Tried to get a window that doesn't exist");
    return windowContainer.children[index] as HTMLElement;
  }

  $: if (notifications.length > 0 && !KEEP_NOTIFICATIONS_DISPLAYED) {
    setTimeout(() => {
      notifications.splice(0, 1);
      notifications = notifications; // trigger render
    }, (notifications[0].displayTime || DEFAULT_NOTIFICATION_DISPLAY_TIME) * 1_000);
  }

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
    // date = ;
    console.log(d);
  });

  listen('<DEBUG_EVENT>', (e) => {
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
  
  <div class="content" style="padding: {desktopPaddingRem}rem; height: calc({100 - toolbarHeightPercent}% - {desktopPaddingRem * 2}rem);">

    {#each apps as shortcut, i}

    {#if shortcut.name !== "DEBUG"}

    <DesktopShortcut
      name={shortcut.name}
      icon={shortcut.iconPath}
      badgeCount={shortcut.badgeCount}
      gridRow={`${i} / ${i + 1}`}
      index={i}
      on:openApp={handleOpenApp}
    />

    {/if}

    {/each}

    <div class="windows" bind:this={windowContainer}>
      <DebugApp iconPath={apps[0].iconPath} on:windowClose={updateUI} on:windowMinimizeStateChange={updateUI} />
      <Email iconPath={apps[1].iconPath} on:windowClose={updateUI} on:windowMinimizeStateChange={updateUI} />
      <BudgetPanel iconPath={apps[2].iconPath} on:windowClose={updateUI} on:windowMinimizeStateChange={updateUI} />
    </div>

    <div class="notifications" style="width: {NOTIFICATIONS_WINDOW_WIDTH}px; height: {NOTIFICATIONS_WINDOW_HEIGHT}px;">
      
      {#if notifications.length > 0}

      <Notification
        iconPath={notifications[0].iconPath}
        header={notifications[0].header}
        content={notifications[0].content}
      />

      {/if}

    </div>

  </div>

  <div class="toolbar" style="height: {toolbarHeightPercent}%;">

    <div class="items">

      <ToolBarItem name="logo" iconPath={undefined} />
      <ToolBarItem name="Search" iconPath={undefined} />

      {#each apps as shortcut, i}

      {#if windowContainer != null && windowContainer.children.length > i && apps[i].name !== "DEBUG" &&
          ((windowContainer.children[i].style.display != 'none' && windowContainer.children[i].dataset['minimized'] == 'false') ||
          (windowContainer.children[i].style.display == 'none' && windowContainer.children[i].dataset['minimized'] == 'true'))}

      <!-- !! to cast (boolean | undefined) to boolean -->
      <!-- empty on:keydown to supress a warning -->
      <span
        data-minimized={!!apps[i].minimized}
        style="background-image: url('{shortcut.iconPath}');"
        title={shortcut.name}
        on:click={() => unminimizeApp(i)}
        on:keydown={() => {}}
      >
        {#if shortcut.iconPath == null}
        {shortcut.iconPath} {shortcut.name}
        {/if}
      </span>

      {/if}

      {/each}
    </div>

    <div class="date-time">
      {date}
    </div>

  </div>

</main>

<style>

  main {
    position: relative;
    width: 100%;
    height: 100%;
    background-repeat: no-repeat;
    background-position: center;
  }

  .content {
    position: relative;
    display: grid;
    grid-template-columns: repeat(16, 1fr);
    grid-template-rows: repeat(7, 1fr);
    background-color: #171717;
  }

  .toolbar {
    position: absolute;
    bottom: 0;
    width: 100%;
    display: flex;
    flex-direction: row;
    justify-content: space-between;
    background-color: #A0A0A0;
  }
  
  .toolbar > .items {
    display: flex;
    flex-direction: row;
    align-items: center;
  }
  
  .toolbar > .items > span {
    position: relative;
    width: 50px;
    height: 50px;
    margin: 0.5rem;
    background-position: center;
    background-size: cover;
    background-repeat: no-repeat;
  }

  .toolbar > .items > span[data-minimized="true"]:after {
    content: '';
    position: absolute;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    background-color: rgba(0, 0, 0, 0.5);
  }

  .toolbar > .date-time {
    width: 150px;
    margin: 0.5em;
    box-sizing: border-box;
    text-align: center;
    border: 1px solid black;
    color: black;
  }

  .windows {
    position: absolute;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    z-index: 100;
    isolation: isolate;
    pointer-events: none;
  }

  .notifications {
    position: absolute;
    top: 0;
    right: 0;
    /* background-color: #4A4A4A; */
    z-index: 200;
    pointer-events: none;
  }

</style>
