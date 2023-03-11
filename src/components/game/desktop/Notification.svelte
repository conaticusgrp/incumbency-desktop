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
    export let justDisplayed: boolean = false; // Dictates whether or not the notification will fade out

    export let actionTitle: string = "";
    export let actionFunction: any = () => {};

    let shown = true;
    let dismissClass: any = null;
</script>

{#if shown}
    <main
        style="
    --notification-color: {severityColors.get(data.severity ?? 'normal')};
    width: {NOTIFICATION_WIDTH}; height: {NOTIFICATION_HEIGHT};
    margin: {NOTIFICATION_MARGIN_Y} 0 {NOTIFICATION_MARGIN_X} 0;
  "
        class={`${dismissClass ? dismissClass : ""} ${
            justDisplayed && !dismissClass ? "new" : ""
        }`}
    >
        <div class="header">
            <button
                on:click={() => {
                    dismissClass = "dismiss";

                    setTimeout(() => {
                        shown = false;
                        onDismissed();
                    }, 200);
                }}>Dismiss</button
            >

            <h2>{data.app ?? ""}</h2>

            <span>{data.date && !justDisplayed ? data.date : "now"}</span>
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

    .new {
        animation-name: fadeout, popup;
        animation-delay: 5s, 0s;
        animation-duration: 5s, 0.5s;
        animation-fill-mode: forwards;
        animation-timing-function: none, ease-out;
    }

    .dismiss {
        animation-name: dismissed;
        animation-fill-mode: forwards;
        animation-timing-function: ease-out;
        animation-duration: 0.2s;
        animation-delay: 0s;
    }

    @keyframes dismissed {
        from {
            scale: 1;
        }
        to {
            scale: 0;
        }
    }

    @keyframes popup {
        from {
            scale: 0;
        }
        to {
            scale: 1;
        }
    }

    @keyframes fadeout {
        from {
            opacity: 1;
        }
        to {
            opacity: 0;
        }
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
        text-overflow: ellipsis;
        overflow: hidden;
        white-space: nowrap;
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
        background-color: var(--color-accent);
    }
</style>
