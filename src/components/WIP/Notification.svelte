<script lang="ts" context="module">

  interface NotificationData {
    iconPath?: string,
    header: string,
    content: string
  }

</script>

<script lang="ts">
  
  import { NOTIFICATIONS_WINDOW_HEIGHT, NOTIFICATION_HEADER_HEIGHT, NOTIFICATION_PADDING } from "../../scripts/desktopConstants";

  export let iconPath: string | undefined = undefined;
  export let header: string = "!";
  export let content: string = "";

</script>

<main>

  <div class="inner" style="padding: {NOTIFICATION_PADDING}px;">

    <div class="header">

      <img
        style="width: {NOTIFICATION_HEADER_HEIGHT}px; height: {NOTIFICATIONS_WINDOW_HEIGHT};"
        src={(iconPath != undefined) ? iconPath : ""}
        alt="icon"
      />

      <h3 style="width: calc(100% - {NOTIFICATION_PADDING}px * 3);" title={header}>{header}</h3>

    </div>

    <p style="width: calc(100% - {NOTIFICATION_PADDING}px * 3);">{content}</p>
  
  </div>

</main>

<style>

  main {
    width: 100%;
    height: 100%;
    margin: 0;
    background-color: #A2A2A2;
    border-bottom-left-radius: 1rem;
  }
  
  .inner {
    width: 100%;
    height: 100%;
    box-sizing: border-box;
  }

  .header {
    display: flex;
    flex-direction: row;
  }

  img {
    margin-right: 1rem;
  }

  h3 {
    text-overflow: ellipsis;
    text-align: left;
  }

  p {
    text-align: left;
    overflow-wrap: break-word;
  }

</style>