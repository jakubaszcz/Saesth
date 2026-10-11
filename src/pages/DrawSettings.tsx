import { openUrl } from "@tauri-apps/plugin-opener";
import { useSettings } from "../hooks/settings/useSettings.ts";

const isWindows = typeof navigator !== "undefined" && /Windows NT/i.test(navigator.userAgent);

const SETTING_METADATA: Record<string, { title: string; description: string }> = {
    "minimize_to_tray": {
        title: "Minimize to Tray",
        description: "When closed, the application will continue to run in the system tray."
    },
    "single_instance": {
        title: "Single Instance",
        description: "Launching multiple instances of the application will result in only one instance running."
    },
    "discord_rich_presence": {
        title: "Discord Rich Presence",
        description: "When enabled, the application will display on your Discord."
    },
    "wist_widget": {
        title: "Wist Widget",
        description: "Display the Saesth widget in Wist. Disabling this removes the widget."
    }
};

export function DrawSettings() {
    const {
        settings,
        toggleSetting,
    } = useSettings();

    return (
        <div className="page-shell">
            <header>
                <p className="page-eyebrow">The little details</p>
                <h1 className="page-title">Make Saesth yours.</h1>
                <p className="page-description">A few preferences to help everything feel at home.</p>
            </header>
            <div className="flex max-w-3xl flex-col gap-4">
            {settings.map((setting) => {
                if (setting.setting_id === "wist_widget" && !isWindows) return null;

                const metadata = SETTING_METADATA[setting.setting_id] || {
                    title: setting.setting_id,
                    description: "No description available."
                };

                return (
                    <div className="quiet-panel flex items-center justify-between gap-6"
                        key={setting.setting_id}>
                        <div className="flex flex-col gap-1 w-3/4">
                            <p className="text-primary-100 text-lg font-medium font-secondary">
                                {metadata.title}
                            </p>

                            <p className="text-primary-200 text-sm leading-6 font-primary">
                                {metadata.description}
                            </p>
                            {setting.setting_id === "wist_widget" && (
                                <aside aria-labelledby="wist-notice-title" className="mt-3 rounded-2xl border border-warning-400/30 bg-warning-950/20 p-4 font-primary text-sm leading-6">
                                    <h2 id="wist-notice-title" className="font-medium text-warning-200">
                                        Wist required
                                    </h2>
                                    <p className="mt-1 text-warning-100">
                                        The Wist app must be installed and running to enable this widget.
                                    </p>
                                    <button
                                        type="button"
                                        onClick={() => {
                                            void openUrl("https://github.com/jakubaszcz/Widget/releases/tag/v0.1.0")
                                                .catch(error => console.error("Unable to open the Wist download page:", error));
                                        }}
                                        className="mt-3 inline-flex items-center justify-center rounded-xl border border-warning-400/30 bg-warning-400/10 px-4 py-2 font-medium text-warning-200 transition-colors hover:bg-warning-400/20 hover:text-warning-100"
                                    >
                                        Download
                                    </button>
                                </aside>
                            )}
                        </div>

                        <button
                            type="button"
                            role="switch"
                            aria-checked={setting.active}
                            aria-label={metadata.title}
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
        </div>
    );
}
