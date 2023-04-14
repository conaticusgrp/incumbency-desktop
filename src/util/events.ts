import { InvokeArgs, invoke } from "@tauri-apps/api/tauri";

type EmitFn = (event: 'windowSendNotification', data: NotificationData) => void;

export type AppString =
    | "finance"
    | "email"
    | "welfare"
    | "business"
    | "healthcare";

enum NotificationAction { OpenApp, Nothing };
type If<T, Y, N> = T extends true ? Y : N;
type TypedResult<T, K extends boolean> = { 
    value: If<K, T, null>,
    success: K, 
}
type Result<T> = TypedResult<T, true> | TypedResult<T, false>;

export const handleInvoke = async <T>(
    dispatcher: EmitFn,
    app: AppString,
    cmd: string,
    args?: InvokeArgs,
): Promise<Result<T>> => {
    try {
        // TODO(dylhack): investigate if we need to handle returned errors
        const result = await invoke<T>(cmd, args);
        return { value: result, success: true };
    } catch (err) {
        if (err instanceof Error) {
            errorNotif(dispatcher, "An error occured", err.message, app);
        }
        return { value: null, success: false};
    }
};

// tell the parent component an error has occurred
export const errorNotif = (
    dispatcher: EmitFn,
    errorTitle: string,
    errorMessage: string,
    app: AppString,
) => {
    dispatcher("windowSendNotification", {
        app,
        header: errorTitle,
        content: errorMessage,
        action: NotificationAction.Nothing,
        severity: "error",
    });
};
