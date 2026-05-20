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
    pub active: Arc<AtomicBool>,
}

#[derive(serde::Serialize)]

pub struct SettingDTO {
    pub setting_id: String,
    pub active: bool,
}

impl From<&Setting> for SettingDTO {
    fn from(setting: &Setting) -> Self {
        Self {
            setting_id: setting.setting_id.clone(),
            active: setting.active.load(std::sync::atomic::Ordering::Relaxed),
        }
    }
}