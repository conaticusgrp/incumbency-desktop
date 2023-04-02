<script lang="ts" context="module">
    export type Severity = "normal" | "warning" | "error";

    export interface NotificationData {
        app?: string;
        header?: string;
        content?: string;
        date?: string;
        severity?: Severity;
        action?: () => void;
        actionTitle?: string;
        shown: boolean;
    }

    export const severityColors = new Map<Severity, string>()
        .set("normal", "var(--color-accent)")
        .set("warning", "#D47A21")
        .set("error", "#C82525");
</script>

<script lang="ts">
    import {
        NOTIFICATION_WIDTH,
        NOTIFICATION_HEIGHT,
        NOTIFICATION_MARGIN_Y,
        NOTIFICATION_MARGIN_X,
    } from "../../../scripts/desktopConstants";

    export let data: NotificationData;
    export let onDismissed: Function;

    export let actionTitle: string = "";
    export let actionFunction: any = () => {};

    export let shown = false;
    export let sectionExpanded = false;

    if (!sectionExpanded) {
        setTimeout(() => {
            shown = false;
        }, 5000);
    }
</script>

{#if (shown && !sectionExpanded) || sectionExpanded}
    <main
        style="
    --notification-color: {severityColors.get(data.severity ?? 'normal')};
    width: {NOTIFICATION_WIDTH}; height: {NOTIFICATION_HEIGHT};
    margin: {NOTIFICATION_MARGIN_Y} 0 {NOTIFICATION_MARGIN_X} 0;
  "
    >
        <div class="header">
            <button on:click={() => onDismissed()}>Dismiss</button>

            <h2>{data.app ?? ""}</h2>

            <span>{data.date && sectionExpanded ? data.date : "now"}</span>
        </div>

        <div class="content">
            <h3>{data.header ?? "Notification"}</h3>

            <p>{data.content ?? ""}</p>
        </div>

        {#if actionTitle}
            <div class="actions">
                <button on:click={actionFunction}>{actionTitle}</button>
            </div>
        {/if}
    </main>
{/if}

<style>
    main {
        display: flex;
        flex-direction: column;
        border: 1px solid var(--notification-color);
        background-color: black;
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
        cursor: pointer;
    }

    .header > h2 {
        width: 100%;
        padding-left: 0.5em;
        font-size: 14px;
        font-weight: bold;
        text-align: left;
        text-transform: uppercase;
    }

    .header > span {
        white-space: nowrap;
        text-overflow: ellipsis;
    }

    .content {
        height: 100%;
        padding: 0 1em 0 1em;
        text-align: left;
    }

    .content > h3 {
        margin-top: 0.5em;
        font-size: 15px;
    }

    .content > p {
        color: var(--color-shaded);
        font-size: 13px;
    }

    .actions {
        border-top: 1px solid var(--notification-color);
    }

    .actions > button {
        padding: 0.5em;
        width: 100%;
    }

    button:hover {
        color: black;
        background-color: var(--notification-color);
    }
</style>
