import {useEffect, useState} from "react";
import {Setup} from "./data-setup.ts";
import {invoke} from "@tauri-apps/api/core";

const SETUP_METADATA: Record<string, { title: string; description: string }> = {
    "setup_global": {
        title: "Global",
        description: "Toggle the global setup."
    },
    "setup_keyboard": {
        title: "Keyboard",
        description: "Toggle the keyboard setup."
    },
    "setup_mouse": {
        title: "Mouse",
        description: "Toggle the mouse setup."
    },
};

export function ComponentSetup() {

    const [setup, setSetup] = useState<Setup[]>([]);

    const toggleSetup = async (setup_id: string) => {
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
            {setup.map((s) => {

                const metadata = SETUP_METADATA[s.setup_id] || {
                    title: s.setup_id,
                    description: "No description available."
                };

                return (
                    <div key={s.setup_id} className="p-5 rounded-lg bg-primary-800 hover:bg-primary-700 duration-300 transition-all">
                        <div className="flex items-center justify-between">
                            <div className="flex flex-col gap-1 w-3/4">
                                <p className="text-primary-200 font-semibold font-secondary">
                                    {metadata.title}
                                </p>

                                <p className="text-primary-300 text-sm font-primary">
                                    {metadata.description}
                                </p>
                            </div>

                            <button
                                onClick={() => toggleSetup(s.setup_id)}
                                className={`cursor-pointer relative inline-flex h-6 w-11 items-center rounded-full transition-colors duration-300 ${
                                    s.toggle ? "bg-primary-500" : "bg-primary-900"
                                }`}
                            >
                            <span
                                className={`inline-block h-4 w-4 transform rounded-full bg-primary-100 transition-transform duration-300 ${
                                    s.toggle ? "translate-x-6" : "translate-x-1"
                                }`}
                            />
                            </button>
                        </div>

                        <div className="grow mt-2">
                            <div className="flex justify-between items-center">
                        <span className="text-primary-300 text-sm font-medium font-primary">
                            Volume
                        </span>

                                <div className="flex items-center gap-1 border bg-primary-800 border-primary-700 rounded-md px-2 py-1">
                                    <input
                                        type="number"
                                        min={0}
                                        max={100}
                                        value={Math.round(s.volume * 100)}
                                        onChange={(e) => {
                                            const value = Math.max(
                                                0,
                                                Math.min(100, Number(e.target.value))
                                            );

                                            volumeSetup(s.setup_id, value / 100);
                                        }}
                                        className="
            w-12
            bg-transparent
            text-primary-200
            text-sm
            font-semibold
            text-right
            outline-none
        "
                                    />

                                    <span className="text-primary-200 text-sm font-semibold">
                                %
                            </span>
                                </div>
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
                );
            })
            }

        </div>
    )

}