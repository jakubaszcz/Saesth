import {invoke} from "@tauri-apps/api/core";
import {Setting} from "../../structures/settings/settings.ts";

export async function APIFetchSetting(): Promise<Setting[]> {
    return await invoke<Setting[]>("fetch_settings");
}

export async function APIToggleSetting(setting_id: string): Promise<boolean> {
    return await invoke<boolean>("toggle_setting", { settingId: setting_id });
}