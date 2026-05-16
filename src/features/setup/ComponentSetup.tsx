import {useEffect, useState} from "react";
import {Setup} from "./data-setup.ts";
import {invoke} from "@tauri-apps/api/core";


export function ComponentSetup() {

    const [setup, setSetup] = useState<Setup[]>([]);

    const toggleSetup = async (setup_id: string) => {

        console.log("toggle setup : ", setup_id)
        try {
            const response = await invoke<boolean>("toggle_setup", { setupId: setup_id });


            setSetup((prev) => (
                prev.map((setup) =>
                setup.setup_id === setup_id
                    ? { ...setup, toggle: response }
                    : setup)
            ))

        } catch (error) {
            console.error("Failed to toggle setup:", error);
        }
    }

    const volumeSetup = async (setup_id: string, volume: number) => {
        try {
            const response = await invoke<number>("volume_setup", { setupId: setup_id, value: volume });

            setSetup((prev) => (
                prev.map((setup) =>
                    setup.setup_id === setup_id
                        ? { ...setup, volume: response }
                        : setup)
            ))

        } catch (error) {
            console.error("Failed to toggle setup:", error);
        }
    }

    useEffect(() => {
        async function fetchSetup() {
            try {
                const response = await invoke<Setup[]>("fetch_setup");

                setSetup(response);
            } catch (error) {
                console.error("Failed loading setup :", error);
            }
        }

        fetchSetup().catch();
    })

    return (
        <div>
            {setup.map((s) => (
                <div key={s.setup_id}>
                    <h1>{s.setup_id}</h1>

                    <button onClick={() => toggleSetup(s.setup_id)}>
                        Toggle setup here
                    </button>

                    <p>Toggle : {s.toggle ? "On" : "Off"}</p>

                    <p>Volume</p>

                    <input
                        type="range"
                        min={0}
                        max={100}
                        step={1}
                        value={s.volume * 100}
                        onChange={(e) =>
                            volumeSetup(s.setup_id, parseFloat(e.target.value) / 100)
                        }
                    />
                </div>
            ))}

        </div>
    )

}