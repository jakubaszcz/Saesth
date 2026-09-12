use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use serde::Deserialize;
use crate::inits::manifest::init_manifest::setting_active;
use crate::types::settings::type_settings::Setting;

#[derive(Deserialize)]
struct Config {
    id: String,
}

const RESOURCES: &str = include_str!("../../ressources/settings.json");

fn make_setting(id: &str) -> Setting {
    let setting_id = id.to_string();

    Setting {
        setting_id: setting_id.clone(),
        active: Arc::new(AtomicBool::new(setting_active(&setting_id.clone()))),
    }
}
pub fn init() -> Vec<Setting> {


    let config: Vec<Config> = serde_json::from_str(RESOURCES).unwrap();

    config
        .iter()
        .map(|setting| make_setting(&setting.id))
        .collect()
}