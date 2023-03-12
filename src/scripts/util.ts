import { WINDOW_SEND_NOTIFICATION } from "./windowEvent";
import { elasticIn } from "svelte/easing";

export const handleInvoke = async (
    dispatcher: Function,
    invokeRes: Promise<any>
): Promise<any> => {
    const result = await invokeRes.catch((e) => {
        errorNotif(dispatcher, "An error occured", e);
    });

    if (result.error) {
        errorNotif(dispatcher, "An error occured", result.error);
        return false;
    }

    return result;
};

export const errorNotif = (
    dispatcher: Function,
    errorTitle: string,
    errorMessage: string
) => {
    dispatcher("criticalWindowEvent", {
        type: WINDOW_SEND_NOTIFICATION,
        data: {
            app: "Finance",
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
