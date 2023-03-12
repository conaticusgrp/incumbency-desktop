<script lang="ts" context="module">
    interface DesktopAppShortcut {
        componentConstructor: any;
        name: string;

        component?: any;
        props?: any;
        badgeCount: number;
        opened?: boolean;
        minimized?: boolean;
    }

    type ModalState = "closed" | "log off" | "shut down";
</script>

<script lang="ts">
    import { listen } from "@tauri-apps/api/event";
    import {
        APP_LIST_MIN_WIDTH,
        APP_LIST_WIDTH,
        TOP_PANEL_HEIGHT,
        NOTIFICATION_MARGIN_X,
        NOTIFICATION_WIDTH,
        TOOLBAR_HEIGHT,
        MODAL_TIMER_DELAY,
    } from "../../../scripts/desktopConstants";
    import {
        WINDOW_AQUIRE_FOCUS,
        WINDOW_CLOSE,
        WINDOW_MINIMIZE,
        WINDOW_SEND_NOTIFICATION,
    } from "../../../scripts/windowEvent";

    import Email from "../windows/Email/Email.svelte";
    import Finance from "../windows/Finance/Finance.svelte";
    import Healthcare from "../windows/Healthcare/Healthcare.svelte";
    import Welfare from "../windows/Welfare/Welfare.svelte";
    import Business from "../windows/Business/Business.svelte";
    import Notification, { type NotificationData } from "./Notification.svelte";

    let startMenu: HTMLElement;
    let startMenuExpanded: boolean = false;
    let notificationSectionExpanded = false;
    let date: string = "undefined date";
    let wallpaperPath: string | null = "./src/assets/Wallpaper.png";
    let apps: DesktopAppShortcut[] = [
        { componentConstructor: Email, name: "Email", badgeCount: 0 },
        { componentConstructor: Finance, name: "Finance", badgeCount: 0 },
        { componentConstructor: Healthcare, name: "Healthcare", badgeCount: 0 },
        { componentConstructor: Welfare, name: "Welfare", badgeCount: 0 },
        { componentConstructor: Business, name: "Business", badgeCount: 0 },
    ];
    let focusedApp: number | null = null;

    let notifications: NotificationData[] = [];
    let showLatestNotification = true;

    let modalState: ModalState = "closed";
    let modalTimerResolve: (() => void) | null = null;
    let modalTimerCountdown = 0;

    $: if (modalState !== "closed") {
        new Promise<void>(async (resolve, reject) => {
            modalTimerResolve = resolve;
            for (let i = 0; i < MODAL_TIMER_DELAY; i++) {
                modalTimerCountdown = MODAL_TIMER_DELAY - i;
                await new Promise<void>((res0) => {
                    setTimeout(res0, 1_000);
                });
            }
            modalTimerResolve = null;
            modalResolve();
            resolve();
        });
    }

    const handleOpenApp = (index: number): void => {
        if (index < 0 || index >= apps.length) return;
        focusedApp = index;
        updateUI();

        if (!apps[index].opened) {
            apps[index].opened = true;
            apps[index].badgeCount = 0;
        }
    };

    const updateUI = () => {
        apps = apps;
    };

    const handleCriticalEvent = (index: number, e: CustomEvent): void => {
        if (index < 0 || index >= apps.length) return;

        switch (e.detail.type) {
            case WINDOW_CLOSE:
                {
                    apps[index].opened = false;
                    if (focusedApp === index) {
                        focusedApp = null;
                    }
                    updateUI();
                }
                break;

            case WINDOW_AQUIRE_FOCUS:
                {
                    focusedApp = index;
                    updateUI();
                }
                break;

            case WINDOW_MINIMIZE:
                {
                    apps[index].minimized = true;
                    updateUI();
                }
                break;

            case WINDOW_SEND_NOTIFICATION:
                {
                    receiveNotification(e.detail.data);
                }
                break;

            default:
                break;
        }
    };

    const openStartMenu = (): void => {
        startMenuExpanded = true;

        document.addEventListener("click", closeStartMenuIfClickedAway);
    };

    const toggleNotificationsSection = (): void => {
        notificationSectionExpanded = !notificationSectionExpanded;
        showLatestNotification = false;
    };

    const closeStartMenuIfClickedAway = (e: MouseEvent): void => {
        if (e.target == null || startMenu.contains(e.target as HTMLElement))
            return;

        startMenuExpanded = false;
    };

    const receiveNotification = (n: NotificationData): void => {
        const app = apps.find(
            (app) => app.name.toLowerCase() === n.app?.toLowerCase()
        );

        if (app && !app.opened) {
            app.badgeCount += 1;
        }

        n.date = date;
        notifications.push(n);
        showLatestNotification = true;
        updateUI();
    };

    const getModalTitle = (): string => {
        switch (modalState) {
            case "log off":
                return "logoff";
            case "shut down":
                return "shutdown";
            default:
                return "";
        }
    };

    const getModalAutoAction = (): string => {
        switch (modalState) {
            case "log off":
                return "Logging off";
            case "shut down":
                return "Shutting down";
            default:
                return "";
        }
    };

    const getModalAction = (): string => {
        switch (modalState) {
            case "log off":
                return "Log off";
            case "shut down":
                return "Shut down";
            default:
                return "";
        }
    };

    const modalResolve = (): void => {
        if (modalTimerResolve != null) modalTimerResolve();

        switch (modalState) {
            case "log off":
                break;
            case "shut down":
                break;
            default:
                break;
        }
        modalState = "closed";
    };

    const modalReject = (): void => {
        if (modalTimerResolve != null) modalTimerResolve();

        switch (modalState) {
            case "log off":
                break;
            case "shut down":
                break;
            default:
                break;
        }
        modalState = "closed";
    };

    listen("new_day", (d) => {
        //@ts-ignore
        date = d.payload.date as string;
    });

    // listen("open_debugger_app", (e) => {
    //     //@ts-ignore
    //     handleOpenApp(apps.findIndex((v) => v.name === "DEBUG"));
    // });

    document.addEventListener("keydown", (e) => {
        if (e.altKey && e.key == "F4") {
            if (focusedApp != null) {
                apps[focusedApp].opened = false;
                focusedApp = null;
            }
            e.preventDefault();
        }
    });
</script>

<main>
    <div
        class="app-list-section"
        style="width: {APP_LIST_WIDTH}; min-width: {APP_LIST_MIN_WIDTH};"
    >
        <h2>Installed Software</h2>

        <div class="app-list">
            {#each apps as shortcut, i}
                <!-- empty on:keydown to supress a warning -->
                <div
                    style="color: var({apps[i].opened
                        ? '--color-highlight'
                        : '--color-shaded'});"
                    on:click={() => handleOpenApp(i)}
                    on:keydown={() => {}}
                >
                    {shortcut.name}

                    {#if shortcut.badgeCount != undefined && shortcut.badgeCount > 0}
                        <span>({shortcut.badgeCount})</span>
                    {/if}
                </div>
            {/each}
        </div>
    </div>

    <div class="content" style="width: calc(100% - {APP_LIST_WIDTH});">
        <div
            class="top-panel"
            style="height: {TOP_PANEL_HEIGHT};"
            on:click={openStartMenu}
            on:keydown={() => {}}
        >
            <div
                class="start-menu"
                aria-expanded={startMenuExpanded}
                bind:this={startMenu}
            >
                {#if !startMenuExpanded}
                    {date}
                {:else}
                    <button>{date}</button>
                    <button on:click={() => {}}>Shut down</button>
                    <button
                        on:click={() => {
                            modalState = "log off";
                        }}>Logoff</button
                    >
                    <button on:click={() => {}}>Restart</button>
                {/if}
            </div>

            <button
                class="notification-section-toggle"
                on:click={toggleNotificationsSection}
            >
                Notifications
            </button>
        </div>

        <div
            class="windows"
            style="
        height: calc(100% - {TOP_PANEL_HEIGHT} - {TOOLBAR_HEIGHT});
        background-image: {wallpaperPath != null
                ? `url(${wallpaperPath})`
                : 'none'};
      "
        >
            {#each apps as app, i (i)}
                <svelte:component
                    this={app.componentConstructor}
                    bind:this={app.component}
                    windowData={{
                        opened: !!app.opened && !app.minimized,
                        focused: i === focusedApp,
                        index: i,
                    }}
                    on:criticalWindowEvent={(e) => handleCriticalEvent(i, e)}
                    {...app.props}
                />
            {/each}

            {#if showLatestNotification && notifications.length > 0}
                <div
                    class="single-notification-container"
                    style="right: {NOTIFICATION_MARGIN_X};"
                >
                    <Notification
                        actionTitle={notifications[notifications.length - 1]
                            .actionTitle}
                        actionFunction={notifications[
                            notifications.length - 1
                        ].actionTitle?.toLowerCase() === "open app"
                            ? () => {
                                  apps.forEach((app, idx) => {
                                      if (
                                          app.name.toLowerCase() ===
                                          notifications[
                                              notifications.length - 1
                                          ].app?.toLowerCase()
                                      ) {
                                          handleOpenApp(idx);
                                      }
                                  });
                              }
                            : () => {}}
                        justDisplayed={true}
                        onDismissed={() =>
                            notifications.splice(notifications.length - 1, 1)}
                        data={notifications[notifications.length - 1]}
                    />
                </div>
            {/if}

            {#if notificationSectionExpanded}
                <div
                    class="notifications-section"
                    style="width: calc({NOTIFICATION_WIDTH} + {NOTIFICATION_MARGIN_X} * 2);"
                >
                    {#each notifications as notif, idx}
                        <Notification
                            actionTitle={notif.actionTitle}
                            actionFunction={notif.actionTitle?.toLowerCase() ===
                            "open app"
                                ? () => {
                                      apps.forEach((app, idx) => {
                                          if (
                                              app.name.toLowerCase() ===
                                              notif.app?.toLowerCase()
                                          ) {
                                              handleOpenApp(idx);
                                          }
                                      });
                                  }
                                : () => {}}
                            onDismissed={() => {
                                notifications.splice(idx, 1);
                            }}
                            data={notif}
                        />
                    {/each}
                </div>
            {/if}
        </div>

        <div class="toolbar" style="height: {TOOLBAR_HEIGHT};">
            {#each apps as shortcut, i}
                {#if apps[i].opened}
                    <!-- !! to cast (boolean | undefined) to boolean -->
                    <!-- empty on:keydown to supress a warning -->
                    <span
                        data-minimized={!!apps[i].minimized}
                        title={shortcut.name}
                        on:click={() => {
                            focusedApp = i;
                            updateUI();
                        }}
                        on:keydown={() => {}}
                    >
                        {shortcut.name}
                    </span>
                {/if}
            {/each}
        </div>
    </div>

    {#if modalState !== "closed"}
        <div class="modal">
            <div class="modal-label">
                <div class="content">
                    <h1>{getModalTitle()}</h1>
                    <span
                        >{getModalAutoAction()} automatically in {modalTimerCountdown}s</span
                    >
                </div>

                <div class="actions">
                    <button on:click={modalResolve}>{getModalAction()}</button>
                    <button on:click={modalReject}>Cancel</button>
                </div>
            </div>
        </div>
    {/if}
</main>

<style>
    main {
        display: flex;
        flex-direction: row;
        position: relative;
        width: 100%;
        height: 100%;
        z-index: 0;
        isolation: isolate;
        color: var(--color-highlight);
        background-color: black;
    }

    .app-list-section {
        display: flex;
        flex-direction: column;
        align-items: center;
        border-right: 1px solid var(--color-accent);
    }

    .app-list-section > h2 {
        margin: 2em;
        font-size: 14px;
        font-weight: bolder;
    }

    .app-list {
        display: flex;
        flex-direction: column;
        align-items: flex-start;
        width: calc(100% - 1em * 2);
        margin: 0 1em 0 1em;
        font-size: 14px;
    }

    .app-list > div {
        width: 100%;
        margin: 0.5em 0 0.5em 0;
        text-align: left;
        cursor: pointer;
    }

    .app-list > div > span {
        color: var(--color-critical);
        font-weight: bold;
    }

    .content {
        display: flex;
        flex-direction: column;
    }

    .top-panel {
        display: flex;
        justify-content: center;
        /* align-items: center; */
        position: relative;
        min-height: min-content;
        border-bottom: 1px solid var(--color-accent);
    }

    .start-menu {
        display: flex;
        flex-direction: column;
        justify-content: center;
        align-items: center;
        width: min-content;
        height: min-content;
        cursor: pointer;
    }

    .start-menu[aria-expanded="false"] {
        align-self: center;
    }

    .start-menu[aria-expanded="true"] {
        width: 30%;
        z-index: 2;
        background-color: var(--color-bg);
        border: 1px solid var(--color-accent);
        border-top: none;
    }

    .start-menu[aria-expanded="true"] > button {
        width: 100%;
    }

    .start-menu[aria-expanded="true"] > button:hover {
        background-color: var(--color-accent);
        color: var(--color-bg);
        font-weight: bold;
    }

    .single-notification-container {
        position: absolute;
        top: 0;
        z-index: 20000;
    }

    .notification-section-toggle {
        position: absolute;
        top: 0;
        right: 0;
        height: 100%;
        border-left: 1px solid var(--color-accent);
    }

    .windows {
        position: relative;
        /* z-index: 100; */
        isolation: isolate;
        background-repeat: no-repeat;
        background-position: center;
    }

    .toolbar {
        /* width: 100%; */
        display: flex;
        flex-direction: row;
        min-height: min-content;
        border-top: 1px solid var(--color-accent);
    }

    .toolbar > span {
        position: relative;
        display: flex;
        justify-content: center;
        align-items: center;
        padding: 0 2em 0 2em;
        border: 1px solid var(--color-accent);

        background-color: var(--color-accent);
        color: var(--color-bg);
        font-weight: bold;
    }

    .toolbar > span[data-minimized="true"] {
        background-color: unset;
        color: unset;
        font-weight: unset;
    }

    .notifications-section {
        display: flex;
        flex-direction: column;
        align-items: center;
        position: absolute;
        top: 0;
        right: 0;
        height: 100%;
        background-color: var(--color-bg);
        border-left: 1px solid var(--color-accent);
        z-index: 20000;
    }

    .modal {
        display: flex;
        align-items: center;
        position: absolute;
        top: 0;
        left: 0;
        width: 100%;
        height: 100%;
        background-color: rgba(0, 0, 0, 0.5);
        z-index: 9999;
    }

    .modal-label {
        display: flex;
        flex-direction: column;
        width: 640px;
        aspect-ratio: 5 / 0.6;
        margin: auto;
        background-color: var(--color-bg);
        border: 1px solid var(--color-accent);
        border-radius: 1rem;
    }

    .modal-label > .content {
        height: 100%;
        border-radius: 0.5em;
        background-color: var(--color-accent);
        color: var(--color-bg);
        font-weight: bold;
    }

    .modal-label > .content > h1 {
        padding: 1em 0 0.5em 0;
        letter-spacing: 0.25em;
        text-align: center;
        text-transform: uppercase;
        white-space: nowrap;
        text-overflow: ellipsis;
        font-weight: bold;
        font-size: 20px;
    }

    .modal-label > .actions {
        display: flex;
    }

    .modal-label > .actions > button {
        width: 100%;
    }

    .modal-label > .actions > button:last-of-type {
        border-left: 1px solid var(--color-accent);
        color: grey;
    }
</style>
