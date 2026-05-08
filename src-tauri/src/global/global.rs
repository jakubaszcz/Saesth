use std::sync::{Mutex, OnceLock};
use crate::types::settings::type_settings::Setting;
use crate::types::sounds::type_sounds::Sound;

pub static PREFIX_FOR_SOUND: &str = "sound";
pub static PREFIX_FOR_SOUND_EFFECT: &str = "sound_effect";
pub static PREFIX_FOR_SETTING: &str = "setting";

pub static SOUNDS: OnceLock<Mutex<Vec<Sound>>> = OnceLock::new();
pub static SETTINGS: OnceLock<Mutex<Vec<Setting>>> = OnceLock::new();