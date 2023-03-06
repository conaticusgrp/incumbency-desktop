<script lang="ts">
    import { createEventDispatcher, onMount, SvelteComponent } from "svelte";
    import Window, {
        type CriticalWindowData,
        type Pos,
        type Size,
    } from "./Window.svelte";
    import {
        TAB_LIST_ENTRY_MARGIN,
        TAB_LIST_MIN_WIDTH,
        TAB_LIST_WIDTH,
        USERNAME,
        USERNAME_HEIGHT,
    } from "../../../scripts/desktopConstants";
    import {
        handleDataEvents,
        WINDOW_OPENED,
    } from "../../../scripts/windowEvent";

    export let title: string;
    export let pos: Pos;
    export let size: Size;
    export let windowData: CriticalWindowData;

    export let tabButtonComponent: typeof SvelteComponent;
    export let tabButtonData: any[] = [];
    export let tabs: { c: typeof SvelteComponent; data: any }[];

    export let currentTabIndex: number = -1;

    let dispatcher = createEventDispatcher();

    const selectTab = (e: CustomEvent): void => {
        const i = e.detail.index;
        if (i < 0 || i >= tabButtonData.length) return;
        currentTabIndex = i;
    };

    let appData: any;
</script>

<Window
    {title}
    {pos}
    {size}
    {windowData}
    on:criticalWindowEvent={(e) => dispatcher("criticalWindowEvent", e.detail)}
    on:windowEvent={(e) => {
        dispatcher("windowEvent", e.detail);

        if (e.detail.type === WINDOW_OPENED && currentTabIndex === -1) {
            currentTabIndex = 0;
        }

        let newData = handleDataEvents(e, appData);
        if (newData) {
            appData = newData;
        }
    }}
>
    <main
        style="
      --tab-list-width: {TAB_LIST_WIDTH};
      --tab-list-min-width: {TAB_LIST_MIN_WIDTH};
      --tab-list-entry-margin: {TAB_LIST_ENTRY_MARGIN};
      --username-height: {USERNAME_HEIGHT};
    "
    >
        <section>
            <div class="tab-list">
                {#each tabButtonData as data, i}
                    <svelte:component
                        this={tabButtonComponent}
                        index={i}
                        selected={i === currentTabIndex}
                        {data}
                        on:selectTab={selectTab}
                    />
                {/each}
            </div>

            <div class="username">
                Authenticated as:
                <div>{USERNAME}</div>
            </div>
        </section>

        <section>
            {#if currentTabIndex >= 0 && currentTabIndex < tabs.length}
                <svelte:component
                    this={tabs[currentTabIndex].c}
                    tabData={tabs[currentTabIndex].data}
                    data={appData}
                />
            {/if}
        </section>
    </main>
</Window>

<style>
    main {
        display: flex;
        flex-direction: row;
        width: 100%;
        height: 100%;
    }

    main > section:first-of-type {
        width: var(--tab-list-width);
        min-width: var(--tab-list-min-width);
        height: 100%;
        border-right: 1px solid var(--color-accent);
    }

    .tab-list {
        display: flex;
        flex-direction: column;
        width: 100%;
        height: calc(100% - var(--username-height));
        overflow-y: scroll;
    }

    .tab-list::-webkit-scrollbar {
        display: none;
    }

    .username {
        width: 100%;
        height: var(--username-height);
        border-top: 1px solid var(--color-accent);
        color: var(--color-accent);
    }

    .username div {
        font-weight: bold;
    }

    main > section:last-of-type {
        width: calc(
            100% - max(var(--tab-list-width), var(--tab-list-min-width))
        );
    }
</style>
