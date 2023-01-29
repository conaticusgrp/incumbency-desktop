<script lang="ts">
  
  import { listen } from "@tauri-apps/api/event";
  import { createEventDispatcher, tick } from "svelte";
  import type { EmailData } from "../../../scripts/email";
  import Window from "./Window.svelte"
  import { countLines, getLineHeight } from "../../../scripts/text";
  import { WINDOW_MAXIMIZE, WINDOW_OPENED, WINDOW_RESIZE } from "../../../scripts/windowEvent";

  export let opened: boolean;
  export let focused: boolean;

  const EMAIL_LIST_WIDTH = 35;        // %
  const EMAIL_LIST_MIN_WIDTH = 250;   // px
  const EMAIL_MARGIN = 0.5;           // em
  const EMAIL_HEIGHT = 6;             // em
  const USERNAME_HEIGHT = 3.5;        // em

  const USERNAME = "Joe";

  let dispatcher = createEventDispatcher();
  let emailHeader: HTMLElement;
  let emailHeaderHeight: number;
  let emailContent: HTMLElement;
  let emailContentSection: HTMLElement;
  let currentDate: string;
  let lastCheckedDate: string = "undefined";
  let contentScrollY: number = 0;
  let selectedEmailIndex: number | null = null;
  let selectedEmailTotalLines: number = 0;
  let selectedEmailTopmostLine: number = 0;
  let selectedEmailScrollPercentage: number = 0;

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

  const selectEmail = async (i: number) => {
    if (i < 0 || i >= emails.length) return;
    selectedEmailIndex = i;

    await tick();
    selectedEmailTotalLines = countLines(emailContentSection);
  }

  const handleContentScroll = (): void => {
    contentScrollY = emailContent?.scrollTop ?? 0;
    selectedEmailTopmostLine = Math.max(
      Math.min(
        Math.floor(contentScrollY / getLineHeight(emailContent)),
        selectedEmailTotalLines
      ),
      0
    );
    selectedEmailScrollPercentage = Math.floor(selectedEmailTopmostLine / selectedEmailTotalLines * 100);
  }

  const handleWindowEvents = async (e: CustomEvent) => {
    switch (e.detail.type) {
      case WINDOW_MAXIMIZE:
      case WINDOW_RESIZE:
        if (selectedEmailIndex != null) selectEmail(selectedEmailIndex);
        break;

      case WINDOW_OPENED:
        await tick();
        lastCheckedDate = currentDate;
        emailHeaderHeight = emailHeader?.clientHeight ?? 0;
        break;

      default:
        break;
    }
  }

  listen('new_day', (d) => {
    //@ts-ignore
    currentDate = d.payload.date as string;
  });

  // DEBUG
  function generateEmail(data: { title: string, content: string, sender: string }): EmailData {
    return {
      date: currentDate,
      title: data.title,
      content: data.content,
      sender: { username: data.sender }
    };
  }

</script>

<Window
  title="Email (last checked {lastCheckedDate})"
  pos={{ x: 100, y: 50 }}
  size={{ width: 800, height: 600 }}
  {opened}
  {focused}
  on:windowEvent={handleWindowEvents}
  on:criticalWindowEvent={(e) => dispatcher('criticalWindowEvent', e.detail)}
>
  <main class="content">

    <div
      class="left-section"
      style="
        width: {EMAIL_LIST_WIDTH}%;
        min-width: {EMAIL_LIST_MIN_WIDTH}px;
      "
    >

      <div
        class="email-list"
        style="
          height: calc(100% - {USERNAME_HEIGHT}em);
        "
      >

        {#each emails as email, i}

        <!-- on:keydown to supress a warning -->
        <div
          class="email-list-entry"
          style="
            margin: {EMAIL_MARGIN}em;
            height: {EMAIL_HEIGHT}em;
          "
          data-selected={i === selectedEmailIndex}
          on:click={() => selectEmail(i)}
          on:keydown={() => {}}
        >
            <h2>{email.sender?.username ?? "Unknown sender"}</h2>
            <h3>{email.title}</h3>
            <p>{email.content}</p>
        </div>

        {/each}

      </div>

      <div
        class="username"
        style="height: {USERNAME_HEIGHT}em;"
      >
        Authenticated as:
        <div>{USERNAME}</div>
      </div>

    </div>

    <div
      class="right-section"
      style="width: calc(100% - max({EMAIL_LIST_MIN_WIDTH}px, {EMAIL_LIST_WIDTH}%));"
    >

      <div
        class="email-header"
        bind:this={emailHeader}
      >
        
        {#if selectedEmailIndex != null && selectedEmailIndex >= 0 && selectedEmailIndex < emails.length}

        <div>
          From: <span>{emails[selectedEmailIndex].sender?.username ?? "Unknown sender"}</span>
          &lt;{emails[selectedEmailIndex].sender?.address ?? "unknown"}&gt;
        </div>

        <div>
          CC: <span>{emails[selectedEmailIndex].cc?.username ?? "Unknown user"}</span>
          &lt;{emails[selectedEmailIndex].cc?.address ?? "unknown"}&gt;
        </div>

        {:else}

        <!-- Space fillers (to compute the height) -->
        <div><pre> </pre></div>
        <div><pre> </pre></div>

        {/if}
        
      </div>

      <div
        class="email-content"
        style="
          margin-top: calc(2em + {emailHeaderHeight}px);
          height: calc(100% - 4em - {emailHeaderHeight}px);
        "
        bind:this={emailContent}
        on:scroll={handleContentScroll}
      >

        {#if selectedEmailIndex != null && selectedEmailIndex >= 0 && selectedEmailIndex < emails.length}

        <h2>{emails[selectedEmailIndex].title}</h2>
        <section bind:this={emailContentSection}>
          {emails[selectedEmailIndex].content}
        </section>
        <p>{emails[selectedEmailIndex].sender?.username ?? ''}</p>
        <div class="space-filler"></div>
        
        <div
          class="line-counter"
          style="bottom: {-contentScrollY - 2}px;"
        >
          - line: {selectedEmailTopmostLine} / {selectedEmailTotalLines} - {selectedEmailScrollPercentage}% -
        </div>

        {/if}

      </div>

    </div>
    
  </main>
</Window>

<style>

  main {
    display: flex;
    flex-direction: row;
    width: 100%;
    height: 100%;
  }

  .left-section {
    display: flex;
    flex-direction: column;
    height: 100%;
    border-right: 1px solid var(--color-accent);
  }

  .email-list {
    display: flex;
    flex-direction: column;
    width: 100%;
    overflow-y: scroll;
  }

  .username {
    border-top: 1px solid var(--color-accent);
  }

  .username > div {
    color: var(--color-accent);
  }

  .email-list::-webkit-scrollbar {
    display: none;
  }

  .email-list-entry {
    text-align: left;
    text-overflow: ellipsis;
    padding: 0.5em;
    color: var(--color-highlight);
    background-color: var(--color-bg);
    border: 1px solid var(--color-accent);
  }
  
  .email-list-entry[data-selected="true"] {
    color: var(--color-bg);
    background-color: var(--color-accent);
  }

  .email-list-entry > h2 {
    font-size: 14px;
    font-weight: bold;
  }

  .email-list-entry > h3 {
    font-size: 12px;
    font-weight: 500;
  }
  
  .email-list-entry > p {
    font-size: 12px;
    font-weight: 400;
    opacity: 0.35;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .right-section {
    position: relative;
    height: 100%;
  }

  .email-header {
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    width: calc(100% - 2 * 0.5em);
    border-bottom: 1px solid var(--color-accent);
    padding: 0.5em;
    color: var(--color-shaded);
    text-align: left;
    font-size: 12px;
  }

  .email-header span {
    color: var(--color-highlight);
  }

  .email-content {
    position: relative;
    margin: 2em;
    box-sizing: border-box;
    text-align: left;
    overflow-y: scroll;
  }

  .email-content::-webkit-scrollbar {
    display: none;
  }

  .email-content > h2 {
    margin-bottom: 1em;
  }

  .email-content > p {
    font-weight: bold;
    margin-top: 1em;
    text-align: right;
  }

  .email-content > .space-filler {
    height: calc(100% - 1em);
  }

  .line-counter {
    position: absolute;
    right: 0;
    bottom: 0;
    padding-left: 2px;
    width: 100%;
    height: 2em;
    text-align: right;
    font-size: 12px;
    color: var(--color-accent);
    background-color: var(--color-bg);
  }

  
</style>
