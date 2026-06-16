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
    },
    "setting_discord_rich_presence": {
        title: "Discord Rich Presence",
        description: "When enabled, the application will display on your Discord."
    }
};

export function DrawSettings() {
    const {
        settings,
        toggleSetting,
    } = useSettings();

    return (
        <div className="flex flex-col gap-2">
            {settings.map((setting) => {
                const metadata = SETTING_METADATA[setting.setting_id] || {
                    title: setting.setting_id,
                    description: "No description available."
                };

                return (
                    <div className="p-5 rounded-lg bg-primary-800 hover:bg-primary-700 duration-300 transition-all"
                        key={setting.setting_id}>
                        <div>
                            <div className="flex flex-col gap-1">
                                <p className="text-primary-200 font-semibold font-secondary">
                                    {metadata.title}
                                </p>

                                <p className="text-primary-300 text-sm font-primary">
                                    {metadata.description}
                                </p>
                            </div>

                            <button onClick={() => toggleSetting(setting.setting_id)}>
                                {setting.active ? (
                                    <p>On</p>
                                ) : (
                                    <p>Off</p>
                                )}
                            </button>
                        </div>
                    </div>
                );
            })}
        </div>
    );
}