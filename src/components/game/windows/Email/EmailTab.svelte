<script lang="ts">
    import type { EmailData } from "./Email.svelte";
    import { countLines, getLineHeight } from "../../../../scripts/text";
    import { onMount, tick } from "svelte";

    export let tabData: EmailData;

    let mounted: boolean = false;
    let emailHeader: HTMLElement;
    let emailHeaderHeight: number;
    let emailContent: HTMLElement;
    let emailContentSection: HTMLElement;
    let contentScrollY: number = 0;
    let selectedEmailTotalLines: number = 0;
    let selectedEmailTopmostLine: number = 0;
    let selectedEmailScrollPercentage: number = 0;

    const handleContentScroll = (): void => {
        contentScrollY = emailContent?.scrollTop ?? 0;
        selectedEmailTopmostLine = Math.max(
            Math.min(
                Math.round(contentScrollY / getLineHeight(emailContent)),
                selectedEmailTotalLines
            ),
            0
        );
        selectedEmailScrollPercentage = Math.floor(
            (selectedEmailTopmostLine / selectedEmailTotalLines) * 100
        );
    };

    onMount(() => {
        emailHeaderHeight = emailHeader?.clientHeight;
        mounted = true;
    });

    // This updates the line counter
    // Is triggered when:
    // 1. The tab changes
    // 2. On window resize (see Email.svelte)
    $: if (tabData != undefined && mounted) {
        (async () => {
            await tick();
            selectedEmailTotalLines = countLines(emailContentSection);
        })();
    }
</script>

<main>
    <div class="email-header" bind:this={emailHeader}>
        <div>
            From: <span>{tabData.sender?.username ?? "Unknown sender"}</span>
            &lt;{tabData.sender?.address ?? "unknown"}&gt;
        </div>

        <div>
            CC: <span>{tabData.cc?.username ?? "Unknown user"}</span>
            &lt;{tabData.cc?.address ?? "unknown"}&gt;
        </div>
    </div>

    <div
        class="email-content"
        style="
      padding-top: calc(2em + {emailHeaderHeight}px);
    "
        bind:this={emailContent}
        on:scroll={handleContentScroll}
    >
        <h2>{tabData.title}</h2>
        <section bind:this={emailContentSection}>
            {tabData.content}
        </section>
        <!-- <p>{tabData.sender?.username ?? ""}</p>
        <div class="space-filler" /> -->

        <div class="line-counter" style="bottom: {-contentScrollY - 2}px;">
            - line: {selectedEmailTopmostLine} / {selectedEmailTotalLines} - {selectedEmailScrollPercentage}%
            -
        </div>
    </div>
</main>

<style>
    main {
        position: relative;
        width: 100%;
        height: 100%;
    }

    .email-header {
        position: absolute;
        top: 0;
        left: 0;
        right: 0;
        width: calc(100% - 2 * 0.5em);
        padding: 0.5em;
        border-bottom: 1px solid var(--color-accent);
        background-color: var(--color-bg);
        color: var(--color-shaded);
        z-index: 2;
        text-align: left;
        font-size: 12px;
    }

    .email-header span {
        color: var(--color-highlight);
    }

    .email-content {
        position: relative;
        height: calc(100% - 2em);
        margin: 0 4em 4em 4em;
        box-sizing: border-box;
        text-align: left;
        overflow-y: scroll;
        white-space: pre-line;
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
        align-content: center;
        text-align: right;
        font-size: 12px;
        color: var(--color-accent);
        background-color: var(--color-bg);
    }
</style>
