import { WINDOW_SEND_NOTIFICATION } from "./windowEvent";
import { elasticIn } from "svelte/easing";

export type AppString =
    | "finance"
    | "email"
    | "welfare"
    | "business"
    | "healthcare";

export const handleInvoke = async (
    dispatcher: Function,
    invokeRes: Promise<any>,
    app: AppString
): Promise<any> => {
    let error = null;

    const result = await invokeRes.catch((e) => {
        errorNotif(dispatcher, "An error occured", e, app);
        error = e;
    });

    if (error !== null) {
        return false;
    }

    if (typeof result === "object" && result !== null) {
        if (result.error) {
            errorNotif(dispatcher, "An error occured", result.error, app);
            return false;
        }
    }

    return result;
};

export const errorNotif = (
    dispatcher: Function,
    errorTitle: string,
    errorMessage: string,
    app: AppString
) => {
    dispatcher("criticalWindowEvent", {
        type: WINDOW_SEND_NOTIFICATION,
        data: {
            app,
            header: errorTitle,
            content: errorMessage,
            severity: "error",
        },
    });
};

export const popout = (node: any): any => {
    return {
        duration: 200,
        css: (t: any) => {
            const eased = elasticIn(t);

            return `
                    scale: ${eased}
            `;
        },
    };
};

interface EmailData {
    title: string;
    content: string;
}
