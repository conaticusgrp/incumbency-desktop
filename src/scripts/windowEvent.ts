// Critical event is an event that is needed for window-desktop communication
// Critical events are transmitted via 'criticalWindowEvent' wrapper event
// Non-critical event is an event that any app can listen for for its own needs
// Non-critical events are transmitted via 'windowEvent' wrapper event

// Critical
export const WINDOW_CLOSE = 'windowClose';
export const WINDOW_MINIMIZE = 'windowMinimize';
export const WINDOW_AQUIRE_FOCUS = 'windowAquireFocus';

// Non-critical
export const WINDOW_MAXIMIZE = 'windowMaximize';
export const WINDOW_RESIZE = 'windowResize';
