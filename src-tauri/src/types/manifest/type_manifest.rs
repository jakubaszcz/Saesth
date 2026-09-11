use serde::Deserialize;

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