import { useSettings } from "../hooks/settings/useSettings.ts";

const SETTING_METADATA: Record<string, { title: string; description: string }> = {
    "setting_minimize_to_tray": {
        title: "Minimize to Tray",
        description: "When closed, the application will continue to run in the system tray."
    },
    "setting_sync_local_database": {
        title: "Sync Local Database",
        description: "Automatically synchronize your local database with the cloud."
    },
    "setting_single_instance": {
        title: "Single Instance",
        description: "Launching multiple instances of the application will result in only one instance running."
    }
};

export function DrawSettings() {
    const {
        settings,
        toggleSetting,
    } = useSettings();

    return (
        <div className="flex flex-col gap-4 p-6">
            {settings.map((setting) => {
                const metadata = SETTING_METADATA[setting.setting_id] || {
                    title: setting.setting_id,
                    description: "No description available."
                };

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

                            <button onClick={() => toggleSetting(setting.setting_id)}>
                                {setting.active ? (
                                    <p className="
                            rounded-lg
                            border border-white/10
                            bg-white/5
                            backdrop-blur-md
                            shadow-[0_10px_40px_rgba(0,0,0,0.18)]
                            transition-all duration-300
                            hover:bg-white/[0.07]
                            hover:shadow-[0_14px_44px_rgba(0,0,0,0.24)]
                            px-4 py-2
                            text-[var(--primary-200)]
                        ">On</p>
                                ) : (
                                    <p className="
                            rounded-lg
                            border border-white/10
                            bg-white/5
                            backdrop-blur-md
                            shadow-[0_10px_40px_rgba(0,0,0,0.18)]
                            transition-all duration-300
                            hover:bg-white/[0.07]
                            hover:shadow-[0_14px_44px_rgba(0,0,0,0.24)]
                            px-4 py-2
                            text-[var(--primary-200)]
                        ">Off</p>
                                )}
                            </button>
                        </div>
                    </div>
                );
            })}
        </div>
    );
}