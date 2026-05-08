use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use serde::Deserialize;
use crate::database::database::database_get_setting_active;
use crate::global::global::PREFIX_FOR_SETTING;
use crate::inits::settings::init_tables_settings::init_tables;
use crate::types::settings::type_settings::Setting;

#[derive(Deserialize)]
struct Config {
    id: String,
}

const RESOURCES: &str = include_str!("../../ressources/settings.json");

fn make_setting(id: &str) -> Setting {
    let setting_id = format!("{}_{}", PREFIX_FOR_SETTING, id);

    Setting {
        setting_id: setting_id.clone(),
        value: Arc::new(AtomicBool::new(database_get_setting_active(setting_id.as_str()))),
    }
}
fn init() -> Vec<Setting> {

    {
        init_tables()
    }

    let config: Vec<Config> = serde_json::from_str(RESOURCES).unwrap();

    config
        .iter()
        .map(|setting| make_setting(&setting.id))
        .collect()
}