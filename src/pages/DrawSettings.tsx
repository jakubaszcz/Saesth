import { invoke } from "@tauri-apps/api/core";
import { useEffect, useState } from "react";
import { Settings } from "../interface/settings.ts";

export function DrawSettings() {
    const [values, setValues] = useState<Record<string, string>>({});
    const [loading, setLoading] = useState<Record<string, boolean>>({});

    const setSettings = async (id: string, value: string) => {
        await invoke("set_settings", { id, value });
    };

    const toggleSetting = async (id: string) => {
        const current = values[id] ?? "false";
        const next = current === "true" ? "false" : "true";

        setValues((prev) => ({
            ...prev,
            [id]: next,
        }));

        setLoading((prev) => ({
            ...prev,
            [id]: true,
        }));

        try {
            await setSettings(id, next);
        } catch (error) {
            setValues((prev) => ({
                ...prev,
                [id]: current,
            }));
            console.error(`Failed to update setting "${id}"`, error);
        } finally {
            setLoading((prev) => ({
                ...prev,
                [id]: false,
            }));
        }
    };

    useEffect(() => {
        const load = async () => {
            const result: Record<string, string> = {};

            for (const setting of Settings) {
                try {
                    result[setting.key] = await invoke<string>("get_settings", {
                        id: setting.key,
                    });
                } catch (error) {
                    console.error(`Failed to load setting "${setting.key}"`, error);
                    result[setting.key] = "false";
                }
            }

            setValues(result);
        };

        load();
    }, []);

    return (
        <div className="flex flex-col gap-4">
            {Settings.map((setting) => {
                const isEnabled = values[setting.key] === "true";
                const isBusy = loading[setting.key] === true;

                return (
                    <div
                        key={setting.key}
                        className="
                            rounded-lg
                            border border-white/10
                            bg-white/5
                            backdrop-blur-md
                            shadow-[0_10px_40px_rgba(0,0,0,0.18)]
                            transition-all duration-300
                            hover:bg-white/[0.07]
                            hover:shadow-[0_14px_44px_rgba(0,0,0,0.24)]
                            p-5
                        "
                    >
                        <div className="flex items-center justify-between gap-4">
                            <div className="min-w-0">
                                <p className="font-inter text-base font-semibold text-[var(--primary-200)]">
                                    {setting.title}
                                </p>

                                <p className="mt-1 text-sm leading-relaxed text-[var(--primary-100)]">
                                    {setting.description}
                                </p>
                            </div>

                            <button
                                type="button"
                                role="switch"
                                aria-checked={isEnabled}
                                aria-label={`Toggle ${setting.title}`}
                                disabled={isBusy}
                                onClick={() => toggleSetting(setting.key)}
                                className={`
                                    relative h-8 w-14 shrink-0 rounded-full
                                    transition-all duration-300
                                    focus:outline-none focus:ring-2 focus:ring-white/20
                                    ${isEnabled
                                    ? "bg-[var(--primary-300)]/80 shadow-[0_0_18px_rgba(255,255,255,0.08)]"
                                    : "bg-white/10"}
                                    ${isBusy ? "opacity-60 cursor-wait" : "cursor-pointer"}
                                `}
                            >
                                <span
                                    className={`
                                        absolute top-1 h-6 w-6 rounded-full
                                        bg-[var(--primary-100)] shadow-md
                                        transition-all duration-300
                                        ${isEnabled ? "left-7" : "left-1"}
                                    `}
                                />
                            </button>
                        </div>
                    </div>
                );
            })}
        </div>
    );
}