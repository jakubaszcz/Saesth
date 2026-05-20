import {invoke} from "@tauri-apps/api/core";
import {Sound} from "../../structures/sounds/sounds.ts";

export async function APIFetchSound(): Promise<Sound[]> {
    return await invoke<Sound[]>("fetch_sounds");
}

export async function APIToggleSound(sound_id: string): Promise<boolean> {
    return await invoke<boolean>("toggle_sound", { soundId: sound_id });
}

export async function APIChangeVolume(sound_id: string, volume: number): Promise<number> {
    return await invoke<number>("volume_sound", { soundId: sound_id, volume: volume });
}