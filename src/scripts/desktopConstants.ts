export type CssUnit = 'px' | 'em' | 'rem' | '%' | 'vw' | 'vh' | 'vmin' | 'vmax' | 'fr' | 'number';

export class CssNumericalValue {
    value: number;
    unit: CssUnit;
    calculatedString: string;

    constructor(v: number, u: CssUnit) {
        this.value = v;
        this.unit = u;
        this.calculatedString = "";
        this.calculateString();
    }

    calculateString(): void {
        if (this.unit === 'number') {
            this.calculatedString = String(this.value);
        }
        this.calculatedString = `${this.value}${this.unit}`;
    }

    toString(): string {
        return this.calculatedString;
    }
};

export const RESIZE_BAR_SIZE = new CssNumericalValue(5, 'px');
export const MIN_WINDOW_WIDTH = new CssNumericalValue(400, 'px');
export const MIN_WINDOW_HEIGHT = new CssNumericalValue(200, 'px');
export const WINDOW_HEADER_HEIGHT = new CssNumericalValue(25, 'px');

export const APP_LIST_WIDTH = new CssNumericalValue(17, '%');
export const APP_LIST_MIN_WIDTH = new CssNumericalValue(200, 'px');
export const TOP_PANEL_HEIGHT = new CssNumericalValue(3, 'em');
export const TOOLBAR_HEIGHT = new CssNumericalValue(3, 'em');

export const NOTIFICATION_WIDTH = new CssNumericalValue(345, 'px');
export const NOTIFICATION_HEIGHT = new CssNumericalValue(133, 'px');
export const NOTIFICATION_MARGIN_X = new CssNumericalValue(2, 'em');
export const NOTIFICATION_MARGIN_Y = new CssNumericalValue(1, 'em');

export const TAB_LIST_WIDTH = new CssNumericalValue(35, '%');
export const TAB_LIST_MIN_WIDTH = new CssNumericalValue(250, 'px');
export const TAB_LIST_ENTRY_MARGIN = new CssNumericalValue(0.5, 'em');

export const USERNAME = "Joe";
export const USERNAME_HEIGHT = new CssNumericalValue(3.5, 'em');


// DEBUG, to be used
export const KEEP_NOTIFICATIONS_DISPLAYED = false;
