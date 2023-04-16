import { InvokeArgs, invoke } from "@tauri-apps/api/tauri";
import { Action, Severity, useNotificationsStore } from "src/store/notifications";

export type AppString =
    | "finance"
    | "email"
    | "welfare"
    | "business"
    | "healthcare";

type If<T, Y, N> = T extends true ? Y : N;
type TypedResult<T, K extends boolean> = { 
    value: If<K, T, null>,
    success: K, 
}
type Result<T> = TypedResult<T, true> | TypedResult<T, false>;

export const handleInvoke = async <T>(
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
            errorNotif("An error occured", err.message, app);
        }
        return { value: null, success: false};
    }
};

// tell the parent component an error has occurred
export const errorNotif = (
    errorTitle: string,
    errorMessage: string,
    app: AppString,
) => {
    const notis = useNotificationsStore();
    notis.addNotification({
        app,
        header: errorTitle,
        content: errorMessage,
        action: Action.Nothing,
        severity: Severity.Error,
    });
};
