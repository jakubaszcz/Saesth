use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Manifest {
    pub name: String,
    pub id: String,
    pub description: Option<String>,
    pub icon: Option<String>,
    pub sounds: Vec<ManifestSounds>,
}
#[derive(Debug, Deserialize)]

pub struct ManifestSounds {
    pub id: String,
    pub effects: Vec<String>,
}