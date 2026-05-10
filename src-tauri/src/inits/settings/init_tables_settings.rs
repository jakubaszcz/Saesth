use serde::Deserialize;
use crate::database::settings::database_settings::{database_create_setting_if_missing, database_get_setting_active, database_sync_setting};
use crate::global::global::{PREFIX_FOR_SETTING, PREFIX_FOR_SOUND, PREFIX_FOR_SOUND_EFFECT};
use crate::types::settings::type_settings::SettingKeys;

#[derive(Deserialize)]
struct Config {
    id: String,
}
const RESOURCES: &str = include_str!("../../ressources/settings.json");

fn sync_tables() {
    let config: Vec<Config> = serde_json::from_str(RESOURCES).unwrap();

    let expected_sounds: Vec<String> = config
        .iter()
        .map(|s| format!("{}_{}", PREFIX_FOR_SOUND, s.id))
        .collect();


    let expected_sounds_ref: Vec<&str> = expected_sounds.iter().map(|s| s.as_str()).collect();

    database_sync_setting(&expected_sounds_ref);
}
pub fn init_tables() {
    let config: Vec<Config> = serde_json::from_str(RESOURCES).unwrap();

    for setting in config {
        let sound_id = format!("{}_{}", PREFIX_FOR_SETTING, setting.id);

        database_create_setting_if_missing(&sound_id);
    }

    // Delete unused sounds & effects data
    if database_get_setting_active(format!("{}_{:?}", PREFIX_FOR_SETTING, SettingKeys::SyncLocalDatabase).as_str()) {
        sync_tables();
    }
}