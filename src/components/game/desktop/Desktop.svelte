<script lang="ts">

  import type { DesktopAppShortcut } from "../../../scripts/desktopApp";
  import type { NotificationData } from "../../../scripts/notificationData";
  import { DEFAULT_NOTIFICATION_DISPLAY_TIME, KEEP_NOTIFICATIONS_DISPLAYED, NOTIFICATIONS_WINDOW_HEIGHT, NOTIFICATIONS_WINDOW_WIDTH } from "../../../scripts/desktopConstants";
  import DesktopShortcut from "./DesktopShortcut.svelte"
  import ToolBarItem from "./ToolBarItem.svelte";
  import Notification from "./Notification.svelte";
  
  // DEBUG
  import TestWindow from "../windows/TestWindow.svelte";
  import { onMount } from 'svelte'
  
  let windowsContainer: HTMLElement;
  let toolbarHeightPercent: number = 15;
  let wallpaperPath: string | null = null;
  let apps: DesktopAppShortcut[] = [
    {
      name: "Email",
      iconPath: "https://seeklogo.com/images/M/mail-icon-logo-28FE0635D0-seeklogo.com.png",
      badgeCount: 2
    },
    {
      name: "Government Spending",
      iconPath: "https://cdn-icons-png.flaticon.com/512/217/217853.png",
      badgeCount: 1
    },
    {
      name: "Stocks",
      iconPath: "https://cdn-icons-png.flaticon.com/512/263/263142.png",
      badgeCount: 0
    },
    {
      name: "Contacts",
      iconPath: "https://encrypted-tbn0.gstatic.com/images?q=tbn:ANd9GcQhqI4txTRkj4_pCfr3NlNdbCbLYgX-nqjMX8wHEfx_A6Q8luaudlecd84nMDGZ1a4nwA0&usqp=CAU",
      badgeCount: 0
    },
    {
      name: "Add/Remove Apps",
      iconPath: "https://static.vecteezy.com/system/resources/previews/008/659/063/original/eps10-black-download-or-install-icon-in-simple-flat-trendy-modern-style-isolated-on-white-background-free-vector.jpg",
      badgeCount: 0
    }
  ];
  
  let notifications: NotificationData[] = [];

  $: if (notifications.length > 0 && !KEEP_NOTIFICATIONS_DISPLAYED) {
    setTimeout(() => {
      notifications.splice(0, 1);
      notifications = notifications; // trigger render
    }, (notifications[0].displayTime || DEFAULT_NOTIFICATION_DISPLAY_TIME) * 1_000);
  }

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
  
  <div class="content" style="height: {100 - toolbarHeightPercent}%;">

    {#each apps as shortcut, i}

    <DesktopShortcut
      name={shortcut.name}
      icon={shortcut.iconPath}
      badgeCount={shortcut.badgeCount}
      gridRow={`${i} / ${i + 1}`}
    />

    {/each}

    <div class="windows" bind:this={windowsContainer}>
      <!-- TODO: add opened windows -->
      <TestWindow parentComponent={windowsContainer} />
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

    <ToolBarItem name="logo" iconPath={undefined} />
    <ToolBarItem name="Search" iconPath={undefined} />

  </div>

</main>

<style>

  main {
    position: relative;
    height: 100%;
    width: 100%;
    background-repeat: no-repeat;
    background-position: center;
  }

  .content {
    position: relative;
    width: calc(100% - 2rem * 2);
    display: grid;
    grid-template-columns: repeat(11, 1fr);
    grid-template-rows: repeat(7, 1fr);
    padding: 2rem;
    background-color: #171717;
  }

  .toolbar {
    width: 100%;
    display: flex;
    flex-direction: row;
    background-color: #A0A0A0;
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
  }

</style>
