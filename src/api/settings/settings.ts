import {invoke} from "@tauri-apps/api/core";
import {Setting} from "../../structures/settings/settings.ts";
import {setSaesthWidgetEnabled} from "../wist.ts";

let settingOperations: Promise<unknown> = Promise.resolve();

function enqueueSetting<T>(operation: () => Promise<T>): Promise<T> {
    const result = settingOperations.then(operation);
    settingOperations = result.catch(() => {});
    return result;
}

export function initializeWistSetting(): Promise<void> {
    return enqueueSetting(async () => {
        const settings = await APIFetchSetting();
        await setSaesthWidgetEnabled(settings.some(setting => setting.setting_id === "wist_widget" && setting.active));
    });
}

export async function APIFetchSetting(): Promise<Setting[]> {
    return await invoke<Setting[]>("fetch_settings");
}

export async function APIToggleSetting(setting_id: string): Promise<boolean> {
    return enqueueSetting(async () => {
        const active = await invoke<boolean>("toggle_setting", { settingId: setting_id });
        if (setting_id === "wist_widget") {
            await setSaesthWidgetEnabled(active).catch(error => console.error("Wist setting failed:", error));
        }
        return active;
    });
}
