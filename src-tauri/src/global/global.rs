use std::sync::{Mutex, OnceLock};
use crate::types::appdata::type_appdata::Appdata;
use crate::types::packs::type_packs::{Pack, SelectedPack};
use crate::types::settings::type_settings::Setting;
use crate::types::setup::type_setup::Setup;
use crate::types::sounds::type_sounds::Sound;

pub static MANIFEST: OnceLock<Mutex<crate::types::manifest::type_manifest::ManifestData>> = OnceLock::new();

pub static SOUNDS: OnceLock<Mutex<Vec<Sound>>> = OnceLock::new();
pub static SETUP: OnceLock<Mutex<Vec<Setup>>> = OnceLock::new();
pub static SETTINGS: OnceLock<Mutex<Vec<Setting>>> = OnceLock::new();
pub static PACKS: OnceLock<Mutex<Vec<Pack>>> = OnceLock::new();
pub static PACK: OnceLock<Mutex<SelectedPack>> = OnceLock::new();

pub static PATHS: OnceLock<Appdata> = OnceLock::new();
