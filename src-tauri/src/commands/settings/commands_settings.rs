use std::sync::atomic::Ordering;
use crate::inits::manifest::init_manifest::update;
use crate::global::global::{SETTINGS};
use crate::types::settings::type_settings::{SettingDTO, SettingKeys};

pub fn commands_settings_fetch_settings() -> Vec<SettingDTO> {
    let settings = SETTINGS.get().unwrap().lock().unwrap();

    settings.iter()
        .map(|setting| SettingDTO::from(setting))
        .collect()
}

pub fn commands_settings_toggle_setting(setting_id: String) -> Result<bool, String> {
    let mut settings = SETTINGS.get().unwrap().lock().unwrap();

    settings.iter_mut()
        .find(|s| s.setting_id == setting_id)
        .map(|s| {
            let new_val = !s.active.load(Ordering::Relaxed);



            {
                update(|manifest| { manifest.settings.insert(setting_id.clone(), new_val); })?;
                s.active.store(new_val, Ordering::Relaxed);
            }

            Ok(new_val)
        })
        .unwrap_or_else(|| Err("Unknown setting".into()))
}