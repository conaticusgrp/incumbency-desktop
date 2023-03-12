<script lang="ts" context="module">
    export interface EmailUser {
        username: string;
        address?: string;
    }

    export interface EmailData {
        title: string;
        content: string;
        date: string;
        sender?: EmailUser;
        cc?: EmailUser;
    }
</script>

<script lang="ts">
    import { listen } from "@tauri-apps/api/event";
    import { createEventDispatcher } from "svelte";
    import TabWindow from "../TabWindow.svelte";
    import type { CriticalWindowData } from "../Window.svelte";

    import EmailTabButton from "./EmailTabButton.svelte";
    import EmailTab from "./EmailTab.svelte";
    import {
        EMAIL_CREATE,
        WINDOW_CLOSE,
        WINDOW_OPENED,
        WINDOW_RESIZE,
        WINDOW_SEND_NOTIFICATION,
    } from "../../../../scripts/windowEvent";
    import type { Severity } from "../../desktop/Notification.svelte";

    export let windowData: CriticalWindowData;

    let dispatcher = createEventDispatcher();

    let currentDate: string;
    let lastCheckedDate: string = "none";
    let currentTab: number;

    let emails: EmailData[] = [];

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
            case EMAIL_CREATE:
                appendEmail(e.detail.data);
                break;
            default:
                break;
        }
    };

    const handleCriticalWindowEvent = (e: CustomEvent): void => {
        switch (e.detail.type) {
            case WINDOW_CLOSE:
                {
                    currentTab = -1;
                }
                break;

            default:
                break;
        }

        dispatcher("criticalWindowEvent", e.detail); // !!!
    };

    listen("new_day", (d) => {
        //@ts-ignore
        currentDate = d.payload.date as string;
    });

    let emailsMapped: any[] = [];

    // DEBUG
    function appendEmail(data: {
        title: string;
        content: string;
        sender: string;
        severity?: Severity;
    }) {
        if (!data.severity) {
            data.severity = "normal";
        }

        const email = {
            date: "now",
            title: data.title,
            content: data.content,
            sender: { username: data.sender },
        };

        emails.unshift(email);
        emails = [...emails]; // Need to reassign for bind

        emailsMapped = emails.map((e) => {
            return { c: EmailTab, data: e };
        });

        dispatcher("criticalWindowEvent", {
            type: WINDOW_SEND_NOTIFICATION,
            data: {
                app: "Email",
                header: `New Email from ${data.sender}`,
                content: data.content,
                severity: data.severity,
                actionTitle: "Open App",
            },
        });
    }
</script>

<TabWindow
    title="Email (last checked {lastCheckedDate})"
    pos={{ x: 100, y: 50 }}
    size={{ width: 800, height: 600 }}
    tabButtonComponent={EmailTabButton}
    bind:tabButtonData={emails}
    bind:tabs={emailsMapped}
    bind:currentTabIndex={currentTab}
    on:windowEvent={handleWindowEvent}
    on:criticalWindowEvent={handleCriticalWindowEvent}
    {windowData}
/>
