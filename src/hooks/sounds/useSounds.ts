import {
    APIFetchSound,
    APIToggleSound,
    APIChangeVolume, APIToggleSoundEffect
} from "../../api/sounds/sounds.ts"
import {useEffect, useState} from "react";
import {Sound} from "../../structures/sounds/sounds.ts";

export function useSounds() {
    const [sounds, setSounds] = useState<Sound[]>([]);

    useEffect(() => {
        async function loadSounds() {
            try {
                const response = await APIFetchSound();

                setSounds(response);
                console.log("success sound")
            } catch (error) {
                console.error("Failed to load sounds:", error);
            }
        }

        loadSounds()
    })

    const toggleSound = async (sound_id: string) => {
        try {
            const response = await APIToggleSound(sound_id);

            setSounds((prev) =>
                prev.map((sound) =>
                    sound.sound_id === sound_id
                        ? { ...sound, play: response }
                        : sound
                )
            );
        } catch (error) {
            console.error("Failed to toggle sound:", error);
        }
    };

    const volumeSound = async (sound_id: string, volume: number) => {
        try {
            const response = await APIChangeVolume(sound_id, volume);

            setSounds((prev) =>
                prev.map((sound) =>
                    sound.sound_id === sound_id
                        ? { ...sound, volume: response }
                        : sound
                )
            )
        } catch (error) {
            console.error("Failed to change volume:", error);
        }
    }

    const toggleSoundEffect = async (sound_id: string, effect_id: string) => {
        try {
            const response = await APIToggleSoundEffect(sound_id, effect_id);

            console.log("response : ", response)


            setSounds((prev) =>
                prev.map((sound) =>
                    sound.sound_id === sound_id
                        ? {
                            ...sound,
                            effects: sound.effects.map((effect) =>
                                effect.effect_id === effect_id
                                    ? { ...effect, active: response }
                                    : effect
                            ),
                        }
                        : sound
                )
            );
        } catch (error) {
            console.error("Failed to toggle sound effect:", error);
        }
    };

    return {
        sounds,
        toggleSound,
        volumeSound,
        toggleSoundEffect
    }
}