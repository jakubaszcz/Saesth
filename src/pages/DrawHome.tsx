import {SoundCard} from "../component/Sound-card.tsx";
import {useEffect, useState} from "react";
import {Setup, SetupKeys, SoundFront} from "../interface/structures.ts";
import {invoke} from "@tauri-apps/api/core";
import {SoundModal} from "../component/SoundModal.tsx";

export function DrawHome() {

    const [sounds, setSounds] = useState<SoundFront[]>([]);
    const [setup, setSetup] = useState<Setup>(null as unknown as Setup);
    const [open, setOpen] = useState<SoundFront | null>(null);

    useEffect(() => {
        async function fetchSounds() {
            try {
                const fetchedSounds = await invoke<SoundFront[]>("get_sounds");
                setSounds(fetchedSounds);
            } catch (error) {
                console.error("Failed loading songs :", error);
            }
        }

        async function fetchSetup() {
            try {
                const fetchedSetup = await invoke<Setup>("fetch_setup");
                setSetup(fetchedSetup);
            } catch (error) {
                console.error("Failed loading setup :", error);
            }
        }

        fetchSounds().catch();
        fetchSetup().catch();
    }, []);

    const handleToggleEffect = async (id: string, effect_id: string) => {
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

    const handleToggleSetup = async (key: keyof Setup) => {
        try {
            await invoke("toggle_setup", { key: key});

            setSetup({
                ...setup,
                [key]: !setup[key],
            })

        } catch (error) {
            console.error("Failed to toggle setup:", error);
        }
    }

    const handeVolumeSetup = async (key: keyof Setup, value: number) => {
        try {
            await invoke("volume_setup", { key: key, value: value})

            setSetup({
                ...setup,
                [key]: value,
            })

        } catch (error) {
            console.error("Failed to toggle setup:", error);
        }
    }

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
        <div>
            <div className="w-full grid grid-cols-2 lg:grid-cols-4 gap-4 font-manrope">
                {sounds.map((data) => (
                    <SoundCard
                        key={data.data.id}
                        id={data.data.id}
                        data={data.data}
                        effect={data.effects.filter((effect) => effect.active)}
                        effects={data.effects}
                        onClick={() => handleTogglePlay(data.data.id)}
                        onOpen={() => setOpen(data)}
                        onChanged={(volume) => handleVolumeChange(data.data.id, volume)}
                    />
                ))}
            </div>

            <div>
                <h1>Setup</h1>
                <button onClick={() => handleToggleSetup(SetupKeys.SETUP_GLOBAL_TOGGLE)}>Toggle setup</button>
                <p>Toggle : {setup?.setup_global_toggle ? "On" : "Off"}</p>
                <p>Volume</p>
                <input
                    type="range"
                    min={0}
                    max={100}
                    step={1}
                    value={setup?.setup_global_volume * 100}
                    onChange={(e) => handeVolumeSetup(SetupKeys.SETUP_GLOBAL_VOLUME, parseFloat(e.target.value) / 100)
                    }/>

                <h1>Keyboard</h1>
                <button onClick={() => handleToggleSetup(SetupKeys.SETUP_KEYBOARD_TOGGLE)}>Toggle setup</button>
                <p>Toggle : {setup?.setup_keyboard_toggle ? "On" : "Off"}</p>
                <p>Volume</p>
                <input
                    type="range"
                    min={0}
                    max={100}
                    step={1}
                    value={setup?.setup_keyboard_volume * 100}
                    onChange={(e) => handeVolumeSetup(SetupKeys.SETUP_KEYBOARD_VOLUME, parseFloat(e.target.value) / 100)
                    }/>

                <h1>Mouse</h1>
                <button onClick={() => handleToggleSetup(SetupKeys.SETUP_MOUSE_TOGGLE)}>Toggle setup</button>
                <p>Toggle : {setup?.setup_mouse_toggle ? "On" : "Off"}</p>
                <p>Volume</p>
                <input
                    type="range"
                    min={0}
                    max={100}
                    step={1}
                    value={setup?.setup_mouse_volume * 100}
                    onChange={(e) => handeVolumeSetup(SetupKeys.SETUP_MOUSE_VOLUME, parseFloat(e.target.value) / 100)
                    }/>

            </div>
            {open && (
                <SoundModal
                    data={open}
                    onClose={() => setOpen(null)}
                    isClose={false}
                    onToggleEffect={handleToggleEffect}
                />
            )}
        </div>
    )
}