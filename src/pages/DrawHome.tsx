import {SoundCard} from "../component/Sound-card.tsx";
import {useEffect, useState} from "react";
import {invoke} from "@tauri-apps/api/core";
/*import {SoundModal} from "../component/SoundModal.tsx";*/
import { ComponentSetup } from "../features/setup/ComponentSetup.tsx";
import {Sound} from "../interfaces/sounds/interface_sounds.ts";

export function DrawHome() {

    const [sounds, setSounds] = useState<Sound[]>([]);
    /*const [open, setOpen] = useState<SoundFront | null>(null);*/

    useEffect(() => {
        async function fetchSounds() {
            try {
                const response = await invoke<Sound[]>("fetch_sounds");
                setSounds(response);
            } catch (error) {
                console.error("Failed loading songs :", error);
            }
        }

        fetchSounds().catch();
    }, []);

    const toggleSound = async (sound_id: string) => {
        try {
            const response = await invoke<boolean>("toggle_sound", { sound_id });

            setSounds((prev) =>
                prev.map((sound) =>
                    sound.sound_id === sound_id
                        ? { ...sound, play: response }
                        : sound
                )
            );
        } catch (error) {
            console.error("Failed to toggle play:", error);
        }
    };

    const volumeSound = async (sound_id: string, volume: number) => {
        try {
            const response = await invoke<number>("change_volume", { sound_id, volume });

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

    /*const handleToggleEffect = async (id: string, effect_id: string) => {
        try {
            const updatedSounds = await invoke<SoundFront[]>("toggle_effect", {
                soundId: id,
                effectId: effect_id,
            });

            setSounds(updatedSounds);

            const updatedOpen = updatedSounds.find(
                (sound) => sound.data.id === id
            );

            setOpen(updatedOpen ?? null);

        } catch (error) {
            console.error("Failed to toggle effect:", error);
        }
    };
*/

    return (
        <div>
            <div className="w-full grid grid-cols-2 lg:grid-cols-4 gap-4 font-manrope">
                {sounds.map((data) => (
                    <SoundCard
                        sound={data}
                    />
                ))}
            </div>
            <ComponentSetup/>
            {/*{open && (
                <SoundModal
                    data={open}
                    onClose={() => setOpen(null)}
                    isClose={false}
                    onToggleEffect={handleToggleEffect}
                />
            )}*/}
        </div>
    )
}