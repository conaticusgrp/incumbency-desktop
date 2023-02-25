<script lang="ts" context="module">
  export interface EmailUser {
      username: string,
      address?: string
  }
  
  export interface EmailData {
      title: string,
      content: string,
      date: string,
      sender?: EmailUser,
      cc?: EmailUser
  }
</script>

<script lang="ts">
  
  import { listen } from "@tauri-apps/api/event";
  import { createEventDispatcher } from "svelte";
  import TabWindow from "../TabWindow.svelte";
  import type { CriticalWindowData } from "../Window.svelte"
    
  import EmailTabButton from "./EmailTabButton.svelte";
  import EmailTab from "./EmailTab.svelte";
  import { WINDOW_CLOSE, WINDOW_OPENED, WINDOW_RESIZE } from "../../../../scripts/windowEvent";
  
  export let windowData: CriticalWindowData;
  
  let dispatcher = createEventDispatcher();

  let currentDate: string;
  let lastCheckedDate: string = "undefined";
  let currentTab: number;

  // NOTE: when emails are added to the array, the selectedEmailIndex should be changed as well!

  // DEBUG
  let l = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Morbi tincidunt augue interdum velit euismod in pellentesque. Porttitor eget dolor morbi non. Diam sit amet nisl suscipit adipiscing. In cursus turpis massa tincidunt dui ut ornare. Vitae nunc sed velit dignissim sodales ut eu sem. Odio morbi quis commodo odio aenean sed adipiscing diam donec. Tempor nec feugiat nisl pretium. Sed turpis tincidunt id aliquet risus feugiat. Mattis molestie a iaculis at erat. Amet consectetur adipiscing elit pellentesque habitant morbi tristique senectus et. Mi bibendum neque egestas congue quisque egestas diam in arcu. Vestibulum mattis ullamcorper velit sed ullamcorper morbi tincidunt ornare massa. Aliquam ultrices sagittis orci a. Id ornare arcu odio ut. Vel risus commodo viverra maecenas accumsan lacus. Nulla posuere sollicitudin aliquam ultrices sagittis. Vitae et leo duis ut. In massa tempor nec feugiat nisl pretium. Diam phasellus vestibulum lorem sed risus. Scelerisque purus semper eget duis at tellus at urna. Lorem ipsum dolor sit amet consectetur adipiscing elit duis tristique. Rutrum quisque non tellus orci ac auctor. Eros donec ac odio tempor orci dapibus. Augue lacus viverra vitae congue eu consequat ac felis. Urna porttitor rhoncus dolor purus non. Duis at consectetur lorem donec massa sapien. Eget dolor morbi non arcu risus quis varius quam. Sed blandit libero volutpat sed cras. Quisque id diam vel quam elementum pulvinar etiam non. Odio pellentesque diam volutpat commodo sed egestas egestas fringilla phasellus. Et molestie ac feugiat sed lectus. Lacus sed viverra tellus in hac habitasse platea dictumst vestibulum. Cras adipiscing enim eu turpis egestas pretium. Sed euismod nisi porta lorem mollis aliquam ut. Dignissim enim sit amet venenatis urna cursus eget nunc scelerisque. Mauris in aliquam sem fringilla ut morbi. Accumsan in nisl nisi scelerisque eu ultrices. In fermentum posuere urna nec tincidunt praesent. Ultricies leo integer malesuada nunc vel risus commodo viverra. Volutpat sed cras ornare arcu dui. Nec feugiat in fermentum posuere urna. Libero volutpat sed cras ornare arcu dui vivamus. Metus aliquam eleifend mi in nulla posuere sollicitudin aliquam ultrices. Sagittis aliquam malesuada bibendum arcu vitae elementum curabitur. Tincidunt id aliquet risus feugiat in. Tincidunt dui ut ornare lectus sit amet est placerat. Commodo nulla facilisi nullam vehicula ipsum a. Hendrerit dolor magna eget est. Tortor consequat id porta nibh. Magna sit amet purus gravida. Vestibulum morbi blandit cursus risus at. Egestas congue quisque egestas diam in arcu cursus euismod quis. Convallis convallis tellus id interdum velit laoreet id. At augue eget arcu dictum varius duis at consectetur. Ultricies lacus sed turpis tincidunt id. Vulputate ut pharetra sit amet. Nulla porttitor massa id neque aliquam vestibulum morbi. Morbi tincidunt ornare massa eget. Quam pellentesque nec nam aliquam. Tortor dignissim convallis aenean et tortor at. Nulla facilisi etiam dignissim diam quis enim lobortis. Iaculis at erat pellentesque adipiscing commodo elit at. A cras semper auctor neque vitae. Ornare arcu odio ut sem nulla. Risus quis varius quam quisque id diam vel. Rutrum tellus pellentesque eu tincidunt. Magna ac placerat vestibulum lectus mauris ultrices eros in. Ut placerat orci nulla pellentesque dignissim enim sit. Velit egestas dui id ornare arcu odio ut sem. Consectetur lorem donec massa sapien faucibus et molestie ac. Mattis vulputate enim nulla aliquet porttitor. Justo eget magna fermentum iaculis eu non. Viverra nibh cras pulvinar mattis nunc sed. Tellus mauris a diam maecenas sed enim ut sem viverra. Nunc vel risus commodo viverra maecenas accumsan lacus. Libero nunc consequat interdum varius sit amet mattis vulputate enim. Mattis rhoncus urna neque viverra justo nec. Dictum non consectetur a erat. Augue mauris augue neque gravida in fermentum et sollicitudin ac. Risus nullam eget felis eget nunc lobortis mattis aliquam faucibus. Malesuada fames ac turpis egestas. Consectetur lorem donec massa sapien faucibus et molestie. Ornare arcu odio ut sem nulla pharetra. Tincidunt lobortis feugiat vivamus at augue eget arcu. Sed risus ultricies tristique nulla.";

  // DEBUG
  let emails: EmailData[] = [
    generateEmail({ title: "Long", content: l, sender: "system" }),
    generateEmail({ title: "Test 1", content: "Lorem ipsum", sender: "system" }),
    generateEmail({ title: "Test 2", content: "Lorem ipsum", sender: "system" }),
    generateEmail({ title: "Test 3", content: "Lorem ipsum", sender: "system" }),
    generateEmail({ title: "Test 4", content: "Lorem ipsum", sender: "system" }),
    generateEmail({ title: "Test 5", content: "Lorem ipsum", sender: "system" }),
    generateEmail({ title: "Test 6", content: "Lorem ipsum", sender: "system" }),
  ];

  const handleWindowEvent = (e: CustomEvent): void => {
    switch (e.detail.type) {
      case WINDOW_OPENED:
        {
          lastCheckedDate = currentDate;
        }
        break;

      case WINDOW_RESIZE:
        {
          emails = emails;
        }
        break;

      default: break;
    }
  }

  const handleCriticalWindowEvent = (e: CustomEvent): void => {
    switch (e.detail.type) {
      case WINDOW_CLOSE:
        {
          currentTab = -1;
          console.log("opened");
        }
        break;

      default: break;
    }
    dispatcher('criticalWindowEvent', e.detail); // !!!
  }

  listen('new_day', (d) => {
    //@ts-ignore
    currentDate = d.payload.date as string;
  });

  // DEBUG
  function generateEmail(data: { title: string, content: string, sender: string }): EmailData {
    return {
      // date: currentDate,
      date: "now",
      title: data.title,
      content: data.content,
      sender: { username: data.sender }
    };
  }
  
</script>
  
<TabWindow
  title="Email (last checked {lastCheckedDate})"
  pos={{ x: 100, y: 50 }}
  size={{ width: 800, height: 600 }}
  {windowData}
  tabButtonComponent={EmailTabButton}
  tabButtonData={emails}
  tabs={
    emails.map((e) => {
      return { c: EmailTab, data: e };
    })
  }
  bind:currentTabIndex={currentTab}
  on:windowEvent={handleWindowEvent}
  on:criticalWindowEvent={handleCriticalWindowEvent}
/>