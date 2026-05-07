import {useEffect, useState} from "react";
import {Setup, SetupKeys} from "./data-setup.ts";
import {invoke} from "@tauri-apps/api/core";


export function ComponentSetup() {

    const [setup, setSetup] = useState<Setup>(null as unknown as Setup);

    const updateSetup = <K extends keyof Setup>(key: K, value: Setup[K]) => {
        setSetup((prev) =>
            prev
                ? { ...prev, [key]: value }
                : prev
        );
    }

    const toggleSetup = async (key: keyof Setup) => {
        try {
            await invoke("toggle_setup", { key: key});

            updateSetup(key, !setup?.[key as keyof Setup])

        } catch (error) {
            console.error("Failed to toggle setup:", error);
        }
    }

    const volumeSetup = async (key: keyof Setup, value: number) => {
        try {
            await invoke("volume_setup", { key: key, value: value})

            updateSetup(key, !setup?.[key as keyof Setup])

        } catch (error) {
            console.error("Failed to toggle setup:", error);
        }
    }

    useEffect(() => {
        async function fetchSetup() {
            try {
                const fetchedSetup = await invoke<Setup>("fetch_setup");
                setSetup(fetchedSetup);
            } catch (error) {
                console.error("Failed loading setup :", error);
            }
        }

        fetchSetup().catch();
    })

    return (
        <div>
            <h1>Setup</h1>
            <button onClick={() => toggleSetup(SetupKeys.SETUP_GLOBAL_TOGGLE)}>Toggle setup</button>
            <p>Toggle : {setup?.setup_global_toggle ? "On" : "Off"}</p>
            <p>Volume</p>
            <input
                type="range"
                min={0}
                max={100}
                step={1}
                value={setup?.setup_global_volume * 100}
                onChange={(e) => volumeSetup(SetupKeys.SETUP_GLOBAL_VOLUME, parseFloat(e.target.value) / 100)
                }/>

            <h1>Keyboard</h1>
            <button onClick={() => toggleSetup(SetupKeys.SETUP_KEYBOARD_TOGGLE)}>Toggle setup</button>
            <p>Toggle : {setup?.setup_keyboard_toggle ? "On" : "Off"}</p>
            <p>Volume</p>
            <input
                type="range"
                min={0}
                max={100}
                step={1}
                value={setup?.setup_keyboard_volume * 100}
                onChange={(e) => volumeSetup(SetupKeys.SETUP_KEYBOARD_VOLUME, parseFloat(e.target.value) / 100)
                }/>

            <h1>Mouse</h1>
            <button onClick={() => toggleSetup(SetupKeys.SETUP_MOUSE_TOGGLE)}>Toggle setup</button>
            <p>Toggle : {setup?.setup_mouse_toggle ? "On" : "Off"}</p>
            <p>Volume</p>
            <input
                type="range"
                min={0}
                max={100}
                step={1}
                value={setup?.setup_mouse_volume * 100}
                onChange={(e) => volumeSetup(SetupKeys.SETUP_MOUSE_VOLUME, parseFloat(e.target.value) / 100)
                }/>

        </div>
    )

}