<script lang="ts" context="module">

  export type Severity = 'normal' | 'warning' | 'error';

  export interface NotificationData {
    app?: string,
    header?: string,
    content?: string,
    date?: string,
    severity?: Severity,
    action?: () => void
  }

  export const severityColors = new Map<Severity, string>()
    .set('normal', 'var(--color-accent)')
    .set('warning', '#D47A21')
    .set('error', '#C82525');

</script>

<script lang="ts">
  
  import { NOTIFICATION_WIDTH, NOTIFICATION_HEIGHT, NOTIFICATION_MARGIN_Y, NOTIFICATION_MARGIN_X } from "../../../scripts/desktopConstants";

  export let data: NotificationData;

</script>

<main
  style="
    --notification-color: {severityColors.get(data.severity ?? 'normal')};
    width: {NOTIFICATION_WIDTH}; height: {NOTIFICATION_HEIGHT};
    margin: {NOTIFICATION_MARGIN_Y} 0 {NOTIFICATION_MARGIN_X} 0;
  "
>

  <div class="header">
    <button>Dismiss</button>

    <h2>{data.app ?? ""}</h2>

    <span>{data.date ?? ""}</span>
  </div>

  <div class="content">

    <h3>{data.header ?? "Notification"}</h3>

    <p>{data.content ?? ""}</p>

  </div>

  <div class="actions">
    <button>action</button>
  </div>

</main>

<style>

  main {
    display: flex;
    flex-direction: column;
    border: 1px solid var(--notification-color);
  }

  .header {
    display: flex;
    align-items: center;
    padding-right: 1em;
    background-color: var(--notification-color);
    border-bottom: 1px solid var(--notification-color);
    color: var(--color-bg);
    font-size: 12px;
  }
  
  .header > button {
    background-color: var(--color-bg);
    color: var(--color-highlight);
    font-size: 10px;
  }
  
  .header > h2 {
    width: 100%;
    padding-left: 0.5em;
    font-size: 14px;
    font-weight: bold;
    text-align: left;
    text-transform: uppercase;
  }

  .content {
    height: 100%;
    padding: 0 1em 0 1em;
    text-align: left;
  }

  .content > h3 {
    margin-top: 0.5em;
    font-size: 12px;
  }

  .content > p {
    color: var(--color-shaded);
    font-size: 10px;
  }

  .actions {
    border-top: 1px solid var(--notification-color);
  }
  
  .actions > button {
    padding: 0.5em;
  }
  
</style>