use std::sync::Arc;
use std::sync::atomic::AtomicBool;

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SettingKeys {
    MinimizeToTray,
    SyncLocalDatabase,
}

pub struct Setting {
    pub setting_id: String,
    pub value: Arc<AtomicBool>,
}