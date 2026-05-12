use serde::Deserialize;
use crate::database::settings::database_settings::{database_settings_get_active_setting};
use crate::database::setup::database_setup::{database_create_setup_if_missing, database_sync_setup};
use crate::global::global::{PREFIX_FOR_SETTING, PREFIX_FOR_SETUP, PREFIX_FOR_SOUND, PREFIX_FOR_SOUND_EFFECT};
use crate::types::settings::type_settings::SettingKeys;

#[derive(Deserialize)]
struct Config {
    id: String,
}
const RESOURCES: &str = include_str!("../../ressources/setup.json");

fn sync_tables() {
    let config: Vec<Config> = serde_json::from_str(RESOURCES).unwrap();

    let excepted_setup: Vec<String> = config
        .iter()
        .map(|s| format!("{}_{}", PREFIX_FOR_SETUP, s.id))
        .collect();

    let expected_sounds_ref: Vec<&str> = excepted_setup.iter().map(|s| s.as_str()).collect();

    database_sync_setup(&expected_sounds_ref);
}
pub fn init_tables() {
    let config: Vec<Config> = serde_json::from_str(RESOURCES).unwrap();

    for setup in config {
        let setup_id = format!("{}_{}", PREFIX_FOR_SOUND, setup.id);

        database_create_setup_if_missing(&setup_id);
    }

    // Delete unused sounds & effects data
    if database_settings_get_active_setting(format!("{}_{:?}", PREFIX_FOR_SETTING, SettingKeys::SyncLocalDatabase).as_str()) {
        sync_tables();
    }
}