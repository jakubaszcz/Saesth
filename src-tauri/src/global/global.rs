use std::sync::{Mutex, OnceLock};
use rusqlite::Connection;
use crate::types::settings::type_settings::Setting;
use crate::types::setup::type_setup::Setup;
use crate::types::sounds::type_sounds::Sound;

pub static PREFIX_FOR_SOUND: &str = "sound";
pub static PREFIX_FOR_SOUND_EFFECT: &str = "sound_effect";
pub static PREFIX_FOR_SETTING: &str = "setting";

pub static PREFIXES: [&str; 3] = [PREFIX_FOR_SOUND, PREFIX_FOR_SOUND_EFFECT, PREFIX_FOR_SETTING];

pub static DATABASE: OnceLock<Mutex<Connection>> = OnceLock::new();

pub fn global_database_get() -> std::sync::MutexGuard<'static, Connection> {
    DATABASE
        .get()
        .unwrap()
        .lock()
        .unwrap()
}
pub static SOUNDS: OnceLock<Mutex<Vec<Sound>>> = OnceLock::new();
pub static SETUP: OnceLock<Mutex<Setup>> = OnceLock::new();
pub static SETTINGS: OnceLock<Mutex<Vec<Setting>>> = OnceLock::new();
