import { WINDOW_SEND_NOTIFICATION } from "./windowEvent";

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
