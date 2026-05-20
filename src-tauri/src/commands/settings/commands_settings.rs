use std::sync::atomic::Ordering;
use crate::database::settings::database_settings::database_settings_set_active_setting;
use crate::global::global::{PREFIX_FOR_SETTING, SETTINGS};
use crate::types::settings::type_settings::{SettingDTO, SettingKeys};

pub fn commands_settings_fetch_settings() -> Vec<SettingDTO> {
    let settings = SETTINGS.get().unwrap().lock().unwrap();

    settings.iter()
        .map(|setting| SettingDTO::from(setting))
        .collect()
}

pub fn commands_settings_toggle_setting(setting_id: String) -> bool {
    let mut settings = SETTINGS.get().unwrap().lock().unwrap();

    settings.iter_mut()
        .find(|s| s.setting_id == setting_id)
        .map(|s| {
            let new_val = !s.active.load(Ordering::Relaxed);

            s.active.store(new_val, Ordering::Relaxed);

            {
                database_settings_set_active_setting(&setting_id, new_val);
            }

            new_val
        })
        .unwrap_or(false)
}