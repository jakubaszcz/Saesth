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
                    <div className="p-5 rounded-lg bg-primary-800 hover:bg-primary-700 duration-300 transition-all flex items-center justify-between"
                        key={setting.setting_id}>
                        <div className="flex flex-col gap-1 w-3/4">
                            <p className="text-primary-200 font-semibold font-secondary">
                                {metadata.title}
                            </p>

                            <p className="text-primary-300 text-sm font-primary">
                                {metadata.description}
                            </p>
                        </div>

                        <button
                            onClick={() => toggleSetting(setting.setting_id)}
                            className={`cursor-pointer relative inline-flex h-6 w-11 items-center rounded-full transition-colors duration-300 ${
                                setting.active ? "bg-primary-500" : "bg-primary-900"
                            }`}
                        >
                            <span
                                className={`inline-block h-4 w-4 transform rounded-full bg-primary-100 transition-transform duration-300 ${
                                    setting.active ? "translate-x-6" : "translate-x-1"
                                }`}
                            />
                        </button>
                    </div>
                );
            })}
        </div>
    );
}