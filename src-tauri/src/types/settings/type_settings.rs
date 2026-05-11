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

#[derive(serde::Serialize)]

pub struct SettingDTO {
    pub setting_id: String,
    pub value: bool,
}

impl From<&Setting> for SettingDTO {
    fn from(setting: &Setting) -> Self {
        Self {
            setting_id: setting.setting_id.clone(),
            value: setting.value.load(std::sync::atomic::Ordering::Relaxed),
        }
    }
}