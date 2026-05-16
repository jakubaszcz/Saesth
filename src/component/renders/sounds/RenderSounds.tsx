import {SoundCard} from "../component/Sound-card.tsx";
import {useEffect, useState} from "react";
import {invoke} from "@tauri-apps/api/core";
import {Sound} from "../../../interfaces/sounds/interface_sounds.ts";
import {useSounds} from "../../../hooks/sounds/useSounds.ts";

export function RenderSounds() {
    const {
        sounds,
        toggleSound,
        volumeSound
    } = useSounds()

    /*const handleToggleEffect = async (id: string, effect_id: string) => {
        try {
            const updatedSounds = await invoke<SoundFront[]>("toggle_effect", {
                soundId: id,
                effectId: effect_id,
            });

            setSounds(updatedSounds);

            const updatedOpen = updatedSounds.find(
                (sounds) => sounds.data.id === id
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
                {sounds.map((data,) => (
                    <SoundCard
                        key={data.sound_id}
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