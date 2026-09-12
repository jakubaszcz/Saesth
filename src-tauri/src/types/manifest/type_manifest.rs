use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(default)]
pub struct ManifestData {
    pub pack: String,
    pub settings: BTreeMap<String, bool>,
}

impl Default for ManifestData {
    fn default() -> Self {
        Self {
            pack: String::new(),
            settings: ["minimize_to_tray", "single_instance", "discord_rich_presence"]
                .into_iter().map(|id| (id.into(), false)).collect(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct Manifest {
    pub name: String,
    pub id: String,
    pub description: Option<String>,
    pub icon: Option<String>,
    pub sounds: Vec<ManifestSounds>,
    pub setup: ManifestSetup
}
#[derive(Debug, Deserialize)]

pub struct ManifestSounds {
    pub id: String,
    pub volume: f32,
    pub effects: Vec<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ManifestSetup {
    pub global: ManifestSetupGlobal,
    pub keyboard: ManifestSetupKeyboard,
    pub mouse: ManifestSetupMouse,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ManifestSetupGlobal {
    pub sample_rate: f32,
    pub active: bool,
    pub volume: f32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ManifestSetupKeyboard {
    pub frequency: f32,
    pub active: bool,
    pub volume: f32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ManifestSetupMouse {
    pub frequency: f32,
    pub active: bool,
    pub volume: f32,
}