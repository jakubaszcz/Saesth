import { useSettings } from "../hooks/settings/useSettings.ts";

const SETTING_METADATA: Record<string, { title: string; description: string }> = {
    "setting_minimize_to_tray": {
        title: "Minimize to Tray",
        description: "When closed, the application will continue to run in the system tray."
    },
    "setting_sync_local_database": {
        title: "Sync Local Database",
        description: "Automatically synchronize your local database with the cloud."
    }
};

export function DrawSettings() {
    const {
        settings,
        loadingKeys,
        toggleSetting,
    } = useSettings();

    return (
        <div className="flex flex-col gap-4">
            {settings.map((setting) => {
                const metadata = SETTING_METADATA[setting.setting_id] || {
                    title: setting.setting_id,
                    description: "No description available."
                };
                const isEnabled = setting.toggle;
                const isBusy = loadingKeys[setting.setting_id] === true;

                return (
                    <div
                        key={setting.setting_id}
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
                                    {metadata.title}
                                </p>

                                <p className="mt-1 text-sm leading-relaxed text-[var(--primary-100)]">
                                    {metadata.description}
                                </p>
                            </div>

                            <button
                                type="button"
                                role="switch"
                                aria-checked={isEnabled}
                                aria-label={`Toggle ${metadata.title}`}
                                disabled={isBusy}
                                onClick={() => toggleSetting(setting.setting_id)}
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