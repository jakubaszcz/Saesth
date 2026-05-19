import {invoke} from "@tauri-apps/api/core";
import {Settings} from "../../structures/settings/settings.ts";

export async function APIFetchSetting(): Promise<Settings[]> {
    return await invoke<Settings[]>("fetch_settings");
}

export async function APIToggleSetting(setting_id: string): Promise<boolean> {
    return await invoke<boolean>("toggle_setting", { settingId: setting_id });
}