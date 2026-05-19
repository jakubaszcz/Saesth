import {useEffect, useState} from "react";
import {Setting} from "../../structures/settings/settings.ts";
import {APIFetchSetting, APIToggleSetting} from "../../api/settings/settings.ts";

export function useSettings() {
    const [settings, setSettings] = useState<Setting[]>([]);
    const [loadingKeys, setLoadingKeys] = useState<Record<string, boolean>>({});

    useEffect(() => {
        async function loadSettings() {
            try {
                const response = await APIFetchSetting();

                setSettings(response);
            } catch (error) {
                console.error("Failed to load settings:", error);
            }
        }

        loadSettings()
    }, [])

    const toggleSetting = async (setting_id: string) => {
        setLoadingKeys((prev) => ({ ...prev, [setting_id]: true }));
        try {
            console.log("toggle setting : ", setting_id)
            const response = await APIToggleSetting(setting_id);

            console.log("response : ", response)

            setSettings((prev) =>
                prev.map((setting) =>
                    setting.setting_id === setting_id
                        ? { ...setting, toggle: response }
                        : setting
                )
            );
        } catch (error) {
            console.error("Failed to toggle setting:", error);
        } finally {
            setLoadingKeys((prev) => ({ ...prev, [setting_id]: false }));
        }
    };

    return {
        settings,
        loadingKeys,
        toggleSetting,
    }
}