use std::sync::Arc;
use std::sync::atomic::AtomicBool;

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SettingKeys {
    MinimizeToTray,
    SyncLocalDatabase,
    SingleInstance,
    DiscordRichPresence,
}

impl SettingKeys {
    pub fn to_key(&self) -> String {
        match self {
            SettingKeys::MinimizeToTray => "minimize_to_tray".to_string(),
            SettingKeys::SyncLocalDatabase => "sync_local_database".to_string(),
            SettingKeys::SingleInstance => "single_instance".to_string(),
            SettingKeys::DiscordRichPresence => "discord_rich_presence".to_string(),
        }
    }
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