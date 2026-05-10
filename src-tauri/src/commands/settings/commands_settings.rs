use crate::database::settings::database_settings::database_settings_set_active_setting;
use crate::global::global::{PREFIX_FOR_SETTING, SETTINGS};
use crate::types::settings::type_settings::{SettingDTO, SettingKeys};

pub fn commands_settings_fetch_settings() -> Vec<SettingDTO> {
    let settings = SETTINGS.get().unwrap().lock().unwrap();

    settings.iter()
        .map(|setting| SettingDTO::from(setting))
        .collect()
}

pub fn commands_settings_toggle_setting(setting_id: SettingKeys, value: bool) -> bool {
    let mut settings = SETTINGS.get().unwrap().lock().unwrap();

    let setting_id = format!("{}_{:?}", PREFIX_FOR_SETTING, setting_id);

    settings.iter_mut()
        .find(|s| s.setting_id == setting_id)
        .map(|s| {
            s.value.store(value, std::sync::atomic::Ordering::Relaxed);

            {
                database_settings_set_active_setting(&setting_id, value);
            }

            value
        })
        .unwrap_or(false)
}