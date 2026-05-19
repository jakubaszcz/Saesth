
import {useEffect, useState} from "react";
import {Setting} from "../../structures/settings/settings.ts";
import {APIFetchSetting, APIToggleSetting} from "../../api/settings/settings.ts";

export function useSounds() {
    const [settings, setSettings] = useState<Setting[]>([]);

    useEffect(() => {
        async function loadSounds() {
            try {
                const response = await APIFetchSetting();

                setSettings(response);
            } catch (error) {
                console.error("Failed to load sounds:", error);
            }
        }

        loadSounds()
    })

    const toggleSetting = async (setting_id: string) => {
        try {
            const response = await APIToggleSetting(setting_id);

            setSettings((prev) =>
                prev.map((setting) =>
                    setting.setting_id === setting_id
                        ? { ...setting, toggle: response }
                        : setting
                )
            );
        } catch (error) {
            console.error("Failed to toggle setting:", error);
        }
    };

    return {
        settings,
        toggleSetting,
    }
}