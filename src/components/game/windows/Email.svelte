<script lang="ts">
  
  import { listen } from "@tauri-apps/api/event";
  import { createEventDispatcher } from "svelte";
  import type { EmailData } from "../../../scripts/email";
  import Window from "./Window.svelte"

  export let opened: boolean;
  export let focused: boolean;

  const EMAIL_LIST_WIDTH = 35;        // %
  const EMAIL_LIST_MIN_WIDTH = 250;   // px
  const EMAIL_MARGIN = 0.5;           // em
  const EMAIL_HEIGHT = 6;             // em
  const USERNAME_HEIGHT = 3.5;        // em
  const EMAIL_HEADER_HEIGHT = 3.5;      // em

  const USERNAME = "Joe";

  let dispatcher = createEventDispatcher();
  let currentDate: string;
  let lastCheckedDate: string = "undefined";
  let selectedEmailIndex: number | null = null;

  // DEBUG
  let emails: EmailData[] = [
    generateEmail({ title: "Test 1", content: "Lorem ipsum", sender: "system" }),
    generateEmail({ title: "Test 2", content: "Lorem ipsum", sender: "system" }),
    generateEmail({ title: "Test 3", content: "Lorem ipsum", sender: "system" }),
    generateEmail({ title: "Test 4", content: "Lorem ipsum", sender: "system" }),
    generateEmail({ title: "Test 5", content: "Lorem ipsum", sender: "system" }),
    generateEmail({ title: "Test 6", content: "Lorem ipsum", sender: "system" }),
  ];

  $: if (opened) {
    lastCheckedDate = currentDate;
  }

  const selectEmail = (i: number): void => {
    if (i < 0 || i >= emails.length) return;
    selectedEmailIndex = i;
  }

  // DEBUG
  function generateEmail(data: { title: string, content: string, sender: string }): EmailData {
    return {
      date: currentDate,
      title: data.title,
      content: data.content,
      sender: { username: data.sender }
    };
  }

  listen('new_day', (d) => {
    //@ts-ignore
    currentDate = d.payload.date as string;
  });

</script>

<Window
  title="Email (last checked {lastCheckedDate})"
  pos={{ x: 100, y: 50 }}
  size={{ width: 800, height: 600 }}
  {opened}
  {focused}
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
        style="height: {EMAIL_HEADER_HEIGHT}em;"
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

        {/if}
        
      </div>

      <div
        class="email-content"
        style="margin-top: {EMAIL_HEADER_HEIGHT + 1}em;"
      >

        {#if selectedEmailIndex != null && selectedEmailIndex >= 0 && selectedEmailIndex < emails.length}

        <h2>{emails[selectedEmailIndex].title}</h2>
        <section>{emails[selectedEmailIndex].content}</section>
        <p>{emails[selectedEmailIndex].sender?.username ?? ''}</p>

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

  
</style>
