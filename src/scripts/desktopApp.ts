export interface DesktopAppShortcut {
  componentConstructor: any,
  name: string,

  component?: any,
  props?: any,
  badgeCount?: number,
  opened?: boolean,
  minimized?: boolean
};
