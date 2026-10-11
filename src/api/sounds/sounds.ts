import {invoke} from "@tauri-apps/api/core";
import {Sound} from "../../structures/sounds/sounds.ts";
import {updateSaesthWidget} from "../wist.ts";

export async function APIFetchSound(): Promise<Sound[]> {
    return await invoke<Sound[]>("fetch_sounds");
}

export async function APIToggleSound(
    sound_id: string,
): Promise<boolean> {
    const response = await invoke<boolean>("toggle_sound", {
        soundId: sound_id,
    });

    try {
        const sounds = await APIFetchSound();

        await updateSaesthWidget({
            sounds: sounds
                .filter((sound) => sound.play)
                .map((sound) => sound.sound_id),
        });
    } catch (error) {
        console.error("Unable to update the Wist widget:", error);
    }

    return response;
}

export async function APIChangeVolume(sound_id: string, volume: number): Promise<number> {
    return await invoke<number>("volume_sound", { soundId: sound_id, volume: volume });
}

export async function APIToggleSoundEffect(sound_id: string, effect_id: string): Promise<boolean> {
    return await invoke<boolean>("toggle_sound_effect", { soundId: sound_id, effectId: effect_id });
}