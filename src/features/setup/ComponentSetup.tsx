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
        <div className="flex flex-col gap-2">
            {setup.map((s) => (
                <div key={s.setup_id} className="p-5 rounded-lg bg-primary-800 hover:bg-primary-700 duration-300 transition-all">
                    <h1>{s.setup_id}</h1>

                    <button onClick={() => toggleSetup(s.setup_id)}>
                        Toggle setup here
                    </button>

                    <p>Toggle : {s.toggle ? "On" : "Off"}</p>

                    <div className="grow">
                        <div className="flex justify-between items-center">
                        <span className="text-primary-300 text-sm font-medium">
                            Volume
                        </span>

                            <span className="text-primary-200 text-sm font-semibold">
                            {Math.round(s.volume * 100)}%
                        </span>
                        </div>

                        <input
                            className="
                            w-full
                            h-1.5
                            bg-primary-900
                            rounded-lg
                            appearance-none
                            cursor-pointer
                            accent-primary-500
                            hover:accent-primary-400
                            transition-all
                            [&::-webkit-slider-thumb]:appearance-none
                            [&::-webkit-slider-thumb]:w-3
                            [&::-webkit-slider-thumb]:h-3
                            [&::-webkit-slider-thumb]:rounded-full
                            [&::-webkit-slider-thumb]:bg-primary-100
                            [&::-webkit-slider-thumb]:shadow-[0_0_10px_rgba(var(--color-primary-500),0.5)]
                            [&::-moz-range-thumb]:border-none
                            [&::-moz-range-thumb]:w-3
                            [&::-moz-range-thumb]:h-3
                            [&::-moz-range-thumb]:rounded-full
                            [&::-moz-range-thumb]:bg-primary-100
                        "
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
                </div>
            ))}

        </div>
    )

}