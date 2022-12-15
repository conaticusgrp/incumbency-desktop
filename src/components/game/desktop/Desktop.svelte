<script lang="ts">

  import type { DesktopAppShortcut } from "../../../scripts/desktopApp";
  import DesktopShortcut from "./DesktopShortcut.svelte"
  import ToolBarItem from "./ToolBarItem.svelte";
  import { NOTIFICATIONS_WINDOW_HEIGHT, NOTIFICATIONS_WINDOW_WIDTH } from "../../../scripts/desktopConstants";
  
  // DEBUG
  import TestWindow from "../windows/TestWindow.svelte";
  
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

    <div class="windows">
      <!-- TODO: add opened windows -->
      <TestWindow />
    </div>

    <div class="notifications" style="width: {NOTIFICATIONS_WINDOW_WIDTH}px; height: {NOTIFICATIONS_WINDOW_HEIGHT}px;">
      <!-- TODO: display notifications -->
    </div>

  </div>

  <div class="toolbar" style="height: {toolbarHeightPercent}%;">

    <ToolBarItem name="logo" iconPath={undefined} />
    <ToolBarItem name="Search" iconPath={undefined}/>

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
    width: 100%;
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

  /* HELP: div.windows' height is equal to the window's height, not to the viewport's height */
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
    background-color: #4A4A4A;
    z-index: 200;
  }

</style>
