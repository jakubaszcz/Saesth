import {Sound} from "../../interfaces/sounds/interface_sounds.ts";
import {invoke} from "@tauri-apps/api/core";

export async function APIFetchSound(): Promise<Sound[]> {
    return await invoke<Sound[]>("fetch_sounds");
}

export async function APIToggleSound(sound_id: string): Promise<boolean> {
    return await invoke<boolean>("toggle_sound", { sound_id });
}

export async function APIChangeVolume(sound_id: string, volume: number): Promise<number> {
    return await invoke<number>("volume_sound", { sound_id, volume });
}