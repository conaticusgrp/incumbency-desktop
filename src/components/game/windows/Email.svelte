<script lang="ts">
  
  import { createEventDispatcher } from "svelte";
  import Window from "./Window.svelte"

  export let iconPath: string | undefined = undefined;

  const EMAIL_LIST_WIDTH = 25; // %
  const EMAIL_LIST_MIN_WIDTH = 200; // px
  const EMAIL_MARGIN = 1; // em
  const EMAIL_HEIGHT = 100; // px

  let dispatcher = createEventDispatcher();

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

</script>

<Window
  title="Email"
  {iconPath}
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

      {#each emails as email}

      <div
        class="email-list-entry"
        style="
          margin: {EMAIL_MARGIN}em;
          height: {EMAIL_HEIGHT}px;
        "
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
    overflow-y: scroll;
  }

  .email-list-entry {
    text-align: left;
    text-overflow: ellipsis;
    color: black;
    background-color: gainsboro;
  }

  .email-content{
  }
  
</style>
