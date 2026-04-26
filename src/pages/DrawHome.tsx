import {SoundCard} from "../component/Sound-card.tsx";
import {useEffect, useState} from "react";
import {SoundFront} from "../interface/sound-data.ts";
import {invoke} from "@tauri-apps/api/core";

export function DrawHome() {

    const [sounds, setSounds] = useState<SoundFront[]>([]);

    useEffect(() => {
        async function fetchSounds() {
            try {
                const fetchedSounds = await invoke<SoundFront[]>("get_sounds");
                setSounds(fetchedSounds);
            } catch (error) {
                console.error("Failed loading songs :", error);
            }
        }
        fetchSounds();
    }, []);

    const handleTogglePlay = async (id: string) => {
        try {
            const updatedSounds = await invoke<SoundFront[]>("toggle_play", { id });
            setSounds(   updatedSounds);
        } catch (error) {
            console.error("Failed to toggle play:", error);
        }
    };

    const handleVolumeChange = async (id: string, volume: number) => {
        try {
            const updatedSounds = await invoke<SoundFront[]>("change_volume", { id, volume });
            setSounds(updatedSounds);
        } catch (error) {
            console.error("Failed to change volume:", error);
        }
    }

    return (
        <div className="w-full grid grid-cols-2 lg:grid-cols-4 gap-4 font-manrope">
            {sounds.map((data) => (
                <SoundCard
                    key={data.data.id}
                    id={data.data.id}
                    data={data.data}
                    effects={data.effects ?? []}
                    onClick={() => handleTogglePlay(data.data.id)}
                    onChanged={(volume) => handleVolumeChange(data.data.id, volume)}
                />
            ))}
        </div>
    )
}