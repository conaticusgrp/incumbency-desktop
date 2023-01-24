<script lang="ts">
  
  import { createEventDispatcher } from "svelte";
  import Window from "./Window.svelte"

  const EMAIL_LIST_WIDTH = 25; // %
  const EMAIL_LIST_MIN_WIDTH = 200; // px
  const EMAIL_MARGIN = 1; // em
  const EMAIL_HEIGHT = 100; // px

  let dispatcher = createEventDispatcher();

  let selectedEmailIndex: number | null = null;

  // DEBUG
  let emails = [
    {
      title: "Test 1",
      content: "Lorem ipsum",
      sender: "system"
    },
    {
      title: "Test 2",
      content: "Lorem ipsum",
      sender: "system"
    },
    {
      title: "Test 3",
      content: "Lorem ipsum",
      sender: "system"
    },
  ];

  const selectEmail = (i: number): void => {
    if (i < 0 || i >= emails.length) return;
    selectedEmailIndex = i;
  }

</script>

<Window
  title="Email"
  pos={{ x: 100, y: 50 }}
  size={{ width: 800, height: 600 }}
  on:windowClose={() => dispatcher('windowClose')}
  on:windowMinimize={() => dispatcher('windowMinimizeStateChange')}
>
  <main class="content">

    <div
      class="email-list"
      style="
        width: {EMAIL_LIST_WIDTH}%;
        min-width: {EMAIL_LIST_MIN_WIDTH}px;
      "
    >

      {#each emails as email, i}

      <!-- on:keydown to supress a warning -->
      <div
        class="email-list-entry"
        style="
          margin: {EMAIL_MARGIN}em;
          height: {EMAIL_HEIGHT}px;
        "
        on:click={() => selectEmail(i)}
        on:keydown={() => {}}
      >
          <h3>{email.title}</h3>
          <p>{email.content}</p>
      </div>

      {/each}

    </div>

    <div
      class="email-content"
      style="width: calc(100% - {EMAIL_LIST_WIDTH}%);"
    >

      {#if selectedEmailIndex != null && selectedEmailIndex >= 0 && selectedEmailIndex < emails.length}

      <h2>{emails[selectedEmailIndex].title}</h2>
      <section>{emails[selectedEmailIndex].content}</section>
      <p>{emails[selectedEmailIndex].sender ?? ''}</p>

      {/if}

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

  .email-list {
    display: flex;
    flex-direction: column;
    border-right: 1px solid var(--color-secondary);
    overflow-y: scroll;
  }

  .email-list::-webkit-scrollbar {
    display: none;
  }

  .email-list-entry {
    text-align: left;
    text-overflow: ellipsis;
    color: black;
    background-color: gainsboro;
  }

  .email-content {
    margin: 2em;
    box-sizing: border-box;
    text-align: left;
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
