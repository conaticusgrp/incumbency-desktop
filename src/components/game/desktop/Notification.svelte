<script lang="ts">
  
  import { NOTIFICATIONS_WINDOW_HEIGHT, NOTIFICATION_HEADER_HEIGHT } from "../../../scripts/desktopConstants";

  export let iconPath: string | undefined = undefined;
  export let header: string = "!";
  export let content: string = "";

</script>

<!-- NOT TESTED -->
<main>

  <div class="inner">

    <div class="header">

      <img
        style="width: {NOTIFICATION_HEADER_HEIGHT}px; height: {NOTIFICATIONS_WINDOW_HEIGHT};"
        src={(iconPath != undefined) ? iconPath : ""}
        alt="icon"
      />

      <h3>{header}</h3>

    </div>

    <p>{content}</p>
  
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
    padding: 1rem;
    box-sizing: border-box;
  }

  .header {
    display: flex;
    flex-direction: row;
  }

  img {
    margin-right: 1rem;
  }

  /* PROBLEM: the content doesn't wrap */
  p {
    inline-size: 100%;
    width: 100%;
    overflow-wrap: break-word;
  }

</style>