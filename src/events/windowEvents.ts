// Critical event is an event that is needed for window-desktop communication
// Critical events are transmitted via 'criticalWindowEvent' wrapper event
// Non-critical event is an event that any app can listen for for its own needs
// Non-critical events are transmitted via 'windowEvent' wrapper event

// Critical
export const WINDOW_CLOSE = "windowClose";
export const WINDOW_MINIMIZE = "windowMinimize";
export const WINDOW_AQUIRE_FOCUS = "windowAquireFocus";
export const WINDOW_SEND_NOTIFICATION = "windowSendNotification";

// Non-critical
export const WINDOW_MAXIMIZE = "windowMaximize";
export const WINDOW_RESIZE = "windowResize";
export const WINDOW_OPENED = "windowOpened";
export const EMAIL_CREATE = "emailCreate";

// Apps
export const APP_UPDATE = "appUpdate";

type DataEvent = { detail: { data: any, type: string } };

export const handleDataEvents = (
    { detail: { data, type } }: DataEvent,
    appData: any
): any => {
    if (!data) return;

    console.log(data);

    switch (type) {
        case WINDOW_OPENED:
            return JSON.parse(data);
        case APP_UPDATE:
            return {
                ...appData,
                ...data.data,
            };
        default:
            break;
    }
};
